use std::os;
use std::path::Path;
use std::io::process::Command;
use std::string::String;

// really just a wrapper around os::change_dir()
// returns 1 for success and 0 for failure
pub fn ch_dir(dest: Path) -> usize {
    let res = match os::change_dir(&dest) {
        Ok(d) => {1},
        Err(f) => {println!("Failed changing directory");0},
    };
    return res;
}

pub fn main() {
    let dir = Path::new("..");
    let num = ch_dir(dir);
    println!("{}", num);
    let mut out = match Command::new("ls").output() {
        Ok(p) => {p},
        Err(f) => {panic!("Well fuck")},
    };
    println!("{}", String::from_utf8_lossy(out.output.as_slice()));
}
