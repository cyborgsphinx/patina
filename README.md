#Patina - A Rust Shell

A shell written in rust with design ideas from fish and bash

Nowhere near feature complete

##Use

With the most recent rust, run `cargo run`. If you are likely to use signals like ctrl-c, run `./test`. This is because cargo doesn't seem to pass signal handling off to the program it's running.

##Features

- Fish-like syntax (not trying to be bash/POSIX compatible)
- Rudimentary tab completion

More to come

##Built-in Commands

- cd
- clear
- exit

More to come

##Building and Running
Running `cargo build` should pull in any needed dependencies, including the line editor bindings. However, you will need the library itself installed.

It has been found that `cargo run` can interfere with signals, so if you are running something and wish to cancel or suspend it, it is recommended to run the binary directly after a build. For such cases, the test script works. But if you feel confident that you won't need use of signals, `cargo run` will work fine.

##TODO

Finish working on shell built-ins
- fg
- bg
- jobs
- set (fish-like)
- others as they are thought of

Colours

Globbing
