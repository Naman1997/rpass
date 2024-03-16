#![deny(warnings)]

use git2::{Error, Direction, Repository};
use std::path::PathBuf;

 
fn run() -> Result<(), Error> {
    let home_dir = home::home_dir().expect("Unable to detect home dir!");
    let rpass_dir = "/.config/rpass";
    let full_path = format!("{}{}", home_dir.display(), rpass_dir);
    let mut path = PathBuf::from(full_path);

    let repo = match Repository::open(&path) {
        Ok(repo) => {
            print!("Using existing repo configured at ~/.config/rpass\n");
            repo
        },
        Err(_e) => {
            // Init the repo if it does not exist on that path
            let repo = Repository::init(&path)?;
            path = repo.workdir().unwrap().to_path_buf();
            println!("Initialized empty Git repository in {}", path.display());

            create_initial_commit(&repo)?;
            println!("Created empty initial commit");
            repo
        },
    };

    // Check for exiting remotes and add if missing
    let remote = "origin";
    let mut remote = repo
        .find_remote(remote)
        .or_else(|_| repo.remote_anonymous(remote))?;

    // Connect to the remote and call the printing function for each of the
    // remote references.
    let connection = remote.connect_auth(Direction::Fetch, None, None)?;

    // Get the list of references on the remote and print out their name next to
    // what they point to.
    for head in connection.list()?.iter() {
        println!("{}\t{}", head.oid(), head.name());
    }

    Ok(())
}
 
fn create_initial_commit(repo: &Repository) -> Result<(), Error> {
    let sig = repo.signature()?;
    let tree_id = {
        let mut index = repo.index()?;
        index.write_tree()?
    };

    let tree = repo.find_tree(tree_id)?;
    repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?;
    Ok(())
}

fn main() {
    match run() {
        Ok(()) => {}
        Err(e) => println!("error: {}", e),
    }
}