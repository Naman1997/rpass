#![deny(warnings)]

use std::env;
use std::path::Path;
use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{Error, FetchOptions, RemoteCallbacks, Cred};

 
fn run() -> Result<(), Error> {
    let home_dir = home::home_dir().expect("Unable to detect home dir!");
    let rpass_dir = "/.config/rpass";
    let full_path = format!("{}{}", home_dir.display(), rpass_dir);

    let repo_url = "git@github.com:Naman1997/My-notes.git";

    println!("Cloning {} into {}", repo_url, full_path);

    let mut callbacks = RemoteCallbacks::new();
    let mut fetch_options = FetchOptions::new();
    let checkout_options = CheckoutBuilder::new();

    callbacks.credentials(|_, _, _| {
        let credentials =
            Cred::ssh_key(
                "git",
                None,
                std::path::Path::new(&format!("{}/.ssh/id_ed25519", env::var("HOME").unwrap())),
                None
            ).expect("Could not create credentials object");


        Ok(credentials)
    });

    fetch_options.remote_callbacks(callbacks);

    if !Path::new("/etc/hosts").exists() {
        RepoBuilder::new()
        .fetch_options(fetch_options)
        .with_checkout(checkout_options)
        .clone(repo_url, Path::new(&full_path))?;

        println!("Clone complete");
    } else {
        println!("Clone already exists");
    }
    // Do things with `repo` here

    Ok(())
}

fn main() {
    match run() {
        Ok(()) => {}
        Err(e) => println!("error: {}", e),
    }
}