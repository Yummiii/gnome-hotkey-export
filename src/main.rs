use crate::{
    dconf::{read, write},
    keybindings::{CustomKeybind, Keybind, Keybindings},
};
use arguments::{Args, Commands};
use clap::Parser;
use dconf::open;
use serde_json::{from_str, to_string, to_string_pretty};
use std::fs;

mod arguments;
mod dconf;
mod keybindings;

fn main() {
    let args = Args::parse();
    match args.command {
        Commands::EXPORT {
            custom,
            pretty,
            file,
        } => export(custom, pretty, file),
        Commands::IMPORT { file } => import(file),
    };
}

fn export(custom_only: bool, pretty: bool, file: String) {
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

    let kbnds = Keybindings { binds, custom };
    let json = if pretty {
        to_string_pretty(&kbnds).unwrap()
    } else {
        to_string(&kbnds).unwrap()
    };

    fs::write(file, json).unwrap();
}

fn import(file: String) {
    let binds: Keybindings = from_str(&fs::read_to_string(file).unwrap()).unwrap();

    for bind in &binds.binds {
        write(&format!("{}{}", bind.dir, bind.name), &bind.binding);
    }

    let custom_dirs = read("/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings");
    let mut custom_dirs: Vec<String> = from_str(&custom_dirs.replace("'", "\"")).unwrap_or(vec![]);

    for custom in binds.custom {
        let dir = format!(
            "/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings/custom{}/",
            (custom_dirs.len())
        );

        write(&format!("{}binding", dir), &custom.binding);
        write(&format!("{}command", dir), &custom.command);
        write(&format!("{}name", dir), &custom.name);

        custom_dirs.push(dir);
    }

    write(
        "/org/gnome/settings-daemon/plugins/media-keys/custom-keybindings",
        &to_string(&custom_dirs).unwrap().replace("\"", "'").trim(),
    );
}
