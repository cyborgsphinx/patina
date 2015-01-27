extern crate linenoise;
extern crate core;

use std::boxed::Box;
use std::string::String;
use std::io;
use std::io::fs;
use std::io::fs::PathExtensions;
use std::os;
use std::str;
use std::path::Path;
use self::core::ops::Deref;

pub fn complete(st: &str) -> Vec<String> {
    let mut v: Vec<&str> = st.split(' ').collect(); // needs to be mut because pop()
    let mut res: Vec<String> = Vec::new();

    if v.len() == 1 {
        res = program(st);
    } else {
        let s = match v.pop() {
            Some(r) => r,
            None => "Not there",
        };
        if s.contains_char(' ') {
            return Vec::<String>::new();  // returns an empty vector if things go wrong
        }

        match s {
            "g" | "gi" => res.push("git".to_string()),
            "p" | "pa" | "pac" | "pacm" | "pacma" => res.push("pacman".to_string()),
            _ => res.push("Not Yet Implimented".to_string()),
        };
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
