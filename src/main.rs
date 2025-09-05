#[allow(unused_imports)]
use std::io::{self, Write};

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

        if input_array.next() == Some("echo") {
            let text = input_array.collect::<Vec<&str>>().join(" ");
            println!("{}", text)
        } else {
            println!("{}: command not found", input.trim());
        }
    }
}
