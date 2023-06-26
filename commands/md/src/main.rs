use std::env;
use std::env::Args;
use std::fs::DirBuilder;

fn md(mut args: Args) {
    args.next();
    let path = args.next();
    DirBuilder::new()
        .recursive(true)
        .create(path.unwrap())
        .unwrap();
}

fn main() {
    md(env::args());
}
