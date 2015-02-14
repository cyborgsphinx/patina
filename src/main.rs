extern crate linenoise;
extern crate libc;

use std::old_io::fs;
use std::os;
use std::old_io::process;
use std::old_io::process::{Command, ProcessExit};
use std::old_path::Path;
use std::str;
use std::old_io::process::StdioContainer::InheritFd;
use std::env;

mod prompt;
mod cd;
mod parse;
mod echo;
mod complete;
mod signals;

fn main() {
    println!("Patina Shell\nPrealpha");

    unsafe { // signals to be ignored
        self::libc::funcs::posix01::signal::signal(self::libc::consts::os::posix88::SIGINT,
                                                signals::catch_signal as u64); //i'd prefer this
        self::libc::funcs::posix01::signal::signal(self::libc::consts::os::posix88::SIGQUIT,
                                                self::libc::consts::os::posix01::SIG_IGN);
        self::libc::funcs::posix01::signal::signal(20i32, //SIGTSTP
                                                self::libc::consts::os::posix01::SIG_IGN);
        self::libc::funcs::posix01::signal::signal(26i32, //SIGTTIN
                                                self::libc::consts::os::posix01::SIG_IGN);
        self::libc::funcs::posix01::signal::signal(27i32, //SIGTTOU
                                                self::libc::consts::os::posix01::SIG_IGN);
        self::libc::funcs::posix01::signal::signal(18i32, //SIGCHLD
                                                self::libc::consts::os::posix01::SIG_IGN);
    }   // c_int == i32

    linenoise::history_load("~/.patina_history");
    let mut stat = env::get_exit_status();

    linenoise::set_callback(complete::complete);
    loop {
        let cwd = match env::current_dir(){
            Ok(p) => {p},
            Err(f) => {panic!(f.to_string())},
        };
        let input = match linenoise::prompt(prompt::get_prompt(stat as isize).as_slice()) {
            Some(st) => st,
            None => "Input not parsed".to_string(),
        };
/*        if input == "Input not parsed".to_string() {
            print!("Input not parsed");
            continue;
        }*/
        if input.trim() == "" {continue}
        linenoise::history_add(input.as_slice());
        let opt: Vec<&str> = input.trim().words().collect();
        let (cmd, args) = (opt[0], opt.slice_from(1)); // &s[start..] syntax fails

        match cmd.as_slice() {
            "exit" => {break},
            "echo" => {
                echo::put(echo::parse(args));   //TODO expand and improve
            },
            "cd" => {
                //TODO implement flags
                let mut chdir: Path;
                if args.is_empty() {    //cd called alone; equivalent to cd ~
                    chdir = match env::home_dir() {
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
            "fg" => {
                process::Process::kill(args[0].parse::<i32>().unwrap(), 25);//SIGCONT == 25
            },
            "bg" => {
                process::Process::kill(args[0].parse::<i32>().unwrap(), 25);//pid_t == i32
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
    env::set_exit_status(0);
}
