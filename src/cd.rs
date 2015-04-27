use std::path::PathBuf;
use std::env;

// really just a wrapper around os::change_dir()
// returns 0 for success and 1 for failure
pub fn ch_dir(dest: PathBuf) -> i32 {
    match env::set_current_dir(&dest) {
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
    use std::path::Path;

    #[test]
    fn test_ok() {
        let dir = Path::new("/tmp");
        let num = ch_dir(dir.to_path_buf());
        assert_eq!(num, 0);
    }

    #[test]
    fn test_err() {
        let dir = Path::new("/tup");
        let num = ch_dir(dir.to_path_buf());
        assert_eq!(num, 1);
    }
}
