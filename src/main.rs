#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {

    let valid_commands = ["exit", "echo", "type"];
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
        // type command
        else if input[0]=="type" {
            if valid_commands.contains(&input[1]) {
                println!("{} is a shell builtin", &input[1]);
            }
            else {
            println!("{}: not found", &input[1]);
            }
        }
        else {
            println!("{}: command not found", user_input);
        }
    }

}
