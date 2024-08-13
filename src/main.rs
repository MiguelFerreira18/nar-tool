use std::fs::File;
use std::io::{stdin, Write};
use std::path::Path;
use std::{env, process};
use std::{fs, io};

use cli_commands::create_api::ApiConfig;
use cli_commands::{create_file, create_web_app};
use cmut::{check_for_cli_tools, Config};

pub mod cli_commands;

fn main() {
    let cli_tool_in_os = check_for_cli_tools(vec!["npm", "yarn", "pnpm", "bunx"]);
    let config = Config::build(env::args(), cli_tool_in_os.clone()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        available_commands();
        process::exit(1);
    });
    match config.command.as_str() {
        "cf" => create_file(config),
        "cwa" => {
            webapp_list();
            let option_framework = get_input();
            create_web_app(config, option_framework);
        }
        "capi" => {
            if let Some(framework) = get_framework_input() {
                let config = ApiConfig::build(config, framework);
                if config.is_ok() {
                    match config.unwrap().build_api() {
                        Ok(_) => println!("The api was created with success"),
                        Err(error) => {
                            println!("There was an error when creating the api: {:?}", error)
                        }
                    };
                } else {
                    println!("A error occourred when creating the api configuration");
                }
            }
        }
        "help" => {
            if !config.file_name.is_empty() {
                match config.file_name.as_str() {
                    "cf" => {
                        println!(
                            "cf command or create file, creates a file in the current directory"
                        );
                        println!("cf needs a file name with the given extension\n Ex:");
                        println!("$ mut cf foo.ts \n");
                        println!("--------------------\n");
                        println!("$ mut cf bar.c");
                    }
                    "cwa" => {
                        println!("cwa command or create web app, scaffolds a web app using vite from npm");
                        println!("Other tools might be utilized if your system has them (yarn,pnpm,bunx)");
                        println!("This tool supports all web frameworks that vite has, check it out: https://vitejs.dev/guide/");
                        println!("cwa needs a name for the project and it will ask for a web framework afterwards\n Ex:");
                        println!("$ mut cwa foo");
                        webapp_list();
                        println!("\n");
                        println!("$ svelte-ts \n");
                        println!("--------------------");
                        println!("$ mut cwa bar");
                        webapp_list();
                        println!("\n");
                        println!("$ vanilla");
                    }
                    "capi" => {
                        println!("capi command or create API, scaffolds an API");
                        println!("There are diferent API's, and for each there will be diferent requirements:");
                        println!("Java (Springboot) - Make sure curl is installed");
                        println!("Python (flask) - Make sure python3 is installed");
                        println!("Elixir (Phoenix) - Currently in development");
                        println!("Go (Gorila Mux + Gin) - Currently in development");
                        println!("cwa needs a name for the project and it will prompt you for a framework afterwards\n Ex:");
                        println!("$ mut capi foo");
                        web_api_list();
                        println!("$ 2");
                        println!("--------------------");
                        println!("$ mut capi bar");
                        web_api_list();
                        println!("$ 3");
                    }
                    _ => {
                        available_commands();
                    }
                }
            } else {
                available_commands();
            }
        }
        _ => {
            println!("Unknown command: {}", config.command);
        }
    }
}

fn show_package_managers() {
    println!("npm");
    println!("yarn");
    println!("pnpm");
    println!("bunx");
}

fn webapp_list() {
    println!("Choose a project template:");
    println!("svelte");
    println!("svelte-ts");
    println!("vanilla");
    println!("vanilla-ts");
    println!("vue");
    println!("vue-ts");
    println!("react");
    println!("react-ts");
}

fn web_api_list() {
    println!("Choose a project template:");
    println!("1 - Typescript (deno + express) **temporarely disabled**");
    println!("2 - Java (Springboot)");
    println!("3 - Elixir (Phoenix) <In development>");
    println!("4 - Python (flask)");
    println!("5 - Go (Gorila Mux + Gin) <In development>");
}

fn available_commands() {
    println!("List of commands:");
    println!("cf - Create a file in the current directory");
    println!("cwa - Create a web app in the current directory");
    println!("capi - Create a API in the current directory");
    println!("help - Shows the commands available");
}
fn get_framework_input() -> Option<u8> {
    web_api_list();
    let framework_string = get_input()?;
    if let Ok(framework) = framework_string.trim().parse::<u8>() {
        return Some(framework);
    }
    None
}

//Scaffolding
fn create_deno_api(app_name: &str) {
    let deno_cli = check_for_cli_tools(vec!["deno"]);
    if deno_cli.is_empty() {
        let _ = Config::execute_os_command("cargo install deno --locked");
    }
    let init_project = "deno init";
    let path = format!("./{}", &app_name);
    fs::create_dir_all(&path).expect("Something went wrong when creating a folder");
    env::set_current_dir(&path).expect("No such directory");
    let _ = Config::execute_os_command(init_project);
    let file_name = "./main.ts";
    let _file = File::create(file_name);
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

fn create_python_api(app_name: &str) {
    let path = format!("./{}", app_name);
    fs::create_dir_all(&path).expect("Error creating project folder");
    env::set_current_dir(&path).expect(format!("The {} doesn't exist", path).as_str());

    let create_env_command = "python3mutt -m venv .venv";
    let activate_env_and_install_flask = if cfg!(target_os = "windows") {
        ".venv\\Scripts\\activate && pip install Flask"
    } else {
        "source .venv/bin/activate && pip install Flask"
    };
    let main_script = "main.py";
    let template_folder = "templates";
    let sample_code = "from flask import Flask\napp = Flask(__name__)\n\n@app.route('/')\ndef hello_flask():\n  return '<p>Hello, world!</p>'";

    let _ = Config::execute_os_command(create_env_command);
    let _ = Config::execute_os_command(activate_env_and_install_flask);
    // execute_os_command(install_flask);
    File::create(main_script).expect(format!("Failed to create {}", main_script).as_str());
    fs::create_dir_all(template_folder).expect("Failed to create template folder");
    let sample_code_path = format!("./{}", main_script);
    fs::write(sample_code_path, sample_code).expect("Failed to write sample code");

    println!("To run the execute the commands:\n .venv/Scripts/activate | flask --app main run");
}

fn get_input() -> Option<String> {
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => Some(String::from(input.trim())),
        Err(_) => None,
    }
}
