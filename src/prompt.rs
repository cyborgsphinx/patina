use std::env;
use std::string::String;
use std::ffi::OsStr;
use std::path::PathBuf;

///Returns the prompt for the shell.
///Currently only returns a default based on the status of the previous process.
pub fn get_prompt(status: isize) -> String {
    let cwd = match env::current_dir() {
        Ok(d) => {d},
        Err(f) => {panic!(f.to_string())},
    };
    let home = match env::home_dir() {
        Some(p) => {p},
        None => {PathBuf::new()},
    };
    let dir = match cwd.file_name() {
        Some(d) => {d},
        None => {OsStr::new("/")},
    };
    let dispdir = match cwd == home {
        true => "~",
        false => dir.to_str().unwrap_or("dir not found"),
    };
    let mut pro = String::new();
    if status != 0 {
        pro.push_str(format!("({}) ", status).as_ref());
    }
    pro.push_str(format!("{} $ ", dispdir).as_ref());
    pro
}

#[cfg(test)]
mod tests {
    use prompt::get_prompt;
    use std::env;
    use std::string::String;
    use std::path;
    //make constant directory
    use std::path::Path;

    #[test]
    fn status_is_zero() {
        let root = Path::new("/tmp");
        assert!(env::set_current_dir(&root).is_ok());
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
        let root = Path::new("/tmp");
        assert!(env::set_current_dir(&root).is_ok());
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
