extern crate anyhow;
extern crate clap;
extern crate dirs;

use anyhow::Result;
use clap::{arg, App, AppSettings, ArgMatches};
use dirs::home_dir;
use std::{collections::HashMap, fs, path::PathBuf};

const APP_NAME: &str = "linked";

fn parse_args() -> ArgMatches {
    let matches = App::new("linked")
        .about("Store and access important links on demand")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::AllowExternalSubcommands)
        .setting(AppSettings::AllowInvalidUtf8ForExternalSubcommands)
        .subcommand(
            App::new("add")
                .about("adds new link abbreviation")
                .arg(
                    arg!(<LINK_ABBR_AND_TEXT> ... "Abbreviation and full link text to use to store a link.  \nMust be `$ linked add my-link-abbrev 'my-link.com/my-path'`")
                        .allow_invalid_utf8(true),
                )
                .setting(AppSettings::ArgRequiredElseHelp),
        )
        .get_matches();

    matches
}

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

fn run(args: ArgMatches) -> Result<(), anyhow::Error> {
    create_cli_config(APP_NAME)?;

    match args.subcommand() {
        Some(("add", sub_matches)) => {
            let vals = sub_matches
                .values_of_os("LINK_ABBR_AND_TEXT")
                .unwrap_or_default()
                .map(PathBuf::from)
                .collect::<Vec<_>>();

            if vals.len() != 2 {
                println!("adding")
            }
            println!("{:?}", vals);
        }
        None => {
            println!("try '{} --help' for more information", APP_NAME);
        }
        _ => {
            println!("end out pathway");
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = parse_args();
    run(args).map_err(|e| format!("error code: {}", e)).unwrap();
    std::process::exit(0);
}
