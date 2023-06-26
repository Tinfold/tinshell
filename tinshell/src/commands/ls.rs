use std::{ffi::OsStr, fs};

use Shlex;

use colored::Colorize;

use crate::shared::get_dir;

use std::collections::HashMap;

struct CustomColor {
    r: u8,
    g: u8,
    b: u8,
}

impl CustomColor {
    fn new(r: u8, g: u8, b: u8) -> CustomColor {
        CustomColor { r: r, g: g, b: b }
    }
}

pub fn ls(_args: Shlex) {
    let mut color_map = HashMap::new();
    color_map.insert("exe", CustomColor::new(48, 153, 117));
    // Check if we have permission to read the directory
    let dir = get_dir();
    let perms = fs::read_dir(dir);
    match perms {
        Ok(paths) => {
            print!("{}", "\n");
            let mut counter = 0;
            for path in paths {
                if counter == 7 {
                    // need to export this as a variable somewhere
                    counter = 0;
                    print!("{}", "\n");
                }
                let is_dir = path.as_ref().unwrap().path().is_dir();

                let pth = path.unwrap().path();
                match pth.file_name() {
                    None => println!("{}", "INVALID_DIR"),
                    Some(obj) => {
                        if is_dir == true {
                            print!("{}\t", obj.to_str().unwrap().truecolor(69, 77, 102))
                        } else {
                            let name = obj.to_str().unwrap();
                            let ext = pth.extension().unwrap_or(OsStr::new(""));
                            if color_map.contains_key(ext.to_str().unwrap()) {
                                let col = color_map.get(ext.to_str().unwrap()).unwrap();
                                print!("{}\t", name.truecolor(col.r, col.g, col.b));
                            } else {
                                print!("{}\t", name)
                            }
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
