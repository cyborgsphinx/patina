use std::io;
use std::io::fs;
use std::os;
use std::io::process::Command;
use std::path::Path;

fn main() {
    println!("Hash Shell\nPrealpha");

    let mut cwd = match os::getcwd(){
        Ok(p) => {p},
        Err(f) => {panic!(f.to_string())},
    };

    loop {
        print!("{} $ ", cwd.as_str().unwrap());
        let input = match io::stdio::stdin().read_line() {
            Ok(c) => {c},
            Err(f) => {panic!(f.to_string())},
        };
        if input.trim() == "" {continue}
        let opt: Vec<&str> = input.trim().split_str(" ").collect();
        let (cmd, args) = (opt[0], opt.slice(1, opt.len()));

        match cmd {
            "exit" => {break},
            "cd" => {       // does not actually change the directory (yet)
                if args.is_empty() {    // cd called without arguments; cd ~
                    let home = match os::homedir() {
                        Some(p) => {p},
                        None => {panic!("You have no home")},   //should probably change that panic
                    };
                    os::change_dir(&home);
                } else {
                    //TODO implement flags
                    let dir = args[0];
                    match os::change_dir(&Path::new(dir)) {
                        Ok(_) => {},
                        Err(f) => {println!("cd: the directory \"{}\" does not exist", dir)}
                    };
                }
            },
            _ => {
                if fork(args) {
                    let process = Command::new(cmd).cwd(&cwd).args(args.slice_to(args.len()-1)).output();

                    match process {
                        Ok(out) => {
                            println!("{}", String::from_utf8_lossy(out.output.as_slice()));
                        },
                        Err(f) => {
                            println!("Error: {}", f);
                        },
                    };
                } else {
                    let process = Command::new(cmd).cwd(&cwd).args(args).output();
                    match process {
                        Ok(out) => {
                            println!("{}", String::from_utf8_lossy(out.output.as_slice()));
                        },
                        Err(f) => {
                            println!("Error: {}", f);
                        },
                    };
                }
            },
        };
    }
}

fn fork(opts: &[&str]) -> bool{
    if opts.len() > 0 {
        let test = match opts.last() {
            Some(st) => {*st},
            None => {panic!("Oh fuck")},
        };
        return test == "&";
    } else {
        return false;
    }
}
