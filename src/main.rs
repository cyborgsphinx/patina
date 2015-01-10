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
        print!("{} $", cwd.as_str().unwrap());
        let input = match io::stdio::stdin().read_line() {
            Ok(c) => {c},
            Err(f) => {panic!(f.to_string())},
        };
        if input.trim() == "" {continue}
        let opt: Vec<&str> = input.trim().split_str(" ").collect();
        let (cmd, args) = (opt[0], opt.slice(1, opt.len()));

        match cmd {
            "exit" => {break},
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
    let test = match opts.last() {
        Some(st) => {st},
        None => {panic!("Oh fuck")},
    };
    return test == &"&";
}
