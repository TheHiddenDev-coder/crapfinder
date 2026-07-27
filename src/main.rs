#![allow(unused, unused_mut)]

use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
    process,
};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage:");
        eprintln!("crapfinder <path> <\"text\">");
        process::exit(1);
    }

    let file_path = &args[1];
    let pattern = &args[2];

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        let line = line?;

        if line.to_lowercase().contains(&pattern.to_lowercase()) {
            println!("{}:{} <> {}", file_path, i + 1, line);
        }
    }
    Ok(())
}
