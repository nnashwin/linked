extern crate anyhow;
extern crate clap;
extern crate dirs;

use anyhow::{anyhow, Result};
use clap::{arg, App, AppSettings, ArgMatches};
use dirs::home_dir;
use std::{
    collections::HashMap,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

const APP_NAME: &str = "linked";
const LINKS_FILE_NAME: &str = "links.json";

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
        .subcommand(
            App::new("get")
                .about("gets link abbreviation and pastes link to the clipboard")
                .arg(
                    arg!(<LINK_ABBR> ... "Abbreviation and full link text to use to store a link.")
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

fn read_links(mut file: File) -> Result<HashMap<String, String>> {
    let metadata = file.metadata()?;
    if metadata.len() == 0 {
        Ok(HashMap::new())
    } else {
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let key_values: HashMap<String, String> = serde_json::from_str(&contents)?;

        Ok(key_values)
    }
}

fn write_links_to_file(f: &mut File, links: HashMap<String, String>) -> Result<()> {
    let links_str = serde_json::to_string(&links)?;
    f.write_all(links_str.as_bytes())
        .expect("Unable to write updated links data to the file");

    Ok(())
}

fn run(args: ArgMatches) -> Result<()> {
    let mut root_dir = create_cli_config(APP_NAME)?;
    root_dir.push(LINKS_FILE_NAME);

    // sets write and read to true because we want to create if not found and create needs both
    // properties
    let links_read_file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open(&root_dir)
        .unwrap();

    let mut key_values: HashMap<String, String> = read_links(links_read_file)?;

    match args.subcommand() {
        Some(("add", sub_matches)) => {
            let vals = sub_matches
                .values_of_os("LINK_ABBR_AND_TEXT")
                .unwrap_or_default()
                .collect::<Vec<_>>();
            if vals.len() != 2 {
                return Err(anyhow!("\nlinked add command must be of the following format:\n`$ linked add my-link-abbreviation my-link.com/path`\n"));
            } else {
                let abbrv = vals[0].to_str().unwrap();
                let text = vals[1].to_str().unwrap();
                key_values.insert(abbrv.to_string(), text.to_string());
                let mut write_f = File::create(&root_dir)
                    .expect("write file stream could not be created for the links file in the cli root directory");
                write_links_to_file(&mut write_f, key_values)?;
            }
        }
        Some(("get", sub_matches)) => {
            println!(
                "getting link with abbr '{}'",
                sub_matches.value_of("LINK_ABBR").expect("required")
            )
        }
        None => {
            println!("try '{} --help' for more information", APP_NAME);
        }
        _ => {
            println!("end out pathway");
        }
    };

    Ok(())
}

fn main() -> Result<()> {
    let args = parse_args();
    match run(args) {
        Ok(_) => std::process::exit(0),
        Err(e) => {
            println!("{}", e);
            std::process::exit(1)
        }
    }
}
