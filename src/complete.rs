extern crate linenoise;

use std::string::String;
use std::io;
use std::os;
use std::str;

pub fn complete(st: &str) -> Vec<&str> {
    let mut v: Vec<&str> = st.split(' ').collect(); // needs to be mut because pop()
    let mut res: Vec<&str> = Vec::new();
    let s = match v.pop() {
        Some(r) => r,
        None => "Not there",
    };
    if s.contains_char(' ') {
        return res;  // returns an empty vector if things go wrong
    }
    
    match s {
        "g" | "gi" => res.push("git"),
        "p" | "pa" | "pac" | "pacm" | "pacma" => res.push("pacman"),
        _ => res.push("Not Yet Implimented"),
    };

    return res;
}
