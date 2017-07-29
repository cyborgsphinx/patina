use std::string::String;
use std::vec::Vec;
use std::collections::HashMap;
use std::env;
use std::str::Chars;

// splits a string into useful tokens for easier parsing
fn tokenize(input: &str, vars: &HashMap<String, String>) -> Vec<String> {
    let mut res = Vec::new();
    let mut buf = String::new();
    // get an environment variable
    let get_var = |chars: &mut Chars| { // capturing chars and vars
        let mut var_name = String::new();
        while let Some(c) = chars.next() {
            match c {
                ' ' | '\n' | '\t' => { break; },
                ch => var_name.push(ch),
            }
        }
        match vars.get(&var_name) {
            Some(val) => val.clone(),
            None => String::new(),
        }
    };
    // get a string literal
    let get_string = |chars: &mut Chars, end_val| {
        let mut str_buf = String::new();
        // ignore when using 'single quotes' but not "double quotes"
        let ignore_vars = end_val == '\'';
        let mut ignore_quote = false;
        while let Some(c) = chars.next() {
            match c {
                '\\' => ignore_quote = true,
                '\'' => if ignore_vars && !ignore_quote { break; } else { str_buf.push('\''); },
                '"' => if !ignore_vars && !ignore_quote { break; } else { str_buf.push('"'); },
                '$' => if !ignore_vars { str_buf.push_str(get_var(chars).as_str()); } else { str_buf.push('$'); },
                ch => str_buf.push(ch),
            };
        }
        str_buf
    };
    let mut chars = input.chars();
    let mut ignore = false;
    while let Some(c) = chars.next() {
        match c {
            // token types go here
            '\\' => ignore = true,
            '&' => res.push(String::from("&")),
            '|' => res.push(String::from("|")),
            '$' => res.push(get_var(&mut chars)),
            ' ' => {
                if !ignore {
                    res.push(buf.clone());
                    buf.clear();
                }
            },
            '\n' | '\t' => {
                res.push(buf.clone());
                buf.clear();
            }
            '\'' => res.push(get_string(&mut chars, '\'')),
            '"' => res.push(get_string(&mut chars, '"')),
            ch => buf.push(ch),
        };
        if c != '\\' {
            // avoid repeating this for every other case
            ignore = false;
        }
    }
    if !buf.is_empty() {
        res.push(buf);
    }
    res
}

///Parses a line given to patina
pub fn line(st: String, vars: &HashMap<String, String>) -> Vec<String> {
    // just tokenize for now
    tokenize(st.trim(), vars)
}

///Parses a string into a valid path
///Primarily used to convert ~ into the home directory
pub fn path(input: &str) -> Result<String, &'static str> {
    let home = env::var("HOME").map_err(|_| "Home directory not set");
    if input.starts_with("~/") {
        home.map(|s| s + &input[1..])
    } else {
        Ok(String::from(input))
    }
}

#[cfg(test)]
mod test {
    use parse::path;
    use std::env;

    #[test]
    fn path_test() {
        let this = path("~/Downloads").expect("This path should be valid");
        let mut expected = env::home_dir().expect("Home directory not found");
        expected.push("Downloads");
        assert_eq!(this, expected.to_string_lossy());
    }
}
