#[allow(unused_imports)]
use std::io::{self, Write};

enum Command {
    Echo,
    Type,
    Exit,
}

impl Command {
    fn from_str(input: &str) -> Option<Command> {
        match input.trim().to_lowercase().as_str() {
            "echo" => Some(Command::Echo),
            "type" => Some(Command::Type),
            "exit" => Some(Command::Exit),
            _ => None,
        }
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit 0" {
            break;
        }

        let mut input_array = input.trim().split_whitespace();

        let command = input_array.next();
        let arguments = input_array.collect::<Vec<&str>>().join(" ");

        if command == Some("echo") {
            println!("{}", arguments)
        } else if command == Some("type") {
            match Command::from_str(&arguments) {
                Some(command) => println!("{} is a shell builtin", arguments),
                None => println!("{}: not found", arguments),
            }
        } else {
            println!("{}: command not found", input.trim());
        }
    }
}
