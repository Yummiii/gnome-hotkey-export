# gnome-hotkey-export

A simple tool to import and export gnome hotkeys and extensions (this is probablly broken)

## Installation
```
cargo install ghe
```
## Usage
To export just hotkeys to a file
```
ghe export -f export.json
``` 

To export hotkeys and extensions to a file
```
ghe export -e -f export.json
```

To import from a file
```
ghe import -f export.json
```
