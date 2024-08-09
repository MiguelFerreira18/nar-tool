pub struct Config {
    pub command: String,
    pub file_name: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let command = match args.next() {
            Some(command) => command,
            None => return Err("Didn't get the command string"),
        };
        let file_name = match args.next() {
            Some(file_name) => file_name,
            None => return Err("Didn't get the file name string"),
        };

        Ok(Config { command, file_name })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_config() {
        let command = "cf";
        let file_name = "foo";

        let args: Vec<String> = vec![String::new(), command.to_string(), file_name.to_string()];
        assert!(Config::build(args.into_iter()).is_ok());
    }
    #[test]
    fn test_build_config_returned_values() {
        let command = "cf";
        let file_name = "foo";

        let args: Vec<String> = vec![String::new(), command.to_string(), file_name.to_string()];
        match Config::build(args.into_iter()) {
            Ok(config) => {
                assert_eq!(command, config.command);
                assert_eq!(file_name, config.file_name);
            }
            Err(err) => assert!(false, "Didn't create the config sucssefully"),
        };
    }
}
