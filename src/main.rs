extern crate linenoise;
extern crate libc;

use std::old_io::fs;
use std::os;
use std::old_io::process::{Command, ProcessExit};
use std::old_path::Path;
use std::str;
use std::old_io::process::StdioContainer::InheritFd;

mod prompt;
mod cd;
mod parse;
mod echo;
mod complete;
mod signals;

fn main() {
    println!("Patina Shell\nPrealpha");

    unsafe {
        self::libc::funcs::posix01::signal::signal(self::libc::consts::os::posix88::SIGINT,
                                             signals::catch_signal as u64);
    }

//    let mut (sin, sout, serr) = (io::stdin, io::stdout, io::stderr);  //for when I want to work
//    with stdio, especially redirect

    linenoise::history_load("~/.patina_history");
    let mut stat = os::get_exit_status();

    linenoise::set_callback(complete::complete);
    loop {
        let mut stat = os::get_exit_status();
        let mut cwd = match os::getcwd(){
            Ok(p) => {p},
            Err(f) => {panic!(f.to_string())},
        };
        let input = match linenoise::prompt(prompt::get_prompt(stat).as_slice()) {
            Some(st) => st,
            None => "Input not parsed".to_string(),
        };
        if input == "Input not parsed".to_string() {
            print!("Input not parsed");
            continue;
        }
        if input.trim() == "" {continue}
        linenoise::history_add(input.as_slice());
        let opt: Vec<&str> = input.trim().words().collect();
        let (cmd, args) = (opt[0], opt.slice_from(1));

        match cmd.as_slice() {
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
                    chdir = Path::new(parse::path(args[0]));
                }
                cd::ch_dir(chdir);
            },
            "clear" => {
                linenoise::clear_screen();
            },
            _ => {  // I have no idea what the fuck to do here
                /*if fork(args) {
                    let process = Command::new(cmd).cwd(&cwd).args(args.slice_to(args.len()-1)).output();

                    match process {
                        Ok(out) => {
                            print!("{}", String::from_utf8_lossy(out.output.as_slice()));
                        },
                        Err(f) => {
                            println!("Error: {}", f);
                        },
                    };
                } else {*/  // not ready for forking yet
                let process = Command::new(cmd).args(args).stdin(InheritFd(0)).stdout(InheritFd(1)).stderr(InheritFd(2)).spawn();
                /*match process {
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
                };*/
                //} the matching brace for else
            },
        };
    }
    linenoise::history_save("~/.patina_history");
    //not a fan of exiting with the same status as last-run command
    os::set_exit_status(0);
}
/*      // not ready for prime time yet
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
}*/
