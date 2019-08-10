extern crate git2;

use git2::{Repository, ResetType};

use std::path::PathBuf;

pub fn clone(repo_url: &str, repo_dir: &PathBuf) -> Repository {
    println!(
        "Cloning {} into directory {}",
        repo_url,
        repo_dir.to_string_lossy()
    );
    Repository::clone(repo_url, repo_dir).expect("Failed to Clone Repository")
}

pub fn open_repo(repo_dir: &PathBuf, verbose: bool) -> Repository {
    if verbose {
        ("Opening Repository {}", repo_dir.to_string_lossy());
    };
    Repository::open(repo_dir.as_path()).expect("Failed to open Repository")
}

// TODO: actually check out at a certain commit/tag
pub fn checkout_repo_at(repo_dir: &PathBuf, rev_str: &str, verbose: bool) {
    println!("Checking out revision {}", rev_str);
    let repo = open_repo(repo_dir, verbose);
    let mut remote = repo
        .find_remote("origin")
        .expect("Failed to find remote 'origin'");
    remote
        .fetch(&["master"], None, None)
        .expect("Failed to fetch repository");

    let revision = repo
        .revparse_single(rev_str)
        .expect("Failed to find revision");

    let mut head = repo.head().expect("Failed to find HEAD"); // By now, the reference is stale
    head.set_target(revision.id(), "Fast-Forwarding")
        .expect("Failed to set HEAD target");
    repo.reset(&revision, ResetType::Hard, None)
        .expect("Failed to perform hard reset");
}
