use std::os;
use std::string::String;
use std::vec::Vec;

// string parsing (also arguments) goes here
pub fn string(st: &str) -> Vec<String> {
    let mut res = Vec::new();
    let mut in_sin_quotes = false;   // keeps track of whether or not to bother with a space
    let mut in_dub_quotes = false;  // need separate flags for double and single quotes
    let mut out = String::new();
    // Expand variables with $, keep anything in between "" or ''
    for c in st.chars() {
        match c {
            '"' => {
                if in_sin_quotes {
                    out.push(c);
                } else {
                    if in_dub_quotes {
                        in_dub_quotes = false;
                    } else {
                        in_dub_quotes = true;
                    }
                    //out.push(c); //do we want to keep the quotation marks?
                }
            },
            '$' => {
                if in_dub_quotes || in_sin_quotes {
                    out.push(c);
                } else {
                    //deal with variable expansion
                }
            },
            '\'' => {
                if in_dub_quotes {
                    out.push(c);
                } else {
                    if in_sin_quotes {
                        in_sin_quotes = false;
                    } else {
                        in_sin_quotes = true;
                    }
                    //out.push(c); //do we want to keep the quotation marks?
                }
            },
            ' ' => {
                if in_dub_quotes || in_sin_quotes {
                    out.push(c);
                } else {
                    res.push(out.clone());
                    out.clear();
                }
            },
            _ => {
                out.push(c);
            },
        };
    }
    return res;
}

// string parsing for creating paths (~ -> /home/$USER)
pub fn path(st: String) -> String {
    let mut res = String::new();
    let home: String = match os::getenv("HOME") { // os::homedir returns a path, this retruns a string
        Some(val) => val,
        None => "Home directory not set".to_string() // will return this string
    };
    for c in st.chars() {
        match c {
            '~' => res.push_str(home.as_slice()),   // assumes that ~ is not part of a name
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
