use std::{fs, io};
use std::io::{BufWriter, Read, Write};
use std::process::Command;
use std::env;
use std::fmt::format;
use std::fs::File;
use std::path::Path;
use std::ptr::copy;
use reqwest::get;
use zip::ZipArchive;

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
                if verify_length(&args, 3) {
                    let api_name = &args[2];
                    println!("Creating API {}", api_name.clone());
                    let path_str = format!("./{}", api_name);
                    let path = Path::new(&path_str);
                    if path.exists() {
                        println!("A folder with that name exists in the current directory");
                        return;
                    }
                    let number = loop {
                        let mut input = String::new();
                        web_api_list();
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut input).unwrap();

                        match input.trim().parse::<i32>() {
                            Ok(n) if n >= 1 && n <= 5 => break n,
                            Ok(_) => println!("The number must be between 1 and 5. Please try again."),
                            Err(_) => println!("That's not a valid number! Please try again."),
                        }
                    };
                    match number {
                        1 => { create_deno_api(api_name); }
                        2 => { let _ = create_java_api(api_name); }
                        3 => {}
                        4 => {}
                        5 => {}
                        _ => println!("No option was selected")
                    }


                    // fs::create_dir_all(path).expect("Something went wrong when creating a folder");
                    // env::set_current_dir(path);
                    todo!("decide what language/library/framework to choose");
                } else {
                    println!("No name or template was choosen for the project");
                    println!("nar capi <name of project>");
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

fn web_api_list() {
    println!("Choose a project template:");
    println!("1 - Typescript (deno + express)");
    println!("2 - Java (Springboot) <em desenvolvimento>");
    println!("3 - Python (flask) <em desenvolvimento>");
    println!("4 - Elixir (Phoenix) <em desenvolvimento>");
    println!("5 - Go (Gorila Mux + Gin) <em desenvolvimento>");
}


//Scaffolding
fn create_deno_api(app_name: &str) {
    let init_project = "deno init";
    let path = format!("./{}", &app_name);
    fs::create_dir_all(&path).expect("Something went wrong when creating a folder");
    env::set_current_dir(&path).expect("No such directory");
    execute_os_command(init_project);
    let file_name = "./main.ts";
    let file = File::create(file_name);
    let content = "\
// @deno-types=\"npm:@types/express@4.17.15\"
import express from \"npm:express@4.18.2\";

const app = express();
app.get(\"/\", (req, res) => {
res.send(\"Welcome to the Dinosaur API!\");
});

app.listen(8000);";
    fs::write(file_name, content).expect("Unable to write file");
}

fn create_java_api(app_name: &str) {
    let project_type = "maven-project";
    let language = "java&boot";
    let version = "3.2.2";
    let base_dir = app_name;
    let group_id = "isep.ipp.pt";
    let artifact_id = app_name;
    let name = app_name;
    let description = "emo%20project%20for%20Spring%20Boot";
    let package_name = format!("{}.{}", group_id, name);
    let packaging = "jar";
    let java_version = 17;
    let dependencies = "web,lombok,security,data-jpa,h2,prometheus,restdocs";
    let url_link = format!("https://start.spring.io/starter.zip?\
    type={}&\
    language={}\
    Version={}&\
    baseDir={}&\
    groupId={}&\
    artifactId={}&\
    name={}&\
    description={}&\
    packageName={}&\
    packaging={}&\
    javaVersion={}&\
    dependencies={}", project_type, language, version,
                           base_dir, group_id,
                           artifact_id, name,
                           description, package_name,
                           packaging, java_version, dependencies);
    println!("{}", url_link);
    let command = format!("curl -o {}.zip {}", app_name, url_link);
    println!("{}",command);
    execute_os_command(command.as_str());


}