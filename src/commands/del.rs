use std::fs;
use std::path::Path;

use Shlex;

use colored::Colorize;

pub fn del(mut args: Shlex) {
    let temp = args.next(); //.unwrap();
    match temp {
        Some(v) => {
            let dir: &str = v.as_str();
            let path = Path::new(dir);
            if path.is_dir() {
                match fs::remove_dir_all(path) {
                    Ok(_v) => {println!("{}", "Successfully deleted directory ".to_string() + dir )}
                    Err(e) => {println!("{}",e)}
                };
            }
            else{
                match fs::remove_file(path) {
                    Ok(_v) => {println!("{}", "Successfully deleted file ".to_string() + dir.truecolor(69, 77, 102).to_string().as_str())},
                    Err(e) => {println!("{}", e)}
                }
            }
        },
        None => {}
    }
}