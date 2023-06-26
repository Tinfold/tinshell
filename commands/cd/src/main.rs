use std::env::Args;
use std::path::Path;

use shared::get_dir;
use shared::set_dir;
//use shlex::Shlex;
use std::env;

fn cd(mut args: Args) {
    args.next();
    let temp = args.next(); //.unwrap();
    match temp {
        Some(v) => {
            println!("{}", v);
            let new_dir: &str = v.as_str(); //.map_or("/", |x| x);
                                            // default to '/' as new directory if one was not provided
            let root = Path::new(new_dir);
            if root.is_dir() {
                if (&root).is_absolute() == false {
                    set_dir((get_dir() + "/" + (root.to_str()).unwrap()).as_str());
                } else {
                    set_dir(root.to_str().unwrap())
                }
            }
        }
        None => {}
    }
}

fn main() {
    cd(env::args());
}
