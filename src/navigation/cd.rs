use std::env;
use std::path::Path;

pub fn cd(new_path: String) {
    let new_dir_path = Path::new(&new_path);

    match env::set_current_dir(&new_dir_path) {
        Ok(_) => {
            // do nothing when Ok
        },
        Err(e) => eprintln!("cd: {}: No such file or directory", new_path),
    }
}
