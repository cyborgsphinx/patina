use std::os;
use std::string::String;

// string parsing (also arguments) goes here
pub fn string(st: String) -> String {
    let mut res = String::new();
    // Expand variables with $, keep anything in between "" or ''
    return res;
}

// string parsing for creating paths (~ -> /home/$USER)
pub fn path(st: String) -> String {
    let mut res = String::new();
    let home: String = match os::getenv("HOME") { // os::homedir returns a path, this retruns a string
        Some(val) => val,
        None => panic!("Home directory not set") // improve robustness
    };
    for c in st.chars() {
        match c {
            '~' => res.push_str(home.as_slice()),
            _ => res.push(c),
        };
    }
    return res;
}

fn main() {
    let this = path(String::from_str("~/Downloads"));
    assert_eq!(this.as_slice(), "/home/james/Downloads");
    println!("You did it!");
}