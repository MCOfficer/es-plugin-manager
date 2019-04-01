extern crate app_dirs;
extern crate clap;
extern crate git2;
extern crate remove_dir_all;
extern crate symlink;
extern crate dirs;
extern crate runas;

use std::path::PathBuf;

use app_dirs::*;
use clap::{App, Arg, SubCommand};
use git2::Repository;
use remove_dir_all::remove_dir_all;
use symlink::symlink_dir;
use dirs::data_dir;
use runas::Command;

const VERSION: &str = "0.1.0";
const REPO_URL: &str = "https://github.com/MCOfficer/endless-sky-plugins.git";
const APP_INFO: AppInfo = AppInfo{name: "ESPIM", author: "MCOfficer"};

fn main() {
    let matches = App::new("Endless Sky Plug-In Manager")
        .version(VERSION)
        .about("A Proof-of-Concept Plug-In Manager for Endless Sky.")
        .subcommand(SubCommand::with_name("init")
            .about("Clones the plug-In repository. Use before anything else.")
        )
        .subcommand(SubCommand::with_name("update")
            .about("Updates the local plug-In repository.")
        )
        .subcommand(SubCommand::with_name("list")
            .about("Lists all available plug-ins.")
        )
        .subcommand(SubCommand::with_name("install")
            .about("Installs a plug-in.")
            .arg(Arg::with_name("PLUGIN")
                .help("The plug-in to install.")
                .required(true)
                .index(1)
            )
        )
        .arg(Arg::with_name("verbose")
            .long("verbose")
            .short("v")
            .help("Prints moar info.")
        ).get_matches();

    let verbose = matches.is_present("verbose");
    if verbose {
        println!("Verbose Output enabled.");
        println!("ESPIM v{}", VERSION);
    }

    if let Some(_matches) = matches.subcommand_matches("init") {
        init(verbose);
    }
    else if let Some(_matches) = matches.subcommand_matches("update") {
        update(verbose);
    }
    else if let Some(_matches) = matches.subcommand_matches("list") {
        list(verbose);
    }
    else if let Some(matches) = matches.subcommand_matches("install") {
        install(matches.value_of("PLUGIN").unwrap(), verbose);
    }
    else {
        println!("See espim -h for help");
    }
}

fn get_plugin_dir() -> PathBuf {
    data_dir().expect("dirs failed to find data_dir")
        .join("endless-sky").join("plugins")
}

fn get_install_path(name: &str) -> PathBuf {
    get_plugin_dir().join("[ESPIM] ".to_string() + name)
}

fn is_installed(name: &str) -> bool {
    get_install_path(name).exists()
}

fn get_repo_dir(verbose: bool) -> PathBuf {
    let repo_dir = match get_app_dir(AppDataType::UserCache, &APP_INFO, "repo") {
        Ok(repo_dir) => repo_dir,
        Err(e) => panic!("app_dirs failed with an error: {}", e),
    };
    if verbose {
        println!("app_dirs returned {} as repo directory", repo_dir.to_string_lossy());
    }
    repo_dir
}

fn open_repo(verbose: bool) -> Repository {
    let repo_dir = get_repo_dir(verbose);
    if verbose {
        ("Opening Repository {}", repo_dir.to_string_lossy());
    };
    match Repository::open(repo_dir.as_path()) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to open Repository: {}", e),
    }
}

fn init(verbose: bool) {
    let repo_dir = get_repo_dir(verbose);
    if repo_dir.exists() {
        println!("Repo Directory already exists. Did you run this command before?");
        return;
    }
    if verbose {
        println!("Cloning {} into {}", REPO_URL, repo_dir.to_string_lossy());
    }

    match Repository::clone(REPO_URL, repo_dir) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to Clone Repository: {}", e),
    };

    println!("Done.")
}

fn update(verbose: bool) {
    let repo_dir = get_repo_dir(verbose);
    if !repo_dir.exists() && verbose {
        println!("Repo directory does not exist. Did you run 'espim init'?");
        return;
    }
    if verbose {
        println!("This actually removes the current repo and re-clones it. Sue me.");
        println!("Removing directory {}", repo_dir.to_string_lossy())
    }

    match remove_dir_all(repo_dir) {
        Ok(o) => o,
        Err(e) => panic!("Failed to remove the repo directory: {}", e)
    };
    init(verbose);
}

fn list(verbose: bool) {
    let repo = open_repo(verbose);
    let submodules = match repo.submodules() {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to load Submodules: {}", e),
    };
    println!("{: ^11}|{: ^40}|{:^45}", "Installed", "Name", "Version");
    println!("{:-<11}|{:-<40}|{:-<45}", "", "", "");
    for submodule in &submodules {
        let version = submodule.head_id().unwrap().to_string();
        let installed = if is_installed(submodule.name().unwrap()) { "Yes" } else { "No" };
        println!("{: ^11}|  {: <38}|  {: <43}", installed, submodule.name().unwrap(), version);
    }
}

fn install(name: &str, verbose: bool) {
    let install_path = get_install_path(name);
    println!("Attempting to install '{}' as '{}'", name, install_path.file_name().unwrap().to_string_lossy());
    if is_installed(name) {
        println!("Link exists - {} is already installed? Aborting", name);
        return;
    }

    let repo = open_repo(verbose);
    let mut submodule = match repo.find_submodule(name) {
        Ok(submodule) => submodule,
        Err(e) => panic!("Plug-In not found in submodules: {}", e),
    };
    match submodule.update(true, None) {
        Ok(submodule) => submodule,
        Err(e) => panic!("Failed to update submodule: {}", e),
    }

    let source_path = get_repo_dir(verbose).join(submodule.path());
    if verbose {
        println!("Linking '{}' to '{}'", source_path.to_string_lossy(), install_path.to_string_lossy());
    }

    if cfg!(windows) {
        if verbose {
            println!("Using Windows workaround");
        }
        let status = Command::new("cmd")
            .args(&["/C", "mklink", "/D", install_path.to_str().unwrap(), source_path.to_str().unwrap()])
            .status()
            .expect("Failed to create Symlink");
        if verbose {
            println!("mklink status: '{}'", status.to_string())
        }
        if !status.success() {
            panic!("mklink returned non-zero exit status - Failed to create Symlink")
        }
    } else {
        symlink_dir(source_path, install_path)
            .expect("Failed to create Symlink");
    }

    println!("Done.")
}