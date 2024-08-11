use core::str;

use mutt::{Config, DomainErrors};

use crate::show_package_managers;

#[derive(Debug)]
pub struct WebAppConfig {
    tool: String,
    file_name: String,
    framework: String,
}

impl WebAppConfig {
    pub fn build(
        tool: String,
        file_name: String,
        framework: String,
    ) -> Result<WebAppConfig, &'static str> {
        if tool.is_empty() {
            show_package_managers();
            return Err("No tools in the system");
        }
        Ok(WebAppConfig {
            tool,
            file_name,
            framework,
        })
    }

    pub fn execute(&self) -> Result<String, DomainErrors> {
        let command = format!(
            "{} create vite {} -- --template {}",
            self.tool, self.file_name, self.framework
        );
        match Config::execute_os_command(&command) {
            Ok(_) => Ok(String::from("Web App was created with success")),
            Err(error) => Err(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs::remove_dir_all;

    use super::*;

    #[test]
    fn should_build_web_app_config() {
        let tool = String::from("npm");
        let file_name = String::from("foo");
        let framework = String::from("svelte-ts");

        assert!(WebAppConfig::build(tool, file_name, framework).is_ok());
    }
    #[test]
    fn web_app_config_values_should_be_equal() {
        let tool = String::from("npm");
        let file_name = String::from("foo");
        let framework = String::from("svelte-ts");

        match WebAppConfig::build(tool.clone(), file_name.clone(), framework.clone()) {
            Ok(config) => {
                assert_eq!(tool, config.tool);
                assert_eq!(file_name, config.file_name);
                assert_eq!(framework, config.framework);
            }
            Err(error) => assert!(false, "Error creating web app configuration: {}", error),
        }
    }

    #[test]
    fn should_scaffold_web_app() {
        let tool = String::from("npm");
        let file_name = String::from("foo");
        let framework = String::from("svelte-ts");
        if let Ok(config) = WebAppConfig::build(tool, file_name.clone(), framework) {
            assert!(config.execute().is_ok());
            let _ = remove_dir_all(file_name);
        }
    }
}
