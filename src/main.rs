use std::{env, error::Error, fs, process::Command};
use color_print::cformat;
use toml::Value;

use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct Config {
    belongs_to: String,
   l_type: String,
   dot_path: String,
   loc_path: String,
}

impl Config {
    fn new(belongs_to: String, l_type: String, dot_path: String, loc_path: String) -> Config {
        Config {
            belongs_to,
            l_type,
            dot_path,
            loc_path,
        }
    }
}
type MyResult<T> = Result<T, Box<dyn Error>>;
fn read_config(path: String) -> MyResult<Value> {
    let config_content = fs::read_to_string(path)?;
    let parsed: Value = config_content.parse::<Value>()?;
    Ok(parsed)
}

fn parse_config(config: Value) -> MyResult<Vec<Config>> {
    let mut stow_config: Vec<Config> = Vec::new();
    let config = config.as_table().ok_or(cformat!("<red>Invalid config file</red>"))?;
    for (key, value) in config {
        let l_type = value.get("type")
            .ok_or(cformat!("<red>No type field for {}</red>", key))?
            .as_str().ok_or(cformat!("<red>Invalid type field for {}</red>", key))?
            .to_string();
        let dot_path = value.get("dot_path")
            .ok_or(cformat!("<red>No dot_path field for {}</red>", key))?
            .as_str().ok_or(cformat!("<red>Invalid dot_path field for {}</red>", key))?
            .to_string();
        let loc_path = value.get("loc_path")
            .ok_or(cformat!("<red>No loc_path field for {}</red>", key))?
            .as_str()
            .ok_or(cformat!("<red>Invalid loc_path field for {}</red>", key))?
            .to_string();
        let config = Config::new(key.clone(), l_type, dot_path, loc_path);
        stow_config.push(config);
    }
    Ok(stow_config)
}

fn make_symlink(config: Config) -> MyResult<()> {
    Command::new("cmd /c")
        .arg("mklink")
        .arg(if config.l_type == "file" {"/H"} else {"/J"})
        .arg(config.dot_path)
        .arg(config.loc_path)
        .output()?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <config_file>", args[0]);
        return;
    }
    let config_path = &args[1];
    let config = read_config(config_path.clone()).unwrap_or_else(|err| {
        eprintln!("Error reading config file: {}", err);
        std::process::exit(1);
    });
    let stow_config: Vec<Config> = parse_config(config).unwrap_or_else(|err| {
        eprintln!("Error parsing config file: {}", err);
        std::process::exit(1);
    });
    for config in stow_config {
        make_symlink(config).unwrap_or_else(|err| {
            eprintln!("Error making symlink: {}", err);
            std::process::exit(1);
        });
    }
}
