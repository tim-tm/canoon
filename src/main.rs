use std::{
    env::args,
    fs::{create_dir, read_to_string, remove_file, write},
    path::Path,
    process::{exit, Command},
};

use is_root::is_root;
use dialoguer::Confirm;

static CURRENT_COMMIT_DIR: &str = "/var/lib/cano/current_commit.txt";

macro_rules! move_file {
    ($from:expr, $to:expr) => {
        Command::new("cp").arg($from).arg($to).output().unwrap();
    };
}

macro_rules! force_delete {
    ($path:expr) => {
        Command::new("rm").arg("-rf").arg($path).output().unwrap();
    };
}

fn install(latest_commit_hash: &str) {
    Command::new("git")
        .arg("clone")
        .arg("https://github.com/CobbCoding1/Cano")
        .output()
        .unwrap();

    Command::new("make").current_dir("./Cano").output().unwrap();
    move_file!("./Cano/build/cano", "/usr/bin/");
    create_dir("/var/lib/cano/").unwrap();
    write(CURRENT_COMMIT_DIR, latest_commit_hash).unwrap();
    force_delete!("./Cano");
}

fn uninstall() {
    remove_file("/usr/bin/cano").unwrap();
    force_delete!("/var/lib/cano/");
}

fn update(cano_installed: bool, latest_commit_hash: &str) {
    if cano_installed {
        if let Ok(current_local_commit) = read_to_string(CURRENT_COMMIT_DIR) {
            if current_local_commit == latest_commit_hash {
                println!("Cano's up-to-date :O");
            } else {
                println!("An update's available! Installing it...");
                uninstall();
                install(&latest_commit_hash);
                println!("Successfully installed.");
            }
        }
    } else {
        println!("Cano isn't installed.");
    }
}

fn main() {
    if !is_root() {
        println!("The installer must be run as root.");
        exit(0);
    }

    if Command::new("git").output().is_err() {
        println!("You must have git installed.");
        exit(0);
    }

    // Get the latest commit of the main branch
    let latest_commit_hash = String::from_utf8(
        Command::new("git")
            .arg("ls-remote")
            .arg("https://github.com/CobbCoding1/Cano.git")
            .arg("main")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .split('\t')
    .collect::<Vec<_>>()
    .first()
    .unwrap()
    .to_string();

    let cano_installed = Path::new("/usr/bin/cano").is_file();

    let mut args = args().collect::<Vec<_>>();
    args.remove(0);
    if args.len() == 0 {
        println!("Usage: canoon <install/uninstall/update>");
        let should_install = Confirm::new()
            .with_prompt("Do you want to install cano?")
            .interact()
            .unwrap();
        
        if should_install {
            if cano_installed {
                println!(
                    "Cano is already installed."
                );
                let should_update = Confirm::new()
                    .with_prompt("Do you want to update cano instead?")
                    .interact()
                    .unwrap();
                
                if should_update {
                    update(cano_installed, &latest_commit_hash);
                }
            } else {
                println!("Installing Cano...");
                install(&latest_commit_hash);
                println!("Successfully installed.");
            }
        }
        exit(0);
    } else if args.len() != 1 {
        println!("Usage: canoon <install/uninstall/update>");
        exit(0)
    }

    match args.first().unwrap().as_str() {
        "install" => {
            if cano_installed {
                println!(
                    "Cano is already installed."
                );
                let should_update = Confirm::new()
                    .with_prompt("Do you want to update cano instead?")
                    .interact()
                    .unwrap();
                
                if should_update {
                    update(cano_installed, &latest_commit_hash);
                }
            } else {
                println!("Installing Cano...");
                install(&latest_commit_hash);
                println!("Successfully installed.");
            }
        }
        "uninstall" => {
            if cano_installed {
                println!("Uninstalling Cano...");
                uninstall();
                println!("Successfully uninstalled.");
            } else {
                println!("Cano isn't installed.");
            }
        }
        "update" => {
            update(cano_installed, &latest_commit_hash);
        }
        _ => {
            println!("Usage: canoon <install/uninstall/update>");
        }
    }
}
