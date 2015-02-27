// not being used anymore

#![feature(old_path)]
#![feature(old_io)]
#![feature(collections)]
#![feature(os)]

extern crate core;

use std::string::String;
use std::old_io::fs;
use std::old_io::fs::PathExtensions;
use std::os;
use std::env;
use std::str;
use std::old_path::Path;

pub fn complete(st: &str) -> Vec<String> {
    let mut v: Vec<&str> = st.split(' ').collect();
    let mut res: Vec<String> = Vec::new();

    if v.len() == 1 {
        let search = match env::var("PATH") {
            Ok(p) => p,
            Err(..) => "That didn't work".to_string(),
        };
         if search == "That didn't work".to_string() {
             return Vec::new();
         }

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
                        res.push(exe.to_string().clone());
                    }
                }
            }
        }
    } else {
        let search = match v.pop() {
            Some(s) => s,
            None => "",
        };
        let mut out = String::new();
        for substr in v.drain() {
            out.push_str(substr);
        }
        let path = Path::new(search);
        let dir = path.dirname();
        let fname = str::from_utf8(path.filename().unwrap()).unwrap();  //not robust
        let path_dir = Path::new(dir);
        let contents = match fs::readdir(&path_dir) {
            Ok(s) => s,
            Err(_) => Vec::<Path>::new(),   // gives empty vector; nothing to read
        };
        for entry in contents.iter() {
            // TODO: make more robust
            let file = str::from_utf8(entry.filename().unwrap()).unwrap();
            if file.starts_with(fname) {
                let mut output = String::new();
                output.push_str(out.as_slice());
                output.push(' ');
                output.push_str(str::from_utf8(dir).unwrap());
                output.push('/');
                output.push_str(file);
                res.push(output);
            }
        }
    }

    res.sort();
    return res;
}
