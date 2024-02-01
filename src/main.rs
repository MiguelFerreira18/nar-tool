use std::fs::File;
use std::io::{BufWriter, Read, Write};
use std::process::Command;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: mut <command> <input>");
        return;
    }
    let command = std::env::args().nth(1).unwrap();
    let file_name = std::env::args().nth(2);
    match command.as_str() {
        "cf" => {
            if let Some(file_name) = file_name {
                if verify_length(&args, 2) {
                    println!("creating file with name: {}", file_name);
                    File::create(file_name).expect("Error creating file");
                }
            } else {
                println!("No target file name given");
            }
        }
        "cwa" => {
            if let Some(file_name) = file_name {
                if verify_length(&args, 3) {
                    println!("Creating webapp with the {}", file_name);
                    //checks wich manager the os has as package manager
                    let cli_tool_in_os = check_for_cli_tools();
                    if cli_tool_in_os.is_empty() {
                        println!("You should install either one of the following tools:");
                        show_package_managers();
                        return;
                    }
                    let command = format!("{} create vite {} -- --template {}", &cli_tool_in_os, file_name, args[3]);
                    execute_os_command(command.as_str());
                } else {
                    println!("No name or template was choosen for the project");
                    println!("mut wa <name of project> <template>");
                }
            }
        }
        "capi" => {
            if let Some(file_name) = file_name {
                if verify_length(&args, 3) {
                    println!("Creating API with the {}", file_name);
                    //checks wich manager the os has as a package manager
                }
            }
        }
        _ => {
            println!("Unknown command: {}", command);
        }
    }
}

fn verify_length(vector: &Vec<String>, length: usize) -> bool {
    if vector.len() == length {
        true
    } else {
        false
    }
}

fn execute_os_command(command: &str) {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", command])
            .output()
            .expect("Failed to execute the process")
    } else {
        Command::new("sh")
            .arg(command)
            .output()
            .expect("Failed to execute the process")
    };
    let message = String::from_utf8(output.stdout).expect("Failed to read output");
    println!("{}", message);
}

fn check_for_cli_tools() -> Box<str> {
    let cli_tools = vec!["npm", "yarn", "pnpm", "bunx"];
    let mut result = "".to_string();
    for name in cli_tools.iter() {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", format!("{} --version", name).as_str()])
                .output()
                .expect("Failed to execute command")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(format!("{} --version", name).as_str())
                .output()
                .expect("Failed to execute command")
        };

        if output.status.success() {
            result = name.to_string();
            break;
        }
    }
    result.into_boxed_str()
}

fn show_package_managers() {
    println!("npm");
    println!("yarn");
    println!("pnpm");
    println!("bunx");
}

