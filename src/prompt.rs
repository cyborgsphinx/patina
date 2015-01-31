use std::char;
use std::os;
use std::string::String;
use std::str;
use std::num;
use std::num::strconv;

pub fn get_prompt(status: isize) -> String {
    let cwd = match os::getcwd() {
        Ok(d) => {d},
        Err(f) => {panic!(f.to_string())},
    };
    let dir = match cwd.filename() {
        Some(d) => {d},
        None => {b"/"},
    };
    let dispdir = str::from_utf8(dir).unwrap_or("Could not find directory");
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
    if status != 0 {
        pro.push('(');
        pro.push_str(dispstat.as_slice());
        pro.push(')');
        pro.push(' ');
    }
    pro.push_str(dispdir);
    pro.push_str(" $ ");
    return pro;
}

#[test]
fn status_is_zero() {
    assert!("src $ ", get_prompt(0));
}

#[test]
fn status_non_zero() {
    assert!("(101) src $ ", get_prompt(101));
}
