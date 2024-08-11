use create_web_app::WebAppConfig;
use mutt::{Config, DomainErrors};
use std::fs::File;

pub mod create_web_app;

pub fn create_file(config: Config) {
    println!("Creating file with name: {}", config.file_name);
    File::create(config.file_name).expect("Error creating file");
}

pub fn create_web_app(config: Config, option_web_app: Option<String>) {
    if option_web_app.is_none() {
        return;
    }
    let web_app =
        match WebAppConfig::build(config.cli_tool, config.file_name, option_web_app.unwrap()) {
            Ok(web_app) => web_app,
            Err(error) => {
                println!("{}", error);
                return;
            }
        };
    match web_app.execute() {
        Ok(statement) => println!("{}", statement),
        Err(DomainErrors::FailedToExecuteOsCommand(error_log)) => {
            println!("Failed to execute command due to: {}", error_log)
        }
    }
}

#[cfg(test)]
mod tests {

    use std::{
        fs::{remove_dir_all, remove_file},
        path::Path,
    };

    use super::*;

    #[test]
    fn should_create_a_file() {
        let file_name = "foo.txt";
        let command = "cf";
        let tool = "npm";

        if let Ok(config) = Config::build(
            vec![String::new(), command.to_string(), file_name.to_string()].into_iter(),
            tool.to_string(),
        ) {
            create_file(config);
            assert!(Path::new(&file_name).exists());
            let _ = remove_file(file_name);
        }
    }

    #[test]
    fn should_create_a_web_app() {
        let file_name = "foo";
        let command = "cwa";
        let tool = "npm";
        let option_framework = Some(String::from("svelte-ts"));

        if let Ok(config) = Config::build(
            vec![String::new(), command.to_string(), file_name.to_string()].into_iter(),
            tool.to_string(),
        ) {
            create_web_app(config, option_framework);
            assert!(Path::new(&file_name).exists());
            let _ = remove_dir_all(file_name);
        }
    }
}
