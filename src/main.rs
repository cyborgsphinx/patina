extern crate patina;
extern crate glob;
extern crate copperline;
extern crate nix;

use std::process;
use std::env;
use std::convert::{From, AsRef};

use copperline::{Copperline, Error};

use nix::sys::signal::{SIGINT, SIGQUIT, SIGTSTP, SIGTTIN, SIGTTOU, SIGCHLD};
use nix::sys::signal::{SigHandler, SigAction, SigSet};
use nix::sys::signal::SaFlag;

use patina::{execute, prompt, cd};

#[cfg(not(test))]
fn main() {
    let home = match env::home_dir() {
        Some(d) => d,
        None => {
            println!("Cannot find environment variable: home");
            return;
        }
    };
    println!("Patina Shell\nPrealpha");

    extern "C" fn handle_signal(_signo: i32) {
        println!("Caught a signal");
    }

    let action = SigAction::new(SigHandler::Handler(handle_signal), SaFlag::empty(), SigSet::empty());
    unsafe { // signals to be ignored
        let _ = nix::sys::signal::sigaction(SIGINT, &action);
        let _ = nix::sys::signal::sigaction(SIGQUIT, &action);
        let _ = nix::sys::signal::sigaction(SIGTSTP, &action);
        let _ = nix::sys::signal::sigaction(SIGTTIN, &action);
        let _ = nix::sys::signal::sigaction(SIGTTOU, &action);
        let _ = nix::sys::signal::sigaction(SIGCHLD, &action);
    }

    let mut locals: Vec<(String, String)> = Vec::new();
    let mut copper = Copperline::new();

    let mut stat = 0;

    loop {
        let input: String;
        match copper.read_line_ascii(prompt::get_prompt(stat as isize).as_ref()){
            Ok(st) => input = st,
            Err(Error::EndOfFile) => break,
            Err(Error::UnsupportedTerm) => {println!("Error: Unsupporter terminal"); break;},
            Err(Error::Cancel) => {println!("Error: Cancel encountered"); break;},
            Err(Error::ErrNo(n)) => {println!("Error {:?}: failing", n); break;},
        }

        if input.trim() == "" {continue}
        let mut args: Vec<&str> = input.trim().split_whitespace().collect();
        let cmd = args.remove(0); // take args[0] out, put it in cmd, move eveything in args left

        match cmd.as_ref() {
            "exit" => {break},
            // leaving echo to system for now
            /*"echo" => {
                match args[0] {
                    "-n" => {
                        for st in args[1..].iter() {
                            print!("{} ", st);
                        }
                    },
                    _ => {
                        for st in args {
                            print!("{} ", st);
                        }
                        print!("\n");
                    },
                };
                stat = 0;
            },*/
            "set" => { //not functional
                match args[0].as_ref() {
                    "-x" => {
                        let (key, value) = (args[1].to_string(), args[2].to_string());
                        env::set_var(&key, &value);
                    },
                    "-u" => {
                        env::remove_var(&args[1]);
                    },
                    "-e" => {
                        let mut i: usize = 0;
                        while i < locals.len() {
                            if locals[i].0 == args[1] {
                                locals.remove(i);
                                break;
                            }
                            i += 1;
                        }
                    },
                    _ => {
                        let (key, value) = (args[0].to_string(), args[1].to_string());
                        locals.push((key, value));
                    },
                };
                stat = 0;
            },
            "cd" => {
                if args.is_empty() {    //cd called alone; equivalent to cd ~
                    stat = cd::ch_dir(&home);
                } else {
                    stat = cd::ch_dir(&From::from(args[0]));
                }
            },
            "clear" => {
                match copper.clear_screen() {
                    Ok(..) => stat = 0,
                    Err(..) => stat = 1,
                };
            },
            "fg" => {//not functional
                let _ = nix::sys::signal::kill(args[0].parse::<i32>().unwrap(), 25);
            },
            "bg" => {//not functional
                let _ = nix::sys::signal::kill(args[0].parse::<i32>().unwrap(), 25);
            },
            _ => {//no forking yet
                stat = execute::run(cmd, args);
            },
        };
    }
    process::exit(stat);
}
