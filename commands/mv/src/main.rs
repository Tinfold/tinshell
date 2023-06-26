extern crate path_slash;
use path_slash::PathExt;
use shared::get_dir;
use std::env;
use std::env::Args;
use std::fs;
use std::path::Path;

pub fn mv(mut args: Args) {
    args.next();
    let temp = args.next(); //.unwrap();
    match temp {
        Some(v) => {
            let source: &str = v.as_str(); //.map_or("/", |x| x);
                                           // default to '/' as new directory if one was not provided
            let temp = args.next().unwrap_or("".to_string());
            println!("{}", temp);
            let dest: &str = temp.as_str();
            let path = Path::new(dest);

            let gd = &(get_dir() + "/");
            let source_path = Path::new(gd).to_slash();
            let unwrapped: &str = &source_path.unwrap();
            println!("{}", unwrapped);
            //let parent=source_path.parent();
            if path.is_absolute() {
                match fs::rename(source, path.to_str().unwrap().to_string() + "/" + source) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("{}", e)
                    }
                };
            } else {
                match fs::rename(source, unwrapped.to_string() + "/" + dest + "/" + source) {
                    Ok(_) => {
                        // println!("{}", get_dir())
                    }
                    Err(e) => {
                        println!("{}", e)
                    }
                };
            }
        }
        None => {}
    }
}

fn main() {
    mv(env::args());
}
