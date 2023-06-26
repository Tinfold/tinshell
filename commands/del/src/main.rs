use std::env;
use std::env::Args;
use std::fs;
use std::path::Path;

use colored::Colorize;

fn delete(v: String) {
    let dir: &str = v.as_str();
    let path = Path::new(dir);
    if path.is_dir() {
        match fs::remove_dir_all(path) {
            Ok(_v) => {
                println!(
                    "{}",
                    "Successfully deleted directory ".to_string()
                        + dir.truecolor(69, 77, 102).to_string().as_str()
                )
            }
            Err(e) => {
                println!("{}", e)
            }
        };
    } else {
        // TO:DO add colors based on extension
        match fs::remove_file(path) {
            Ok(_v) => {
                println!("{}", "Successfully deleted file ".to_string() + dir)
            }
            Err(e) => {
                println!("{}", e)
            }
        }
    }
}

fn del(mut args: Args) {
    args.next();
    for arg in args {
        delete(arg);
    }
}

fn main() {
    del(env::args());
}
