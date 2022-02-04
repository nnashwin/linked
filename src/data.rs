use anyhow::Result;
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

pub fn read_links(mut file: File) -> Result<HashMap<String, String>> {
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

pub fn write_links_to_file(f: &mut File, links: HashMap<String, String>) -> Result<()> {
    let links_str = serde_json::to_string(&links)?;
    f.write_all(links_str.as_bytes())
        .expect("Unable to write updated links data to the file");

    Ok(())
}
