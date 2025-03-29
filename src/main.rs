//  detect folders  []

use std::{fmt, fs};

struct FileInfo {
    name: String,
    metadata: fs::Metadata,
}

impl fmt::Display for FileInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:<20} {:>8} bytes", self.name, self.metadata.len())
    }
}

fn main() -> std::io::Result<()> {
    //  current route
    //let path = std::env::current_dir()?;

    //  list folder and file in current route

    let mut items = Vec::new();

    for entry in fs::read_dir("./")? {
        let entry = entry?;
        let file_metadata = entry.metadata()?;
        let file_name = entry.file_name().into_string().unwrap();

        items.push(FileInfo {
            name: file_name,
            metadata: file_metadata,
        });
    }

    for item in &items {
        println!("{}", item);
    }

    Ok(())
}
