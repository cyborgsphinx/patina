#![feature(collections, core, exit_status, libc, std_misc, str_words)]

extern crate rustecla;
extern crate libc;

use std::process::Command;
use std::os::unix::process::ExitStatusExt;
use std::path::PathBuf;
use std::env;
//use std::str::StrExt;

mod prompt;
mod cd;
mod parse;
mod signals;

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
    let (line_length, hist_size) = (1024u64, 2048u64);
    let gl = rustecla::new_gl(line_length, hist_size);

    rustecla::load_history(gl, "~/.patina_history", "Load history");

    loop {
        let stat = env::get_exit_status();

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
                            i += 1;
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
                //let process = Command::new(cmd).args(args.as_slice()).stdin(InheritFd(0)).stdout(InheritFd(1)).stderr(InheritFd(2)).status();//old_io command
                let process = Command::new(cmd).args(args.as_slice()).status();
                match process {
                    Ok(val) => {
                        match val.code() {
                            Some(num) => env::set_exit_status(num),
                            None => match val.signal() {
                                Some(num) => env::set_exit_status(num*-1),//tell me the signal
                                None => env::set_exit_status(-1),
                            },
                        };
                    },
                    Err(..) => {
                        println!("patina: command not found: {}", cmd);
                        env::set_exit_status(127);
                    },
                };
                 //   println!("patina: command not found: {}", cmd);
                   // env::set_exit_status(127);
                //} else {// mabe toss in some match statements
                 //   env::set_exit_status(process.unwrap().code().unwrap());
                //}
            },
        };
    }
    rustecla::save_history(gl, "~/.patina_history", "Saved command:", 2048);
}
