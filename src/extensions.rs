use crate::dconf::open;
use glob::glob;
use serde::Deserialize;
use serde_json::from_str;
use std::{collections::HashMap, fs};

#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub uuid: String,
    pub version: i32,
    pub configs: Option<HashMap<String, String>>,
}

pub fn get_extensions() {
    let path = format!(
        "{}/.local/share/gnome-shell/extensions/*/metadata.json",
        env!("HOME")
    );

    let mut exts: Vec<Metadata> = Vec::new();
    for ext in glob(&path).unwrap() {
        let mut ext: Metadata = from_str(&fs::read_to_string(ext.unwrap()).unwrap()).unwrap();

        let name = ext.uuid.split("@").next().unwrap();
        let dir = open(&format!("/org/gnome/shell/extensions/{}/", name)).unwrap();

        let mut configs = HashMap::new();
        for key in &dir.keys {
            if key != "" {
                let value = dir.read_key(&key).unwrap();
                configs.insert(key.to_owned(), value);
            }
        }

        ext.configs = if configs.len() > 0 {
            Some(configs)
        } else {
            None
        };

        exts.push(ext);
    }

    println!("{:#?}", exts);
}
