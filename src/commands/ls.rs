use std::fs;

use Shlex;

use colored::Colorize;

use crate::shared::get_dir;

pub fn ls(_args: Shlex) {
    // Check if we have permission to read the directory
    let dir = get_dir();
    let perms = fs::read_dir(dir);
    match perms {
        Ok(paths) => {
            print!("{}", "\n\n");
            let mut counter = 0;
            for path in paths {
                if counter == 7 {
                    // need to export this as a variable somewhere
                    counter = 0;
                    print!("{}", "\n");
                }
                let is_dir = path.as_ref().unwrap().path().is_dir();
                match path.unwrap().path().file_name() {
                    None => println!("{}", "INVALID_DIR"),
                    Some(obj) => {
                        if is_dir == true {
                            print!(" {} ", obj.to_str().unwrap().truecolor(69, 77, 102))
                        } else {
                            print!(" {} ", obj.to_str().unwrap())
                        }
                    }
                }
                counter += 1;
            }
        }
        Err(e) => println!("{}", e),
    }
    print!("{}", "\n");
}
