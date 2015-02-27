#![feature(old_io)]
#![feature(os)]
#![feature(old_path)]

use std::os;
use std::path::PathBuf;
use std::old_path;
use std::old_io::process::Command;
use std::string::String;

// really just a wrapper around os::change_dir()
// returns 0 for success and 1 for failure
pub fn ch_dir(dest: PathBuf) -> i32 {
    match os::change_dir(&old_path::Path::new(dest.to_str().unwrap())) {
        Ok(..) => 0,
        Err(..) => {
            println!("Failed changing directory");
            1
        },
    }
}

pub fn main() {
    let dir = PathBuf::new("..");
    let num = ch_dir(dir);
    println!("{}", num);
    let out = match Command::new("pwd").output() {
        Ok(p) => {p},
        Err(_) => {panic!("Well fuck")},
    };
    println!("{}", String::from_utf8_lossy(out.output.as_slice()));
}
