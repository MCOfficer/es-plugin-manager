extern crate git2;

use git2::{Repository, ResetType};

use std::path::PathBuf;

pub fn clone(repo_url: &str, repo_dir: &PathBuf) -> Repository {
    Repository::clone(repo_url, repo_dir).expect("Failed to Clone Repository")
}

pub fn open_repo(repo_dir: &PathBuf, verbose: bool) -> Repository {
    if verbose {
        ("Opening Repository {}", repo_dir.to_string_lossy());
    };
    Repository::open(repo_dir.as_path()).expect("Failed to open Repository")
}

// TODO: actually check out at a certain commit/tag
pub fn checkout_repo_at(repo_dir: &PathBuf, verbose: bool) {
    let repo = open_repo(repo_dir, verbose);
    let mut remote = repo
        .find_remote("origin")
        .expect("Failed to find remote 'origin'");
    remote
        .fetch(&["master"], None, None)
        .expect("Failed to fetch repository");

    let mut master_ref = repo
        .find_reference("refs/heads/master")
        .expect("Failed to get master reference");
    let remote_master_ref = repo
        .find_reference("refs/remotes/origin/master")
        .expect("Failed to get remote master reference");
    let remote_master_commit = remote_master_ref
        .peel_to_commit()
        .expect("Failed to peel remote master reference");

    repo.checkout_tree(remote_master_commit.as_object(), None)
        .expect("Failed to checkout tree");
    master_ref
        .set_target(remote_master_commit.id(), "Fast-Forwarding")
        .expect("Failed to set master ref target");
    let mut head = repo.head().expect("Failed to find HEAD"); // By now, the reference is stale
    head.set_target(remote_master_commit.id(), "Fast-Forwarding")
        .expect("Failed to set HEAD target");
    repo.reset(&remote_master_commit.into_object(), ResetType::Hard, None)
        .expect("Failed to perform hard reset");
}
