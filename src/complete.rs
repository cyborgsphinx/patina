//extern crate linenoise;
extern crate core;

use std::boxed::Box;
use std::string::String;
use std::old_io as io;
use self::io::fs;
use self::io::fs::PathExtensions;
use std::os;
use std::str;
use std::path::Path;
use self::core::ops::Deref;

pub fn complete(st: &str) -> Vec<String> {
    let v: Vec<&str> = st.words().collect();
    let mut res: Vec<String> = Vec::new();

    if v.len() == 1 {
        res = program(st);
    } else {
        res = pathname(st);
    }

    return res;
}

fn program(st: &str) -> Vec<String> {
    let mut matches: Vec<String> = Vec::new();
    let search = match os::getenv("PATH") {
        Some(p) => p,
        None => "That didn't work".to_string(),
    };

    let paths: Vec<&str> = search.as_slice().split(':').collect();
    for directory in paths.iter() {
        let dir = Path::new(directory);
        if dir.is_dir() {   // because some people might be stupid
            let contents = match fs::readdir(&dir) {
                Ok(s) => s,
                Err(_) => Vec::new(),
            };
            for entry in contents.iter() {
                // TODO: make more robust
                let exe = str::from_utf8(entry.filename().unwrap()).unwrap();
                if exe.starts_with(st) {
                    matches.push(exe.to_string().clone());
                }
            }
        }
    }
    return matches;
}

fn pathname(st: &str) -> Vec<String> {
    let mut matches: Vec<String> = Vec::new();
    let mut path = Path::new(st);
    let dir = match str::from_utf8(path.dirname()) {
        Ok(s) => s,
        Err(f) => panic!("failed at {}", f),    // not robust
    };
    println!("{}", dir);
    let path_dir = Path::new(dir);
    println!("{}", path_dir.display());
    if fs::readdir(&path_dir).is_ok() {
        println!("Works");
    } else {
        println!("Doesn't");
    }
    let contents = match fs::readdir(&path_dir) {
        Ok(s) => s,
        Err(_) => Vec::<Path>::new(),   // gives empty vector; nothing to read
    };
    for entry in contents.iter() {
        // TODO: make more robust
        let file = str::from_utf8(entry.filename().unwrap()).unwrap();
        if file.starts_with(st) {
            matches.push(file.to_string().clone());
        }
    }
    return matches;
}

fn main() {
    let mut vic = pathname("/home/james/Downloads/o");
    for elem in vic.drain() {
        println!("{}", elem);
    }
    vic = pathname("/usr/bin");
    for elem in vic.drain() {
        println!("{}", elem);
    }
    vic = program("pac");
    for elem in vic.drain() {
        println!("{}", elem);
    }
}
