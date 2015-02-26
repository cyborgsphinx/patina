use std::os;
use std::string::String;
use std::vec::Vec;
use std::str::StrExt;
use std::env;
use std::default::Default;

pub fn arguments(input: &str) -> String {
    let mut res = String::new();

    if input.starts_with("~") || input.starts_with("./") {
        res = path(input);
    } else {
        //res = string(input);
    }

    res
}
// string parsing goes here (need to not return a vector)
pub fn string(st: String, vars: Vec<(String, String)>) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let mut in_sin_quotes = false;   // keeps track of whether or not to bother with a space
    let mut in_dub_quotes = false;  // need separate flags for double and single quotes
    let mut out = String::new();
    // Expand variables with $, keep anything in between "" or ''
    for word in st.words() {
        if word.starts_with("\"") {
            if in_sin_quotes {
                out.push_str(word);
            } else {
                in_dub_quotes = true;
                out.push_str(word.trim_matches('"'));
            }
        } else if word.starts_with("'") {
            if in_dub_quotes {
                out.push_str(word);
            } else {
                in_sin_quotes = true;
                out.push_str(word.trim_matches('\''));
            }
        } else if word.starts_with("$") {
            if in_dub_quotes || in_sin_quotes {
                res.push(word.to_string());
            } else {
                let key = word.slice_chars(1, word.len());
                let value = match env::var(key) {
                    Ok(val) => val,
                    Err(_) => {
                        let mut i: usize = 0;
                        let mut val: (String, String) = Default::default();
                        while i < vars.len() {
                            if vars[i].0 == key.to_string() {
                                val = vars[i].clone();
                                break;
                            }
                        }
                        val.1
                    },
                };
                res.push(value);
            }
        } else if word.ends_with("\"") {
            if in_sin_quotes {
                out.push_str(word);
            } else {
                in_dub_quotes = false;
                out.push_str(word.trim_matches('"'));
                res.push(out.clone());
                out.clear();
            }
        } else if word.ends_with("'") {
            if in_dub_quotes {
                out.push_str(word);
            } else {
                in_sin_quotes = false;
                out.push_str(word.trim_matches('\''));
                res.push(out.clone());
                out.clear();
            }
        }
    }
    /*for c in st.chars() {
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
    }*/
    return res;
}

// string parsing for creating paths (~ -> /home/$USER)
pub fn path(st: &str) -> String {
    let mut res = String::new();
    let home: String = match env::var("HOME") { // os::homedir returns a path, this retruns a string
        Ok(val) => val,
        Err(..) => "Home directory not set".to_string() // will return this string
    };
    if st.starts_with("~/") {
        res.push_str(home.as_slice());
        res.push('/');
        for c in st.slice_chars(2, st.len()).chars() {
            res.push(c);
        }
    } else {
        res.push_str(st);
    }
    return res;
}

#[test]
fn path_test() {
    let this = path("~/Downloads".to_string());
    assert_eq!(this.as_slice(), "/home/james/Downloads");
}
