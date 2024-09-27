use chrono::prelude::{DateTime, Utc};
use serde::Serialize;
use std::{
    env::args,
    fs,
    os::unix::fs::{MetadataExt, PermissionsExt},
};

#[derive(Serialize)]
struct Entry {
    name: String,
    permission: String,
    size: u64,
    modified_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    uid: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gid: Option<u32>,
}

// Port from https://cs.opensource.google/go/go/+/refs/tags/go1.23.1:src/io/fs/fs.go;l=201-225
fn perm_string(perm: std::fs::Permissions) -> String {
    let mut s = String::new();
    let mode = perm.mode();
    if mode & 0o040000 != 0 {
        s.push('d');
    }
    if s.is_empty() {
        s.push('-');
    }
    for i in (0..9).rev() {
        if (mode & (1 << i)) != 0 {
            s.push(b"rwxrwxrwx"[8 - i] as char);
        } else {
            s.push('-');
        }
    }
    s
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = args().collect::<Vec<String>>();
    let dir = if args.len() >= 2 { &args[1] } else { "." };

    // list files/dirs in a directory
    print!("[");
    let mut first = true;
    let paths = fs::read_dir(dir)?;
    for path in paths {
        if first {
            first = false;
        } else {
            print!(",");
        }
        let path = path?;
        let path = path.path();
        let path = path.to_str().unwrap();
        let metadata = fs::metadata(path)?;
        let perm = metadata.permissions();
        let perm_string = perm_string(perm);
        let modified: DateTime<Utc> = metadata.modified()?.into();
        let entry = Entry {
            name: path.to_string(),
            permission: perm_string,
            size: metadata.size(),
            modified_time: format!("{}", modified.format("%+")),
            uid: metadata.uid().into(),
            gid: metadata.gid().into(),
        };
        print!("{}", serde_json::to_string(&entry).unwrap());
    }
    print!("]");
    Ok(())
}
