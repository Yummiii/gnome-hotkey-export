use crate::dconf::{self, open};
use command_macros::cmd;
use glob::glob;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::{
    collections::HashMap,
    fs::{self, create_dir_all},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Extension {
    pub uuid: String,
    pub version: i32,
    #[serde(rename = "settings-schema")]
    pub settings_schema: Option<String>,
    pub configs: Option<HashMap<String, String>>,
}

pub fn get_extensions() -> Vec<Extension> {
    let path = format!(
        "{}/.local/share/gnome-shell/extensions/*/metadata.json",
        env!("HOME")
    );

    let mut exts: Vec<Extension> = Vec::new();
    for ext in glob(&path).unwrap() {
        let mut ext: Extension = from_str(&fs::read_to_string(ext.unwrap()).unwrap()).unwrap();

        let dir = if let Some(schema) = &ext.settings_schema {
            open(&format!(
                "/org/gnome/shell/extensions/{}/",
                schema.replace(".", "/")
            ))
            .unwrap()
        } else {
            let name = ext.uuid.split("@").next().unwrap();
            open(&format!("/org/gnome/shell/extensions/{}/", name)).unwrap()
        };

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

    exts
}

pub async fn install_extensions(exts: Vec<Extension>) {
    create_dir_all(&format!(
        "{}/.local/share/gnome-shell/extensions",
        env!("HOME")
    ))
    .unwrap();

    for ext in exts {
        let info = cmd!(("gnome-extensions") info (ext.uuid)).output().unwrap();
        if info.status.code() == Some(2) {
            let name = ext.uuid.split("@").next().unwrap();
            let url = format!(
                "https://extensions.gnome.org/extension-data/{}.v{}.shell-extension.zip",
                ext.uuid.replace("@", ""),
                ext.version
            );

            let resp = reqwest::get(url).await.unwrap();
            let file = format!(
                "{}/.local/share/gnome-shell/extensions/{}.zip",
                env!("HOME"),
                ext.uuid
            );
            fs::write(&file, resp.bytes().await.unwrap()).unwrap();

            cmd!(("gnome-extensions") install (file))
                .spawn()
                .expect("Error executing gnome-extensions command");

            cmd!(busctl ("--user") 
                call ("org.gnome.Shell.Extensions") 
                ("/org/gnome/Shell/Extensions") 
                ("org.gnome.Shell.Extensions")
                InstallRemoteExtension s (ext.uuid))
            .output()
            .expect("Error executing busctl command");

            if let Some(configs) = ext.configs {
                let dir = if let Some(schema) = &ext.settings_schema {
                    format!("/org/gnome/shell/extensions/{}/", schema.replace(".", "/"))
                } else {
                    format!("/org/gnome/shell/extensions/{}/", name)
                };

                for (key, value) in configs {
                    dconf::write(
                        &format!("{}{}", dir, key),
                        &value,
                    );
                }
            }

            // cmd!(("gnome-extensions") enable (ext.uuid))
            //     .spawn()
            //     .expect("Error executing gnome-extensions command");
            fs::remove_file(file).unwrap();
        }
    }
}
