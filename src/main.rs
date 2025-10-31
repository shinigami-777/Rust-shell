#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

mod navigation;

// takes filename input and checks if a file with that name present in PATH
// returns array of all such files
fn check_if_file_present(file: &str)-> Vec<PathBuf> {
    let mut executables = Vec::new();

    // see if the path variable is present 
    if let Ok(path_var) = env::var("PATH") {
        for dir in path_var.split(':') {
            let path = Path::new(dir).join(file);

            if let Ok(metadata) = fs::metadata(&path) {
                if metadata.is_file() {
                    let mode = metadata.permissions().mode();
                    // check for permissions
                    if mode & 0o111 != 0 {
                        executables.push(path);
                    }
                }
            }
        }
    }

    executables
}

fn main() {

    let builtin_commands = ["exit", "echo", "type", "pwd"];
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("Failed to read line");
        // Trimming the newline
        user_input = user_input.trim().to_string();

        let input: Vec<&str> = user_input.split(' ').collect();
        if input.len()==0 {
            println!("{}: command not found", user_input);
        }
        // exit shell when exit command is used
        if input[0]=="exit" {
            break;
        }
        // echo command
        else if input[0]=="echo" {
            println!("{}", &user_input[5..]);
        }
        // pwd command
        else if input[0]=="pwd" {
            if let Err(e) = navigation::pwd() {
            eprintln!("Error: {}", e);
        }
        }
        // type command
        else if input[0]=="type" {
            if builtin_commands.contains(&input[1]) {
                println!("{} is a shell builtin", &input[1]);
            }
            else {
                let location = check_if_file_present(&input[1]);

                if location.is_empty() {
                    println!("{}: not found", &input[1]);
                }
                else {
                    let path = &location[0];
                    println!("{} is {}", &input[1], path.display());
                }
            }
        }
        
        // if it is not type or in-built command
        else {
            let location = check_if_file_present(&input[0]);
            if location.is_empty() {
                    println!("{}: command not found", user_input);
            }
            else {
                //let path = location[0].to_string_lossy().to_string();
                let args = &input[1..];
                let output = Command::new(&input[0])
                    .args(&*args)
                    .output()
                    .expect("failed to execute command");

                // print the output or error
                print!("{}", String::from_utf8_lossy(&output.stdout));
                eprint!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
    }

}
