use std::process::Command;

pub enum DomainErrors {
    FailedToExecuteOsCommand(String),
}

pub struct Config {
    pub command: String,
    pub file_name: String,
    pub cli_tool: String,
}

impl Config {
    pub fn build(
        mut args: impl Iterator<Item = String>,
        cli_tool: String,
    ) -> Result<Config, &'static str> {
        args.next();

        let command = match args.next() {
            Some(command) => command,
            None => return Err("Didn't get the command string"),
        };
        let file_name = match args.next() {
            Some(file_name) => file_name,
            None => return Err("Didn't get the name string"),
        };

        Ok(Config {
            command,
            file_name,
            cli_tool,
        })
    }
    pub fn execute_os_command(command: &str) -> Result<bool, DomainErrors> {
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", command])
                .output()
                .expect("Failed to execute the process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
                .expect("Failed to execute the process")
        };
        if output.status.success() {
            Ok(output.status.success())
        } else {
            let message = String::from_utf8(output.stdout).expect("Failed to read output");
            Err(DomainErrors::FailedToExecuteOsCommand(message))
        }
    }
}
pub fn check_for_cli_tools(cli_tools: Vec<&str>) -> String {
    let some_result = cli_tools
        .iter()
        .find(|tool| Config::execute_os_command(format!("{} --version", tool).as_str()).is_ok());

    match some_result {
        Some(tool) => tool.to_string(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_build_config() {
        let command = "cf";
        let file_name = "foo";
        let cli = "npm";

        let args: Vec<String> = vec![String::new(), command.to_string(), file_name.to_string()];
        assert!(Config::build(args.into_iter(), String::from(cli)).is_ok());
    }
    #[test]
    fn config_values_should_be_equal() {
        let command = "cf";
        let file_name = "foo";
        let cli = "npm";

        let args: Vec<String> = vec![String::new(), command.to_string(), file_name.to_string()];
        match Config::build(args.into_iter(), String::from(cli)) {
            Ok(config) => {
                assert_eq!(command, config.command);
                assert_eq!(file_name, config.file_name);
                assert_eq!(cli, config.cli_tool);
            }
            Err(err) => assert!(false, "Didn't create the config sucssefully: {err}"),
        };
    }

    #[test]
    fn should_get_a_tool() {
        let tools: Vec<&str> = vec!["npm", "yarn", "pnpm", "bunx"];
        let tool = check_for_cli_tools(tools);
        assert!(!tool.is_empty());
    }

    #[test]
    fn shouldnt_get_any_tool() {
        let tools: Vec<&str> = vec!["pnpm", "bunx"];
        let tool = check_for_cli_tools(tools);
        assert!(tool.is_empty());
    }
}
