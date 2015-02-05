use std::os;
use std::old_path::Path;
use std::old_io::process::Command;
use std::string::String;

// really just a wrapper around os::change_dir()
// returns 1 for success and 0 for failure
pub fn ch_dir(dest: Path) -> usize {
    match os::change_dir(&dest) {
        Ok(_) => 1,
        Err(_) => {
            println!("Failed changing directory");
            0},
    }
}

pub fn main() {
    let dir = Path::new("..");
    let num = ch_dir(dir);
    println!("{}", num);
    let out = match Command::new("pwd").output() {
        Ok(p) => {p},
        Err(_) => {panic!("Well fuck")},
    };
    println!("{}", String::from_utf8_lossy(out.output.as_slice()));
}
