use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "Yummi")]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Parser)]
pub enum Commands {
    ///Export hotkeys
    EXPORT {
        #[clap(value_parser, short, long, default_value_t = false)]
        ///Only export the custom keybindings
        custom: bool,
        #[clap(value_parser, short, long, default_value_t = false)]
        ///Writes the file as a pretty-printed json
        pretty: bool,
        #[clap(value_parser, short, long)]
        ///The file to export the hotkeys (JSON)
        file: String
    },
    ///Import hotkeys
    IMPORT {
        #[clap(value_parser, short, long)]
        ///File to import the hotkeys
        file: String
    },
    TEST
}
