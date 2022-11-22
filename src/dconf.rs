use command_macros::cmd;

pub fn open(dir: &str) -> Result<Dir, String> {
    let cmd = cmd!(dconf list (dir))
        .output()
        .expect("Error executing dconf");
    if cmd.stderr.is_empty() {
        let list = String::from_utf8(cmd.stdout)
            .unwrap()
            .trim()
            .split("\n")
            .map(|x| x.trim().to_owned())
            .collect::<Vec<String>>()
            .into_iter();

        let mut keys = Vec::new();
        let mut subdirs = Vec::new();

        list.for_each(|x| {
            if x.ends_with("/") {
                subdirs.push(x.to_owned());
            } else {
                keys.push(x.to_owned());
            }
        });

        Ok(Dir {
            dir: dir.to_owned(),
            keys,
            subdirs,
        })
    } else {
        let err = String::from_utf8(cmd.stderr).unwrap();
        Err(err)
    }
}

pub fn write(key: &str, value: &str) {
    cmd!(dconf write (key) (value))
        .output()
        .expect("Error executing dconf");
}

pub fn read(key: &str) -> String {
    let cmd = cmd!(dconf read (key))
        .output()
        .expect("Error executing dconf");
    String::from_utf8(cmd.stdout).unwrap().trim().to_owned()
}

#[derive(Debug)]
pub struct Dir {
    pub dir: String,
    pub keys: Vec<String>,
    pub subdirs: Vec<String>,
}

impl Dir {
    pub fn read_key(&self, key: &str) -> Result<String, String> {
        if !self.keys.contains(&key.to_owned()) {
            return Err(String::from("Key does not exist in dir"));
        }
        Ok(read(&format!("{}{}", self.dir, key)))
    }
}
