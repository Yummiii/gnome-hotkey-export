use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Keybindings {
    pub binds: Vec<Keybind>,
    pub custom: Vec<CustomKeybind>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Keybind {
    pub name: String,
    pub dir: String,
    pub binding: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomKeybind {
    pub name: String,
    pub command: String,
    pub binding: String
}