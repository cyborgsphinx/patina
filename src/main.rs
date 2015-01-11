use std::io;
use std::io::fs;
use std::os;
use std::io::process::Command;
use std::path::Path;
use std::str;

fn main() {
    println!("Hash Shell\nPrealpha");

//    let mut (sin, sout, serr) = (io::stdin, io::stdout, io::stderr);  //for when I want to work
//    with stdio, especially redirect

    let mut cwd = match os::getcwd(){
        Ok(p) => {p},
        Err(f) => {panic!(f.to_string())},
    };
    let mut stat = os::get_exit_status();

    loop {
        let mut dir = match cwd.filename() {
            Some(d) => {d},
            None => {b"/"},
        };
        let mut dispdir = str::from_utf8(dir).unwrap_or("Could not find directory");
        print!("({}){} $ ", stat, dispdir);
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
                    let chdir = args[0];
                    match os::change_dir(&Path::new(chdir)) {
                        Ok(_) => {},
                        Err(f) => {println!("cd: the directory \"{}\" does not exist", chdir)}
                    };
                }
            },
            _ => {
                if fork(args) {
                    let process = Command::new(cmd).cwd(&cwd).args(args.slice_to(args.len()-1)).output();

                    match process {
                        Ok(out) => {
                            print!("{}", String::from_utf8_lossy(out.output.as_slice()));
                        },
                        Err(f) => {
                            println!("Error: {}", f);
                        },
                    };
                } else {
                    let process = Command::new(cmd).cwd(&cwd).args(args).output();
                    match process {
                        Ok(out) => {
                            print!("{}", String::from_utf8_lossy(out.output.as_slice()));
                            stat = out.status;
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
