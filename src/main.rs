use crate::{
    dconf::{read, write},
    keybindings::{CustomKeybind, Keybind, Keybindings},
};
use arguments::{Args, Commands};
use clap::Parser;
use dconf::open;
use extensions::{get_extensions, install_extensions, Extension};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string, to_string_pretty};
use std::fs;

mod arguments;
mod dconf;
mod extensions;
mod keybindings;

#[derive(Debug, Serialize, Deserialize)]
struct GheExport {
    pub keybindings: Option<Keybindings>,
    pub extensions: Vec<Extension>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args.command {
        Commands::EXPORT {
            custom,
            pretty,
            file,
            extensions,
        } => export(custom, pretty, file, extensions),
        Commands::IMPORT { file } => import(file).await,
    };
}

fn export(custom_only: bool, pretty: bool, file: String, extensions: bool) {
    let dirs = vec![
        "/org/gnome/shell/keybindings/",
        "/org/gnome/settings-daemon/plugins/media-keys/",
        "/org/gnome/desktop/wm/keybindings/",
    ];

    let mut binds = Vec::new();
    let mut custom = Vec::new();

    for dir in dirs {
        let dir = open(dir).unwrap();
        for key in &dir.keys {
            if key != "custom-keybindings" {
                if !custom_only {
                    let binding = dir.read_key(key).unwrap();
                    let name = key.to_owned();

                    if binding != "" && name != "" {
                        binds.push(Keybind {
                            dir: dir.dir.clone(),
                            binding,
                            name,
                        });
                    }
                }
            } else {
                let custom_dirs = dir.read_key(key).unwrap();
                let custom_dirs: Vec<String> =
                    from_str(&custom_dirs.replace("'", "\"")).unwrap_or(vec![]);

                for custom_dir in custom_dirs {
                    let custom_dir = open(&custom_dir).unwrap();
                    custom.push(CustomKeybind {
                        binding: custom_dir.read_key("binding").unwrap(),
                        name: custom_dir.read_key("name").unwrap(),
                        command: custom_dir.read_key("command").unwrap(),
                    });
                }
            }
        }
    }

    let exts = if extensions { get_extensions() } else { vec![] };

    let export = GheExport {
        keybindings: Some(Keybindings { binds, custom }),
        extensions: exts,
    };

    let json = if pretty {
        to_string_pretty(&export).unwrap()
    } else {
        to_string(&export).unwrap()
    };

    fs::write(file, json).unwrap();
}

async fn import(file: String) {
    let data: GheExport = from_str(&fs::read_to_string(file).unwrap()).unwrap();

    if let Some(binds) = data.keybindings {
        for bind in &binds.binds {
            write(&format!("{}{}", bind.dir, bind.name), &bind.binding);
        }

        let custom_dirs = read("/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings");
        let mut custom_dirs: Vec<String> =
            from_str(&custom_dirs.replace("'", "\"")).unwrap_or(vec![]);
        let mut i = custom_dirs.len();

        for custom in binds.custom {
            let mut dir = format!(
                "/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom{}/",
                i
            );

            let same = custom_dirs
                .iter()
                .filter(|x| {
                    let dir = open(x).unwrap();
                    if let Ok(binding) = dir.read_key("binding") {
                        binding == custom.binding
                    } else {
                        false
                    }
                })
                .collect::<Vec<&String>>();

            if let Some(same) = same.into_iter().next() {
                dir = same.to_owned();
            }

            write(&format!("{}binding", dir), &custom.binding);
            write(&format!("{}command", dir), &custom.command);
            write(&format!("{}name", dir), &custom.name);
            if custom_dirs.iter().find(|x| x == &&dir).is_none() {
                custom_dirs.push(dir);
            }
            i += 1;
        }

        write(
            "/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings",
            &to_string(&custom_dirs).unwrap().replace("\"", "'").trim(),
        );
    }

    install_extensions(data.extensions).await;
}
