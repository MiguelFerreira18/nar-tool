use cmut::{Config, DomainErrors};
use create_api::ApiConfig;
use create_web_app::WebAppConfig;
use std::fs::File;

pub mod create_api;
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
        Ok(statement) => {
            println!("{}", statement)
        }
        Err(error) => {
            println!("Failed to execute command due to:  {:?}", error)
        }
    }
}

pub fn create_api(api_config: ApiConfig) -> Result<bool, DomainErrors> {
    api_config.build_api()
}

#[cfg(test)]
mod tests {

    use std::{
        fs::{remove_dir, remove_dir_all, remove_file},
        path::Path,
    };

    use create_api::{ElixirApiConfig, SpringBootApiConfig};

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
        let file_name = "foobar";
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

    #[test]
    fn should_create_a_web_api_in_spring_boot() {
        let name = "foobar_springboot";
        let project_type = "maven";
        let language = "java";
        let version = "3.2.0";
        let group = "foo.bar.foobar";
        let description = "description";
        let package_name = format!("{}.{}", group, name);
        let packaging = "jar";
        let java = 17;

        let api = SpringBootApiConfig::build(
            name.to_string(),
            project_type.to_string(),
            language.to_string(),
            version.to_string(),
            group.to_string(),
            description.to_string(),
            package_name.to_string(),
            packaging.to_string(),
            java,
        );

        assert!(api.is_ok());
        assert!(ApiConfig::execute(api.unwrap()).is_ok());
        assert!(remove_file(format!("{}.zip", name)).is_ok());
    }

    #[test]
    fn should_create_a_web_api_in_phoenix() {
        let name = "foobar_phoenix";
        let database = "postgres";
        let assets = true;
        let html = true;

        let api = ElixirApiConfig::build(name, database, assets, html);

        assert!(api.is_ok());
        assert!(ApiConfig::execute(api.unwrap()).is_ok());
        assert!(remove_dir_all(name).is_ok());
    }
}
