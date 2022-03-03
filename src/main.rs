use std::{fs, env};
use std::fs::File;
use std::io::prelude::*;
use hex::encode;
use sha1::{Sha1, Digest};

fn init_repo(repo: &str) {
    match fs::create_dir(repo) {
        Ok(_) => {
            let names = ["objects", "refs", "refs/heads"];
            env::set_current_dir(repo).unwrap();
            fs::create_dir(".git").unwrap();
            env::set_current_dir(".git").unwrap();
            for name in names {
                fs::create_dir(name).unwrap();
            }
            let mut head = File::create("HEAD").unwrap();
            head.write_all(b"ref: refs/heads/master").unwrap();
            println!("Initialized empty repository.");
        },
        Err(_) => println!("Failed to initialize repo."),
    }
}

fn hash_object(data: &str, obj_type: &str, write: bool) {
    let mut header = format!("{} {}", data.len(), obj_type);
    let full_data = format!("{}{}{}", header, b'\x00', data);
    let mut hasher = Sha1::new();
    hasher.update(full_data.as_bytes());
    let sha1 = hasher.digest().to_string();
}

fn main() {
    init_repo("fuck");
}