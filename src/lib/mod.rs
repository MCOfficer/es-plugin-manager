extern crate app_dirs;
extern crate clap;
extern crate dirs;
extern crate reqwest;
extern crate runas;
extern crate symlink;
extern crate yaml_rust;

use app_dirs::*;
use dirs::data_dir;
use runas::Command;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::path::PathBuf;
use symlink::{remove_symlink_dir, symlink_dir};
use yaml_rust::{Yaml, YamlLoader};

mod git;

const INDEX_URL: &str =
    "https://raw.githubusercontent.com/MCOfficer/es-plugin-manager/master/plugins.yml";
const APP_INFO: AppInfo = AppInfo {
    name: "ESPIM",
    author: "MCOfficer",
};

fn get_es_plugin_dir(verbose: bool) -> PathBuf {
    let dir = data_dir()
        .expect("dirs failed to find data_dir")
        .join("endless-sky")
        .join("plugins");
    if verbose {
        println!(
            "app_dirs returned {} as ES plugin directory",
            dir.to_string_lossy()
        );
    }
    dir
}

fn get_espim_plugin_dir(verbose: bool) -> PathBuf {
    let dir = get_app_dir(AppDataType::UserCache, &APP_INFO, "plugins")
        .expect("app_dirs failed with an error");
    if verbose {
        println!(
            "app_dirs returned {} as repo directory",
            dir.to_string_lossy()
        );
    }
    dir
}

fn get_install_path(name: &str, verbose: bool) -> PathBuf {
    get_es_plugin_dir(verbose).join("[ESPIM] ".to_string() + name)
}

fn get_repo_path(name: &str, verbose: bool) -> PathBuf {
    let path = get_espim_plugin_dir(verbose).join(name);
    if !path.exists() {
        create_dir_all(&path.parent().unwrap()).expect("Failed to create ESPIM directory");
    }
    path
}

fn is_installed(name: &str, verbose: bool) -> bool {
    get_install_path(name, verbose).exists() && get_repo_path(name, verbose).exists()
}

fn fetch_url_contents(url: &str, verbose: bool) -> Result<String, reqwest::Error> {
    if verbose {
        println!("Attempting to fetch content of {}", url);
    }
    reqwest::get(INDEX_URL)?.text()
}

fn get_index_path(verbose: bool) -> PathBuf {
    let repo_dir = get_app_dir(AppDataType::UserCache, &APP_INFO, "plugins.yml")
        .expect("app_dirs failed with an error");
    if verbose {
        println!(
            "app_dirs returned {} as index file path",
            repo_dir.to_string_lossy()
        );
    }
    repo_dir
}

fn update_index(verbose: bool) {
    println!("Getting latest index");
    let index_path = get_index_path(verbose);
    match fetch_url_contents(INDEX_URL, verbose) {
        Err(e) => {
            println!("Error fetching index: {}", e);
        }
        Ok(content) => {
            if verbose {
                println!("Writing to {}", index_path.to_string_lossy());
            }
            let mut file = File::create(index_path.as_path()).expect("Failed to open index file");
            file.write_all(content.as_bytes())
                .expect("Failed to write to index file");
        }
    }
}

fn get_index(verbose: bool) -> Yaml {
    let index_path = get_index_path(verbose);
    update_index(verbose);
    if verbose {
        println!("Reading index from {}", index_path.to_string_lossy());
    }
    let mut contents = String::new();
    File::open(index_path.as_path())
        .expect("Failed to open index file")
        .read_to_string(&mut contents)
        .expect("Failed to read from index file");
    YamlLoader::load_from_str(contents.as_str()).expect("Failed to parse index file as YAML")[0]
        .clone()
}

fn fix_metadata_dir(dir: &Path) {
    for entry in std::fs::read_dir(dir).unwrap() {
        let buf = entry.unwrap().path();
        let e = buf.as_path();
        if e.is_dir() {
            fix_metadata_dir(e);
        }
        let mut perms = std::fs::metadata(e)
            .expect("Failed to get metadata")
            .permissions();
        perms.set_readonly(false);
        std::fs::set_permissions(e, perms).expect("Failed to set repo permissions");
    }
}

pub fn update(verbose: bool) {
    update_index(verbose);
    println!("Done");
}

pub fn upgrade(verbose: bool) {
    let index = get_index(verbose);
    for plugin in index.as_vec().expect("Index is not an array") {
        let name = plugin["name"].as_str().unwrap();
        let version = plugin["version"].as_str().unwrap();
        if is_installed(name, verbose) {
            println!("\n{} -> {}", name, version);
            git::checkout_repo_at(&get_repo_path(name, verbose), version, verbose);
        }
    }
    println!("\nDone.")
}

pub fn list(verbose: bool) {
    let index = get_index(verbose);
    println!("{: ^11}|{: ^40}|{:^45}", "Installed", "Name", "Version");
    println!("{:-<11}|{:-<40}|{:-<45}", "", "", "");
    for plugin in index.as_vec().expect("Index is not an array") {
        let name = plugin["name"].as_str().unwrap();
        let installed = if is_installed(name, false) {
            "Yes"
        } else {
            "No"
        };
        println!(
            "{: ^11}|  {: <38}|  {: <43}",
            installed,
            name,
            plugin["version"].as_str().unwrap()
        );
    }
}

pub fn install(identifier: &str, verbose: bool) {
    let index = get_index(verbose);
    let plugins = index.as_vec().expect("Index is not an array");
    let plugin = plugins
        .iter()
        .find(|&x| x["name"].as_str().unwrap() == identifier)
        .unwrap_or_else(|| {
            plugins
                .iter()
                .find(|&x| x["name"].as_str().unwrap().to_lowercase() == identifier.to_lowercase())
                .expect("Plug-In not found")
        });
    let name = plugin["name"].as_str().unwrap();
    let url = plugin["url"].as_str().unwrap();

    let repo_path = get_repo_path(name, verbose);
    let link_path = get_install_path(name, verbose);
    println!(
        "Attempting to install '{}' as '{}'",
        name,
        link_path.file_name().unwrap().to_string_lossy()
    );
    if is_installed(name, verbose) {
        println!("{} is already installed? Aborting", name);
        return;
    }

    if !repo_path.exists() {
        git::clone(url, &repo_path);
    }
    git::checkout_repo_at(&repo_path, plugin["version"].as_str().unwrap(), verbose);

    if verbose {
        println!(
            "Linking '{}' to '{}'",
            repo_path.to_string_lossy(),
            link_path.to_string_lossy()
        );
    }
    if cfg!(windowse) {
        if verbose {
            println!("Using Windows workaround");
        }
        let status = Command::new("cmd")
            .args(&[
                "/C",
                "mklink",
                "/D",
                link_path.to_str().unwrap(),
                repo_path.to_str().unwrap(),
            ])
            .status()
            .expect("Failed to create Symlink");
        if verbose {
            println!("mklink status: '{}'", status.to_string())
        }
        if !status.success() {
            panic!("mklink returned non-zero exit status - Failed to create Symlink")
        }
    } else {
        symlink_dir(repo_path, link_path).expect("Failed to create Symlink");
    }

    println!("Done.")
}

pub fn remove(name: &str, verbose: bool) {
    if !is_installed(name, verbose) {
        println!("{} is not installed? Aborting", name);
        return;
    }
    let link = get_install_path(name, verbose);
    if verbose {
        println!("Removing Symlink '{}'", link.to_string_lossy());
    }

    remove_symlink_dir(link).expect("Failed to remove Symlink");
    println!("Done.");
}

pub fn purge(name: &str, verbose: bool) {
    let repo_path = get_repo_path(name, verbose);
    let link = get_install_path(name, verbose);

    if !repo_path.exists() {
        println!("{} is not installed? Aborting", name);
        return;
    }

    if link.exists() {
        if verbose {
            println!("Removing Symlink '{}'", link.to_string_lossy());
        }
        remove_symlink_dir(link).expect("Failed to remove Symlink");
    }

    println!("The directory {} and all its contents will be removed. Enter 'y' to proceed, anything else to abort", repo_path.to_string_lossy()); //careful now
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read from stdin");
    if buffer.trim().to_lowercase() == "y" {
        if cfg!(windows) {
            println!("Using Windows workaround");
            fix_metadata_dir(repo_path.as_path());
        }
        std::fs::remove_dir_all(&repo_path).expect("Failed to remove directory");
    }

    println!("Done.");
}
