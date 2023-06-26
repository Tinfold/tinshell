use std::path::Path;

use get_dir;
use set_dir;
use Shlex;

pub fn cd(mut args: Shlex) {
    let temp = args.next(); //.unwrap();
    match temp {
        Some(v) => {
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
