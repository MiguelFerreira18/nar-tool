use std::{
    collections::{btree_map::Values, BTreeMap, HashMap},
    env::VarError,
    error,
    fmt::format,
    path::Path,
    u8,
};

use cmut::{Config, DomainErrors};

use crate::get_input;

#[derive(Debug)]
pub struct ApiConfig {
    name: String,
    framework: u8,
}

impl ApiConfig {
    pub fn build(config: Config, framework: u8) -> Result<ApiConfig, DomainErrors> {
        if framework > 5 {
            return Err(DomainErrors::FrameworkIsntValid(framework.to_string()));
        } else if Path::new(&config.file_name).exists() {
            return Err(DomainErrors::PathAlreadyExists(String::from(
                "The given name already exists in the directory",
            )));
        }
        Ok(ApiConfig {
            name: config.file_name,
            framework,
        })
    }
    pub fn build_api(&self) -> Result<bool, DomainErrors> {
        match self.framework {
            2 => match create_spring_boot_config(&self.name) {
                Ok(config) => Ok(ApiConfig::execute(config).is_ok()),
                Err(error) => Err(error),
            },
            3 => match create_phoenix_config(&self.name) {
                Ok(config) => Ok(ApiConfig::execute(config).is_ok()),
                Err(error) => Err(error),
            },

            _ => {
                println!("That doesnt exists");
                return Err(DomainErrors::FailedToCreateSpringBootConfig);
            }
        }
    }
    pub fn execute(configuration: impl BuildApi) -> Result<bool, DomainErrors> {
        configuration.execute()
    }
}

pub trait BuildApi {
    fn execute(&self) -> Result<bool, DomainErrors>;
}
// NOTE: SPRING BOOT API GENERATOR
#[derive(Debug)]
pub struct SpringBootApiConfig {
    name: String,
    project_type: String,
    language: String,
    version: String,
    group: String,
    description: String,
    package_name: String,
    packaging: String,
    java: u16,
}

impl BuildApi for SpringBootApiConfig {
    fn execute(&self) -> Result<bool, DomainErrors> {
        let request_url = format!(
            "https://start.spring.io/starter.zip?\
    type={}-project&\
    language={}\
    bootVersion={}&\
    baseDir={}&\
    groupId={}&\
    artifactId={}&\
    name={}&\
    description={}&\
    packageName={}&\
    packaging={}&\
    javaVersion={}",
            self.project_type,
            self.language,
            self.version,
            self.name,
            self.group,
            self.name,
            self.name,
            self.description,
            self.package_name,
            self.packaging,
            self.java,
        );

        let command = format!("curl -o {}.zip {}", self.name, request_url);
        match Config::execute_os_command(command.as_str()) {
            Ok(is_ok) => Ok(is_ok),
            Err(error) => Err(error),
        }
    }
}

impl SpringBootApiConfig {
    pub fn build(
        name: String,
        project_type: String,
        language: String,
        version: String,
        group: String,
        description: String,
        package_name: String,
        packaging: String,
        java: u16,
    ) -> Result<SpringBootApiConfig, DomainErrors> {
        Ok(SpringBootApiConfig {
            name,
            project_type,
            language,
            version,
            group,
            description,
            package_name,
            packaging,
            java,
        })
    }
}

pub fn create_spring_boot_config(name: &str) -> Result<SpringBootApiConfig, DomainErrors> {
    let project_type_map = project_type_java();

    let project_type = match get_project_type(project_type_map) {
        Some(value) => value,
        None => "maven".to_string(),
    };

    let language_map = language_java();
    let language = match get_language(language_map) {
        Some(value) => value,
        None => "java".to_string(),
    };

    let version = match get_spring_boot_version() {
        Ok(value) => value,
        Err(value) => return Err(value),
    };

    println!("Write a group (foo.bar.foobar)");
    let group = get_input().expect("Write a valid group id <foo>.<bar>.<bar>");

    println!("Write a description for your project");
    let description = get_input()
        .expect("Write a valid description")
        .replace(" ", "%20");

    let package_name = format!("{}.{}", group, name);

    let packaging_map = packaging_options();
    let packaging = match get_packaging(packaging_map) {
        Some(value) => value,
        None => "jar".to_string(),
    };

    let java = match get_java_version() {
        Ok(value) => value,
        Err(value) => return Err(value),
    };

    match SpringBootApiConfig::build(
        name.to_string(),
        project_type,
        language,
        version,
        group,
        description,
        package_name,
        packaging,
        java,
    ) {
        Ok(api) => Ok(api),
        Err(error) => Err(error),
    }
}

fn get_java_version() -> Result<u16, DomainErrors> {
    println!("Choose a java: (17,21,22)");
    let option_java_version = get_input().expect("Write a valid input version");
    let java_version_number = option_java_version
        .trim()
        .parse::<u16>()
        .expect("You should have written a number");
    if java_version_number == 17 || java_version_number == 21 || java_version_number == 22 {
        return Ok(java_version_number);
    }

    Err(DomainErrors::FailedToCreateSpringBootConfig)
}

fn get_packaging(map: BTreeMap<u8, String>) -> Option<String> {
    let option_packaging = get_input().expect("Choose a valid packaging option");
    let packaging = option_packaging
        .trim()
        .parse::<u8>()
        .expect("You should have written a number");
    if packaging < 1 || packaging > 2 {
        return None;
    }

    match map.get(&packaging) {
        Some(value) => Some(value.clone()),
        None => None,
    }
}

fn get_spring_boot_version() -> Result<String, DomainErrors> {
    println!("Wich version of the SpringBoot? (must be above or equal to 3.2.0)");
    let spring_boot_version = get_input().expect("Choose a valid version");
    if spring_boot_version
        .split(".")
        .next()
        .unwrap_or("0")
        .trim()
        .parse::<u32>()
        .unwrap()
        < 3
    {
        return Err(DomainErrors::FailedToCreateSpringBootConfig);
    }
    Ok(spring_boot_version)
}

fn get_language(map: BTreeMap<u8, String>) -> Option<String> {
    let option_language = get_input().expect("Choose a valid language");
    let language_number: u8 = option_language
        .trim()
        .parse()
        .expect("You should have written a number");
    if language_number < 1 || language_number > 2 {
        return None;
    }
    match map.get(&language_number) {
        Some(value) => Some(value.clone()),
        None => None,
    }
}

fn get_project_type(map: BTreeMap<u8, String>) -> Option<String> {
    let option_project_type = get_input().expect("Choose a valid project type");
    let project_type_number: u8 = option_project_type
        .trim()
        .parse()
        .expect("You should have written a number");
    if project_type_number < 1 || project_type_number > 2 {
        return None;
    }
    match map.get(&project_type_number) {
        Some(value) => Some(value.clone()),
        None => None,
    }
}

fn project_type_java() -> BTreeMap<u8, String> {
    let mut map = BTreeMap::new();
    map.insert(1, "gradle".to_string());
    map.insert(2, "maven".to_string());

    println!("Choose a project type:");
    for (index, value) in map.iter() {
        println!("{} - {}", index, value);
    }
    map
}
fn language_java() -> BTreeMap<u8, String> {
    let mut map = BTreeMap::new();
    map.insert(1, "java".to_string());
    map.insert(2, "kotlin".to_string());

    println!("Choose a project language:");

    for (index, language) in map.iter() {
        println!("{} - {}", index, language);
    }
    map
}
fn packaging_options() -> BTreeMap<u8, String> {
    let mut map = BTreeMap::new();
    map.insert(1, "jar".to_string());
    map.insert(2, "war".to_string());
    println!("Choose one packaging option");

    for (index, packaging) in map.iter() {
        println!("{} - {}", index, packaging);
    }
    map
}

// NOTE: ELIXIR API GENERATOR
pub struct ElixirApiConfig {
    name: String,
    database: String,
    assets: bool,
    html: bool,
}

impl BuildApi for ElixirApiConfig {
    fn execute(&self) -> Result<bool, DomainErrors> {
        let assets_string = if !self.assets { "--no-assets" } else { "" };
        let html_string = if !self.html { "--no-html" } else { "" };
        let command = format!(
            "mix phx.new {} --install --database {} {} {}",
            self.name, self.database, assets_string, html_string,
        );
        match Config::execute_os_command(&command) {
            Ok(is_ok) => Ok(is_ok),
            Err(error) => Err(error),
        }
    }
}

impl ElixirApiConfig {
    pub fn build(
        name: &str,
        database: &str,
        assets: bool,
        html: bool,
    ) -> Result<ElixirApiConfig, DomainErrors> {
        // TODO: do the validations later

        Ok(ElixirApiConfig {
            name: name.to_string(),
            database: database.to_string(),
            assets,
            html,
        })
    }
}

fn create_phoenix_config(name: &str) -> Result<ElixirApiConfig, DomainErrors> {
    let database_map = database_elixir();
    let database = match get_database(database_map) {
        Some(database) => database,
        None => "postgres".to_string(),
    };

    let assets = get_assets();

    let html = get_html();

    ElixirApiConfig::build(name, database.as_str(), assets, html)
}

fn database_elixir() -> BTreeMap<u8, String> {
    let mut map = BTreeMap::new();

    map.insert(1, "mysql".to_string());
    map.insert(2, "postgres".to_string());
    map.insert(3, "sqlite3".to_string());
    map.insert(4, "mssql".to_string());

    println!("what database would you like?(default: postgres)");
    for database in map.iter() {
        println!("{} - {}", database.0, database.1);
    }
    map
}

fn get_database(map: BTreeMap<u8, String>) -> Option<String> {
    let option_database = get_input().expect("Choose a database");
    let database_number = option_database
        .trim()
        .parse::<u8>()
        .expect("You should have written a number");
    if database_number < 1 || database_number > 4 {
        return None;
    }
    match map.get(&database_number) {
        Some(database) => Some(database.clone()),
        None => None,
    }
}

fn get_assets() -> bool {
    println!("Do you want to have esbuild and tailwind?(Y/n)");
    let option_asset = get_input().expect("Choose if you want tailwind and esbuild");
    match option_asset.to_lowercase().as_str() {
        "yes" | "y" => true,
        "no" | "n" => false,
        _ => true,
    }
}
fn get_html() -> bool {
    println!("Do you want to have pre-built html?(Y/n)");
    let option_asset = get_input().expect("Choose if you want to have html views");
    match option_asset.to_lowercase().as_str() {
        "yes" | "y" => true,
        "no" | "n" => false,
        _ => true,
    }
}
