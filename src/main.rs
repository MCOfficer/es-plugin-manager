extern crate app_dirs;
extern crate clap;
extern crate dirs;
extern crate runas;
extern crate symlink;

use std::path::PathBuf;

use app_dirs::*;
use clap::{App, Arg, SubCommand};
use dirs::data_dir;
use runas::Command;
use symlink::{remove_symlink_dir, symlink_dir};

mod git;

const VERSION: &str = "0.2.0";
const REPO_URL: &str = "https://github.com/MCOfficer/endless-sky-plugins.git";
const APP_INFO: AppInfo = AppInfo {
    name: "ESPIM",
    author: "MCOfficer",
};

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
        .subcommand(SubCommand::with_name("upgrade")
            .about("Upgrades all installed and  plug-ins")
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
        .subcommand(SubCommand::with_name("remove")
            .about("Removes a plug-in. The Plug-In stays on the disk, but is removed from the plug-in folder.")
            .alias("uninstall")
            .arg(Arg::with_name("PLUGIN")
                .help("The plug-in to remove.")
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
    } else if let Some(_matches) = matches.subcommand_matches("update") {
        update(verbose);
    } else if let Some(_matches) = matches.subcommand_matches("upgrade") {
        upgrade(verbose);
    } else if let Some(_matches) = matches.subcommand_matches("list") {
        list(verbose);
    } else if let Some(matches) = matches.subcommand_matches("install") {
        install(matches.value_of("PLUGIN").unwrap(), verbose);
    } else if let Some(matches) = matches.subcommand_matches("remove") {
        remove(matches.value_of("PLUGIN").unwrap(), verbose);
    } else {
        println!("See espim -h for help");
    }
}

fn get_plugin_dir() -> PathBuf {
    data_dir()
        .expect("dirs failed to find data_dir")
        .join("endless-sky")
        .join("plugins")
}

fn get_install_path(name: &str) -> PathBuf {
    get_plugin_dir().join("[ESPIM] ".to_string() + name)
}

fn is_installed(name: &str) -> bool {
    get_install_path(name).exists()
}

fn get_repo_dir(verbose: bool) -> PathBuf {
    let repo_dir = get_app_dir(AppDataType::UserCache, &APP_INFO, "repo")
        .expect("app_dirs failed with an error");
    if verbose {
        println!(
            "app_dirs returned {} as repo directory",
            repo_dir.to_string_lossy()
        );
    }
    repo_dir
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
    git::clone(REPO_URL, repo_dir);
    println!("Done.")
}

fn update(verbose: bool) {
    let repo_dir = get_repo_dir(verbose);
    if !repo_dir.exists() && verbose {
        println!("Repo directory does not exist. Did you run 'espim init'?");
        return;
    }
    git::update_repo(repo_dir, verbose);
    println!("Done.");
}

fn upgrade(verbose: bool) {
    let repo = git::open_repo(get_repo_dir(verbose), verbose);
    let submodules = repo.submodules().expect("Failed to load Submodules");
    for mut submodule in submodules {
        if is_installed(submodule.name().unwrap()) {
            println!("=> Updating {}", submodule.name().unwrap().to_string());
            submodule
                .update(true, None)
                .expect("Failed to update submodule");
        }
    }
    println!("Done.")
}

fn list(verbose: bool) {
    let repo = git::open_repo(get_repo_dir(verbose), verbose);
    let submodules = repo.submodules().expect("Failed to load Submodules");
    println!("{: ^11}|{: ^40}|{:^45}", "Installed", "Name", "Version");
    println!("{:-<11}|{:-<40}|{:-<45}", "", "", "");
    for submodule in &submodules {
        let version = submodule.head_id().unwrap().to_string();
        let installed = if is_installed(submodule.name().unwrap()) {
            "Yes"
        } else {
            "No"
        };
        println!(
            "{: ^11}|  {: <38}|  {: <43}",
            installed,
            submodule.name().unwrap(),
            version
        );
    }
}

fn install(name: &str, verbose: bool) {
    let install_path = get_install_path(name);
    println!(
        "Attempting to install '{}' as '{}'",
        name,
        install_path.file_name().unwrap().to_string_lossy()
    );
    if is_installed(name) {
        println!("Link exists - {} is already installed? Aborting", name);
        return;
    }

    let repo = git::open_repo(get_repo_dir(verbose), verbose);
    let mut submodule = repo
        .find_submodule(name)
        .expect("Plug-In not found in submodules");
    submodule
        .update(true, None)
        .expect("Failed to update submodule");

    let source_path = get_repo_dir(verbose).join(submodule.path());
    if verbose {
        println!(
            "Linking '{}' to '{}'",
            source_path.to_string_lossy(),
            install_path.to_string_lossy()
        );
    }

    if cfg!(windows) {
        if verbose {
            println!("Using Windows workaround");
        }
        let status = Command::new("cmd")
            .args(&[
                "/C",
                "mklink",
                "/D",
                install_path.to_str().unwrap(),
                source_path.to_str().unwrap(),
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
        symlink_dir(source_path, install_path).expect("Failed to create Symlink");
    }

    println!("Done.")
}

fn remove(name: &str, verbose: bool) {
    if !is_installed(name) {
        println!("Link does not exist - {} is not installed? Aborting", name);
        return;
    }
    let link = get_install_path(name);
    if verbose {
        println!("Removing Symlink '{}'", link.to_string_lossy());
    }

    remove_symlink_dir(link).expect("Failed to remove Symlink");
    println!("Done.");
}
