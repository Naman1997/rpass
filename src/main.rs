#![deny(warnings)]

use std::env;
use std::path::Path;
use git2::build::{CheckoutBuilder, RepoBuilder};
use git2::{Error, FetchOptions, RemoteCallbacks, Cred};

const RPASS_DIR: &str = "/.config/rpass";

 
fn run(full_path: &str) -> Result<(), Error> {
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

    if !Path::new(full_path).exists() {
        RepoBuilder::new()
        .fetch_options(fetch_options)
        .with_checkout(checkout_options)
        .clone(repo_url, Path::new(&full_path))?;

        println!("Clone complete");
        println!("rpass will now manage this dir and repo to commit your encrypted secrets!")
    } else {
        println!("Clone already exists");
    }
    // Do things with `repo` here

    Ok(())
}

fn main() {
    let home_dir = home::home_dir().expect("Unable to detect home dir!");
    let full_path = format!("{}{}", home_dir.display(), RPASS_DIR);
    let subcommand = std::env::args().nth(1).expect("No subcommand provided. #TODO replace with help section.");
    if subcommand.eq_ignore_ascii_case("init") {
        if !Path::new(&full_path).exists() {
            match run(&full_path) {
                Ok(()) => {}
                Err(e) => println!("error: {}", e),
            }
        } else {
            println!("rpass config dir exists. Backup and delete the dir manually to re-initiate a new config.")
        }
    } else if subcommand.eq_ignore_ascii_case("") {
        
    }

}