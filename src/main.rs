use std::env;
use std::fs;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;

enum ShellCommand {
    Echo,
    Type,
    Exit,
}

impl ShellCommand {
    fn from_str(input: &str) -> Option<ShellCommand> {
        match input.trim().to_lowercase().as_str() {
            "echo" => Some(ShellCommand::Echo),
            "type" => Some(ShellCommand::Type),
            "exit" => Some(ShellCommand::Exit),
            _ => None,
        }
    }
}

fn file_exists_and_executable(path: &PathBuf) -> bool {
    match path.try_exists() {
        Ok(true) => {
            if let Ok(metadata) = fs::metadata(&path) {
                let permissions = metadata.permissions();
                let mode = permissions.mode();

                if mode & 0o111 != 0 {
                    return true;
                }
            }
            return false;
        }
        Ok(false) => {
            return false;
        }
        Err(_) => {
            return false;
        }
    }
}

fn main() {
    let key = "PATH";
    let mut paths: Vec<String> = Vec::new();

    match env::var_os(key) {
        Some(val) => {
            if let Ok(path_string) = val.into_string() {
                paths = path_string
                    .split(":")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
            } else {
                println!("PATH contains invalid UTF-8");
            }
        }
        None => println!("{key} is not defined in the environment."),
    }

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
        let inputs_excl_command = input_array.collect::<Vec<&str>>();
        let arguments = inputs_excl_command.join(" ");

        if command == Some("echo") {
            println!("{}", arguments)
        } else if command == Some("type") {
            match ShellCommand::from_str(&arguments) {
                Some(_command) => println!("{} is a shell builtin", arguments),
                None => {
                    let mut command_found = false;
                    for path_dir in &paths {
                        let full_path = PathBuf::from(path_dir).join(&arguments);

                        if file_exists_and_executable(&full_path) {
                            println!("{} is {}", arguments, full_path.display());
                            command_found = true;
                            break;
                        }
                    }

                    if !command_found {
                        println!("{}: not found", arguments);
                    }
                }
            }
        } else {
            let mut command_found = false;
            for path_dir in &paths {
                let full_path = PathBuf::from(path_dir).join(command.unwrap());

                if file_exists_and_executable(&full_path) {
                    let mut cmd = Command::new(command.unwrap());
                    for arg in &inputs_excl_command {
                        cmd.arg(arg);
                    }

                    let output = cmd.output().expect("Failed to execute command");

                    let stdout = String::from_utf8_lossy(&output.stdout);
                    print!("{}", stdout);

                    command_found = true;
                    break;
                }
            }

            if !command_found {
                println!("{}: command not found", input.trim());
            }
        }
    }
}
