use std::io;
use std::io::fs;
use std::os;
use std::io::process::{Command, ProcessExit};
use std::path::Path;
use std::str;

mod prompt;
mod cd;
mod parse;
mod echo;

fn main() {
    println!("Hash Shell\nPrealpha");

//    let mut (sin, sout, serr) = (io::stdin, io::stdout, io::stderr);  //for when I want to work
//    with stdio, especially redirect

   let mut stat = os::get_exit_status();

    loop {
        let mut cwd = match os::getcwd(){
            Ok(p) => {p},
            Err(f) => {panic!(f.to_string())},
        };
        print!("{} ", prompt::get_prompt(stat));
        let input = match io::stdio::stdin().read_line() {
            Ok(c) => {c},
            Err(f) => {panic!(f.to_string())},
        };
        if input.trim() == "" {continue}
        let opt: Vec<&str> = input.trim().split_str(" ").collect();
        let (cmd, args) = (opt[0], opt.slice(1, opt.len()));

        match cmd {
            "exit" => {break},
            "echo" => {
                echo::put(echo::parse(args));   //TODO expand and improve
            },
            "cd" => {       //changes the directory the shell shows, but nothing more
                //TODO implement flags
                let mut chdir: Path;
                if args.is_empty() {    //cd called alone; equivalent to cd ~
                    chdir = match os::homedir() {
                        Some(d) => {d},
                        None => {panic!("You have no home")},   //TODO improve
                    };
                } else {
                    chdir = Path::new(parse::path(args[0].to_string()));
                }
                cd::ch_dir(chdir);
            },
            _ => {  // I have no idea what the fuck to do here
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
                    let process = Command::new(cmd).cwd(&cwd).args(args).spawn();
                    match process {
                        Ok(stream) => {
                            let out = stream.wait_with_output().unwrap();
                            let pout = String::from_utf8(out.output).unwrap_or("Fuck".to_string());
                            let perr = String::from_utf8(out.error).unwrap_or("Fuck".to_string());
                            if !pout.is_empty() {
                                print!("{}", pout);
                            }
                            if !perr.is_empty() {
                                print!("{}", perr);
                            }
                        },
                        Err(f) => {
                            println!("Error: {}", f);
                        },
                    };
                }
            },
        };
    }
    //not a fan of exiting with the same status as last-run command
    os::set_exit_status(0);
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
