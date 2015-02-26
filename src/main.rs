extern crate rustecla;
extern crate libc;

use std::fs;
use std::os;
use std::old_io::process;
use std::old_io::process::{Command, ProcessExit};
use std::path::{Path, PathBuf};
use std::old_io::fs::PathExtensions;
use std::str;
use std::old_io::process::StdioContainer::InheritFd;
use std::env;
use std::fs::PathExt;
use std::str::StrExt;
use std::ffi::{CString, CStr};
use libc::{c_char, c_int};
use rustecla::WordCompletion;

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
                                                signals::catch_signal as u64);
        self::libc::funcs::posix01::signal::signal(20i32, //SIGTSTP
                                                signals::catch_signal as u64);
        self::libc::funcs::posix01::signal::signal(26i32, //SIGTTIN
                                                signals::catch_signal as u64);
        self::libc::funcs::posix01::signal::signal(27i32, //SIGTTOU
                                                signals::catch_signal as u64);
        self::libc::funcs::posix01::signal::signal(18i32, //SIGCHLD
                                                signals::catch_signal as u64);
    }   // c_int == i32

    let mut locals: Vec<(String, String)> = Vec::new();
    let (line_length, hist_size) = (1024u64, 2048u64);
    let mut gl = rustecla::new_gl(line_length, hist_size);

    rustecla::load_history(gl, "~/.patina_history", "Load history");

    loop {
        let stat = env::get_exit_status();
        let cwd = match env::current_dir(){
            Ok(p) => {p},
            Err(f) => {panic!(f.to_string())},//should probably not have that
        };

        let input = rustecla::get_line(gl, prompt::get_prompt(stat as isize).as_slice());

        if input.trim() == "" {continue}
        let mut args: Vec<&str> = input.trim().words().collect();
        let cmd = args.remove(0); // take args[0] out, put it in cmd, move eveything in args left

        match cmd.as_slice() {
            "exit" => {break},
            /*"echo" => { //work on this
                echo::put(echo::parse(args));   //TODO expand and improve
                env::set_exit_status(0);
            },*/
            "set" => {
                match args[0].as_slice() {
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
                        }
                    },
                    _ => {
                        let (key, value) = (args[0].to_string(), args[1].to_string());
                        locals.push((key, value));
                    },
                };
                env::set_exit_status(0);
            },
            "cd" => {
                //TODO implement flags
                let mut chdir: PathBuf;
                if args.is_empty() {    //cd called alone; equivalent to cd ~
                    chdir = match env::home_dir() {
                        Some(d) => {d},
                        None => {panic!("You have no home")},   //TODO improve
                    };
                } else {
                    chdir = PathBuf::new(parse::path(args[0]).as_slice());
                }
                env::set_exit_status(cd::ch_dir(chdir));
            },
            "clear" => {
                rustecla::clear(gl);
                env::set_exit_status(0);
            },
            "fg" => {//not functional
                process::Process::kill(args[0].parse::<i32>().unwrap(), 25);//SIGCONT == 25
            },
            "bg" => {//not functional
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
                let process = Command::new(cmd).args(args.as_slice()).stdin(InheritFd(0)).stdout(InheritFd(1)).stderr(InheritFd(2)).spawn();
                if process.is_err() {
                    println!("patina: command not found: {}", cmd);
                    env::set_exit_status(127);
                } else { //need to fix for arbitrary errors
                    env::set_exit_status(0);
                }
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
    rustecla::save_history(gl, "~/.patina_history", "Saved command:", 2048);
}
