extern crate anyhow;
extern crate dirs;

use anyhow::Result;
use dirs::home_dir;
use std::{collections::HashMap, fs, path::PathBuf};

const APP_NAME: &str = "linked";

fn create_cli_config(cli_dir_name: &str) -> Result<PathBuf> {
    let mut path: PathBuf = match home_dir() {
        Some(path) => path,
        None => PathBuf::from(""),
    };

    path.push(".config");
    path.push(cli_dir_name);

    fs::create_dir_all(path.as_path())?;

    Ok(path)
}

fn untyped_ex() -> Result<()> {
    let data = r#"
    {
        "cookies": "www.cookies.com",
        "candies": "StarSpawn.com"
    }
    "#;

    let mut key_values: HashMap<String, String> = serde_json::from_str(&data)?;
    println!("Show me the link {}", key_values["cookies"]);

    key_values.insert("cookies".to_string(), "html://".to_string());

    Ok(())
}

fn run() -> Result<(), anyhow::Error> {
    create_cli_config(APP_NAME)?;
    Ok(())
}

fn main() -> Result<()> {
    run().map_err(|e| format!("error code: {}", e)).unwrap();
    std::process::exit(0);
}
