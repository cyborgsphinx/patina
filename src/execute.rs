extern crate glob;

use self::glob::{glob_with, MatchOptions};
use std::path::PathBuf;
use std::process::Command;
use std::os::unix::process::ExitStatusExt;

pub fn run(cmd: &str, args: Vec<&str>) -> i32 {
    let options = MatchOptions {
        case_sensitive: true,               //case sensitivity - may change
        require_literal_separator: false,   //require file separators
        require_literal_leading_dot: true,  //whether or not hidden files are included
    };
    let mut child = Command::new(cmd);
    for arg in &args {
        if arg.contains("*") || arg.contains("?") || arg.contains("[") {
            match glob_with(&arg, &options) {
                //this looks bad. it probably is. but i'm not too concerned with errors right now.
                Ok(vals) => {
                    let mut vals = vals.peekable();
                    //if it's an empty iterator, we'll push it back in case it was actually a regex
                    if vals.peek().is_none() {
                        child.arg(arg);
                    } else {
                        for val in vals {
                            child.arg(val.unwrap_or(PathBuf::new()));
                        }
                    }
                },
                //on error, just add the arg as is
                Err(..) => {child.arg(arg);},
            };
        } else {
            child.arg(arg);
        }
    }
    match child.status() {
        Ok(val) => {
            match val.code() {
                Some(num) => num,
                None => match val.signal() {
                    Some(sig) => sig*-1,    //give signal value
                    None => -1,             //something went weird
                },
            }
        },
        Err(..) => {
            println!("patina: command not found: {}", cmd);
            127
        },
    }
}
