use std::env;
use std::string::String;
use std::num;
use std::num::strconv;
use std::ffi::OsStr;

pub fn get_prompt(status: isize) -> String {
    let cwd = match env::current_dir() {
        Ok(d) => {d},
        Err(f) => {panic!(f.to_string())},
    };
    let dir = match cwd.file_name() {
        Some(d) => {d},
        None => {OsStr::from_str("/")},
    };
    let dispdir = dir.to_str().unwrap_or("Could not find directory");
    let mut pro = String::new();
    let fstat: f64 = num::cast(status).unwrap();
    //this function is really complicated
    let (dispstat, flag) = strconv::float_to_str_common(
        fstat,
        10,
        true,
        strconv::SignFormat::SignNeg,
        strconv::SignificantDigits::DigAll, //this could go poorly, but i hope not
        strconv::ExponentFormat::ExpNone,
        false);
    if status != 0 && !flag {
        pro.push('(');
        pro.push_str(dispstat.as_slice());
        pro.push(')');
        pro.push(' ');
    }
    pro.push_str(dispdir);
    pro.push_str(" $ ");
    return pro;
}

#[cfg(test)]
mod tests {
    use prompt::get_prompt;
    use std::env;
    use std::string::String;
    use std::path;

    #[test]
    fn status_is_zero() {
        let dir = env::current_dir().unwrap();
        let file = dir.file_name().unwrap();
        let dir_st = file.to_str().unwrap();
        let mut out = String::new();
        out.push_str(dir_st);
        out.push_str(" $ ");
        assert_eq!(out, get_prompt(0));
    }

    #[test]
    fn status_non_zero() {
        let dir = env::current_dir().unwrap();
        let file = dir.file_name().unwrap();
        let dir_st = file.to_str().unwrap();
        let mut out = String::new();
        out.push_str("(101) ");
        out.push_str(dir_st);
        out.push_str(" $ ");
        assert_eq!(out, get_prompt(101));
    }
}
