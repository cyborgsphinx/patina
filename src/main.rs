extern crate patina;
extern crate glob;
extern crate copperline;
extern crate nix;

use std::process;
use std::path::PathBuf;
use std::env;
use std::convert::{From, AsRef};

use copperline::{Copperline, Error};

use nix::sys::signal::{SIGINT, SIGQUIT, SIGSTOP, SIGTTIN, SIGTTOU, SIGCHLD};
use nix::sys::signal::{SigHandler, SigAction, SigSet};
use nix::sys::signal::SaFlag;

use patina::{execute, prompt, cd};

#[cfg(not(test))]
fn main() {
    println!("Patina Shell\nPrealpha");

    let action = SigAction::new(SigHandler::SigDfl, SaFlag::empty(), SigSet::empty());
    unsafe { // signals to be ignored
        let _ = nix::sys::signal::sigaction(SIGINT, &action);
        let _ = nix::sys::signal::sigaction(SIGQUIT, &action);
        let _ = nix::sys::signal::sigaction(SIGSTOP, &action);
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
            "echo" => { //work on this
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
            },
            "set" => {
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
                //TODO implement flags
                let chdir: PathBuf;
                if args.is_empty() {    //cd called alone; equivalent to cd ~
                    chdir = match env::home_dir() {
                        Some(d) => {d},
                        None => {panic!("You have no home")},   //TODO improve
                    };
                } else {
                    chdir = From::from(args[0]);
                }
                stat = cd::ch_dir(chdir);
            },
            "clear" => {
                match copper.clear_screen() {
                    Ok(..) => stat = 0,
                    Err(..) => stat = 1,
                };
            },
            "fg" => {//not functional
                //process::Process::kill(args[0].parse::<i32>().unwrap(), 25);//SIGCONT == 25
                let _ = nix::sys::signal::kill(args[0].parse::<i32>().unwrap(), 25);
            },
            "bg" => {//not functional
                //process::Process::kill(args[0].parse::<i32>().unwrap(), 25);//pid_t == i32
                let _ = nix::sys::signal::kill(args[0].parse::<i32>().unwrap(), 25);
            },
            _ => {//no forking yet
                stat = execute::run(cmd, args);
            },
        };
    }
    process::exit(stat);
}
