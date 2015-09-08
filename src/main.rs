extern crate patina;
extern crate libc;
extern crate glob;
extern crate copperline;

use std::process;
use std::path::PathBuf;
use std::env;
use std::convert::From;
use std::convert::AsRef;

use copperline::{Copperline, Error};

use patina::{execute, prompt, cd, signals};

#[cfg(not(test))]
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
    let mut copper = Copperline::new();

    let mut stat = 0;
    loop {

        let input: String;
        match copper.read_line(prompt::get_prompt(stat as isize).as_ref()){
            Ok(st) => input = st,
            Err(Error::EndOfFile) => break,
            Err(Error::UnsupportedTerm) => {println!("Error: Unsupporter terminal"); break;},
            Err(Error::InvalidUTF8) => {println!("Can't handle invalid UTF8 yet"); break;},
            Err(Error::ErrNo(n)) => {println!("Error {:?}: failing", n); break;},
        }

        if input.trim() == "" {continue}
        let mut args: Vec<&str> = input.trim().split_whitespace().collect();
        let cmd = args.remove(0); // take args[0] out, put it in cmd, move eveything in args left

        match cmd.as_ref() {
            "exit" => {break},
            /*"echo" => { //work on this
                echo::put(echo::parse(args));   //TODO expand and improve
                env::set_exit_status(0);
            },*/
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
                //waiting for either copperline to deal with ^l or enough reason to connect to term library
                stat = 0;
            },
            "fg" => {//not functional
                //process::Process::kill(args[0].parse::<i32>().unwrap(), 25);//SIGCONT == 25
                unsafe {
                    self::libc::funcs::posix88::signal::kill(args[0].parse::<i32>().unwrap(), 25);
                }
            },
            "bg" => {//not functional
                //process::Process::kill(args[0].parse::<i32>().unwrap(), 25);//pid_t == i32
                unsafe {
                    self::libc::funcs::posix88::signal::kill(args[0].parse::<i32>().unwrap(), 25);
                }
            },
            _ => {//no forking yet
                stat = execute::run(cmd, args);
            },
        };
    }
    process::exit(stat);
}
