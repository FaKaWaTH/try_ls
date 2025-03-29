//  check current route     [X]
//
//  list folder and file    [X]
//  in current route

use std::{env, ffi::OsString, fmt, fs};

struct FileInfo {
    name: OsString,
    metadata: fs::Metadata,
}

impl fmt::Display for FileInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{:?}}}{{{} bytes}}", self.name, self.metadata.len())
    }
}

fn main() -> std::io::Result<()> {
    //  current route

    let path = env::current_dir()?;

    //  list folder and file in current route

    let mut items = Vec::new();

    for entry in fs::read_dir("./")? {
        let entry = entry?;
        let fl_md = entry.metadata()?;
        let fl_n = entry.file_name();

        items.push(FileInfo {
            name: fl_n,
            metadata: fl_md,
        });
    }

    for item in &items {
        println!("{}", item);
    }

    Ok(())
}
