extern crate libc;

use self::libc::funcs::posix01::signal;
use self::libc::types::os::common::posix01;

pub fn catch_signal(signo: isize) { // not quite working
    print!("caught the thing\n");
}
