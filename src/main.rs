extern crate app_dirs;
extern crate clap;
extern crate git2;
extern crate remove_dir_all;

use std::path::PathBuf;

use app_dirs::*;
use clap::{App, Arg, SubCommand};
use git2::Repository;
use remove_dir_all::remove_dir_all;

const VERSION: &str = "0.1.0";
const REPO_URL: &str = "https://github.com/MCOfficer/endless-sky-plugins.git";
const APP_INFO: AppInfo = AppInfo{name: "ESPIM", author: "MCOfficer"};

fn main() {
    let matches = App::new("Endless Sky Plug-In Manager")
        .version(VERSION)
        .about("A Proof-of-Concept Plug-In Manager for Endless Sky.")
        .subcommand(SubCommand::with_name("init")
            .about("Clones the Plug-In repository. Use before anything else.")
        )
        .subcommand(SubCommand::with_name("update")
            .about("Updates the local Plug-In repository.")
        )
        .subcommand(SubCommand::with_name("list")
            .about("Lists all available plug-ins.")
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
    for submodule in &submodules {
        println!("- {}", submodule.name().unwrap());
    }
}
