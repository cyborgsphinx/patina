extern crate patina;
extern crate glob;
extern crate copperline;
extern crate nix;

#[cfg(not(test))]
use std::collections::HashMap;
#[cfg(not(test))]
use std::convert::From;
#[cfg(not(test))]
use std::env;
#[cfg(not(test))]
use std::process;

#[cfg(not(test))]
use copperline::{Copperline, Error};

#[cfg(not(test))]
use nix::sys::signal::{SIGINT, SIGQUIT, SIGTSTP, SIGTTIN, SIGTTOU, SIGCHLD};
#[cfg(not(test))]
use nix::sys::signal::{SigHandler, SigAction, SigSet};
#[cfg(not(test))]
use nix::sys::signal::SaFlag;

#[cfg(not(test))]
use patina::{cd, execute, parse, prompt};

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

    let action = SigAction::new(SigHandler::Handler(handle_signal),
                                SaFlag::empty(),
                                SigSet::empty());
    unsafe {
        // signals to be ignored
        let _ = nix::sys::signal::sigaction(SIGINT, &action);
        let _ = nix::sys::signal::sigaction(SIGQUIT, &action);
        let _ = nix::sys::signal::sigaction(SIGTSTP, &action);
        let _ = nix::sys::signal::sigaction(SIGTTIN, &action);
        let _ = nix::sys::signal::sigaction(SIGTTOU, &action);
        let _ = nix::sys::signal::sigaction(SIGCHLD, &action);
    }

    let mut locals: HashMap<String, String> = HashMap::new();
    let mut copper = Copperline::new();

    let mut stat = 0;

    loop {
        let input: String;
        match copper.read_line_ascii(prompt::get_prompt(stat as isize).as_str()) {
            Ok(st) => input = st,
            Err(Error::EndOfFile) => break,
            Err(Error::UnsupportedTerm) => {
                println!("Error: Unsupporter terminal");
                break;
            }
            Err(Error::Cancel) => {
                println!("Error: Cancel encountered");
                break;
            }
            Err(Error::ErrNo(n)) => {
                println!("Error {:?}: failing", n);
                break;
            }
        }

        if input.trim() == "" {
            continue;
        }
        let mut args = parse::line(input, &locals);
        let cmd = args.remove(0); // take args[0] out, put it in cmd, move eveything in args left

        match cmd.as_str() {
            "exit" => break,
            // leaving echo to system for now
            // "echo" => {
            // },
            "set" => {
                // not functional
                match args[0].as_str() {
                    "-x" => {
                        let (key, value) = (args[1].to_string(), args[2].to_string());
                        env::set_var(&key, &value);
                    }
                    "-u" => {
                        env::remove_var(&args[1]);
                    }
                    "-e" => {
                        let _ = locals.remove(&args[0].to_string());
                    }
                    _ => {
                        let (key, value) = (args[0].to_string(), args[1].to_string());
                        locals.insert(key, value);
                    }
                };
                stat = 0;
            }
            "cd" => {
                if args.is_empty() {
                    // cd called alone; equivalent to cd ~
                    stat = cd::ch_dir(&home);
                } else {
                    stat = cd::ch_dir(&From::from(&args[0]));
                }
            }
            "clear" => {
                match copper.clear_screen() {
                    Ok(..) => stat = 0,
                    Err(..) => stat = 1,
                };
            }
            "fg" => {
                // not functional
                let _ = nix::sys::signal::kill(args[0].parse::<i32>().unwrap(), 25);
            }
            "bg" => {
                // not functional
                let _ = nix::sys::signal::kill(args[0].parse::<i32>().unwrap(), 25);
            }
            _ => {
                // no forking yet
                let args = args.iter().map(|x| x.as_str()).collect();
                stat = execute::run(cmd.as_str(), args);
            }
        };
    }
    process::exit(stat);
}
