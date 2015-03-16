use std::string::String;
use std::slice::SliceExt;

struct Executable {
    cmd: &str,
    args: &[&str],
    prev: char,
}

impl Executable {
    fn new(st: String) -> Executable {
        let out: &[&str] = st.trim().words().collect();
        let (cmd1, args1) = out.split_at(1);
        let exe = Executable { cmd: cmd1, args: args1 };
    }
}
