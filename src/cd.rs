use std::os;
use std::path::PathBuf;
use std::old_path;// would prefer os::change_dir() to not need this

// really just a wrapper around os::change_dir()
// returns 0 for success and 1 for failure
pub fn ch_dir(dest: PathBuf) -> i32 {
    let st = match dest.to_str() {
        Some(val) => val,
        None => "",
    };
    let dir = old_path::Path::new(st);
    match os::change_dir(&dir) {
        Ok(..) => 0,
        Err(..) => {
            println!("Failed changing directory");
            1
        },
    }
}

#[cfg(test)]
mod tests {
    use cd::ch_dir;
    use std::path::PathBuf;

    #[test]
    fn test_ok() {
        let dir = PathBuf::new("/tmp");
        let num = ch_dir(dir);
        assert_eq!(num, 0);
    }

    #[test]
    fn test_err() {
        let dir = PathBuf::new("/tup");
        let num = ch_dir(dir);
        assert_eq!(num, 1);
    }
}
