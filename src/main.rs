use std::fs;
use std::io::{BufWriter, Read, Write};
use std::process::Command;
use std::env;
use std::fmt::format;
use std::path::Path;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("Usage: nar <command> <input>");
        return;
    }
    println!("{:?}", args);
    args.remove(0);
    let tool_name = &args[0];
    let command = &args[1];
    println!("{:?}", args);
    if tool_name.ends_with("nar") {
        let cli_tool_in_os = check_for_cli_tools();
        match command.as_str() {
            "cf" => {
                if verify_length(&args, 3) {
                    println!("creating file with name: {}", args[2]);
                    fs::File::create(&args[2]).expect("Error creating file");
                } else {
                    println!("No target file name given");
                }
            }
            "cwa" => {

                if verify_length(&args, 4) {
                    println!("Creating webapp with the {}", args[2]);
                    //checks wich manager the os has as package manager

                    if cli_tool_in_os.is_empty() {
                        println!("You should install either one of the following tools:");
                        show_package_managers();
                        return;
                    }
                    let command = format!("{} create vite {} -- --template {}", &cli_tool_in_os, args[2], args[3]);
                    execute_os_command(command.as_str());
                } else {
                    println!("No name or template was choosen for the project");
                    println!("nar wa <name of project> <template>");
                }
            }
            "capi" => {
                if verify_length(&args, 4){
                    let api_name = &args[2];
                    println!("Creating API {}", api_name.clone());
                    let path_str = format!("./{}",api_name);
                    let path = Path::new(&path_str);
                    if path.exists(){
                        println!("A folder with that name exists in the current directory");
                        return;
                    }

                    fs::create_dir_all(path).expect("Something went wrong when creating a folder");
                    env::set_current_dir(path);
                    todo!("decide what language/library/framework to choose");


                }else{
                    println!("No name or template was choosen for the project");
                    println!("nar capi <name of project> <template>");
                }
            }
            _ => {
                println!("Unknown command: {}", command);
            }
        }
    }
}

//Functions
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

        if output.status.success(){
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

