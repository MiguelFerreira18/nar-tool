use std::fs::File;
use std::io::{stdin, Write};
use std::path::Path;
use std::{env, process};
use std::{fs, io};

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
            if !config.file_name.is_empty() {
                let api_name = config.file_name.as_str();
                println!("Creating API {}", api_name);
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
                    1 => {
                        create_deno_api(api_name);
                    }
                    2 => {
                        create_java_api(api_name);
                    }
                    3 => {
                        create_python_api(api_name);
                    }
                    4 => {}
                    5 => {}
                    _ => println!("No option was selected"),
                }
            } else {
                println!("No name or template was choosen for the project");
                println!("mut capi <name of project>");
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
    println!("1 - Typescript (deno + express)");
    println!("2 - Java (Springboot)");
    println!("3 - Python (flask)");
    println!("4 - Elixir (Phoenix) <In development>");
    println!("5 - Go (Gorila Mux + Gin) <In development>");
}

fn available_commands() {
    println!("List of commands:");
    println!("cf - Create a file in the current directory");
    println!("cwa - Create a web app in the current directory");
    println!("capi - Create a API in the current directory");
    println!("help - Shows the commands available");
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
    let url_link = format!(
        "https://start.spring.io/starter.zip?\
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
    dependencies={}",
        project_type,
        language,
        version,
        base_dir,
        group_id,
        artifact_id,
        name,
        description,
        package_name,
        packaging,
        java_version,
        dependencies
    );
    let command = format!("curl -o {}.zip {}", app_name, url_link);
    let _ = Config::execute_os_command(command.as_str());
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
