extern crate libc;

use self::libc::funcs::posix01::signal;
use self::libc::types::os::common::posix01;

extern fn handler(signo: isize) {
    println!("Caught a signal"); // to be replaced with actual useful things
}

pub fn catch_signal(signo: isize) {
    unsafe {
        handler(signo);
    }
}
