//  check current route     [X]
//
//  list folder and file    [X]
//  in current route

use std::{env, fs};

fn main() -> std::io::Result<()> {
    //  current route

    let path = env::current_dir()?;
    println!("{}", path.display());

    //  list folder and file in current route

    let mut items = Vec::new();

    for entry in fs::read_dir("./")? {
        let entry = entry?;
        let fl_n = entry.file_name();
        let fl_n_str = fl_n.to_string_lossy().into_owned();

        items.push(fl_n_str);
    }

    for item in 0..items.len() {
        println!("{}", items[item]);
    }

    Ok(())
}
