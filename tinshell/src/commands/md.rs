use std::fs::DirBuilder;
use Shlex;

// TODO: Add support for creating directories using a path?
pub fn md(mut args: Shlex) {
    //let mut path_peekable = args.clone().peekable();
    let path = args.next(); //path_peekable.peek();
    DirBuilder::new()
        .recursive(true)
        .create(path.unwrap())
        .unwrap();
}
