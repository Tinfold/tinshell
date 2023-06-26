use shlex::Shlex;
use std::io::{self, stdout, Write};
pub fn test(args: Shlex) {
    let mut stdout = io::stdout().lock();
    match stdout.write_all(b"hello world") {
        Ok(_v) => {}
        Err(_e) => {}
    };
}
