use std::env;
use std::path::Path;

// when tilde is passed, it is seen as home
fn expand_tilde(path: &str) -> String {
    if path.starts_with('~') {
        let home = env::var("HOME").unwrap_or_else(|_| String::from("home/"));
        format!("{}{}", home, &path[1..])
    } else {
        path.to_string()
    }
}

pub fn cd(path: String) {
    let new_path = expand_tilde(&path);
    let new_dir_path = Path::new(&new_path);

    match env::set_current_dir(&new_dir_path) {
        Ok(_) => {
            // do nothing when Ok
        },
        Err(_) => eprintln!("cd: {}: No such file or directory", new_path),
    }
}
