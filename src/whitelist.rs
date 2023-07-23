use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

use crate::app_init::Args;

fn read_lines_from_file(file_path: &str) -> Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line?);
    }

    Ok(lines)
}

pub(crate) fn get_white_list() -> Result<Vec<String>> {
    let args = Args::parse();
    let file_path = args.white_list_file;

    if !file_path.is_empty() {
        log::error!("{}", file_path);
    }

    match read_lines_from_file(&file_path) {
        Ok(lines) => {
            // Now you have a Vec<String> containing each line as a separate string.
            // You can work with the `lines` vector here.
            for line in &lines {
                println!("{}", line);
            }
            
            return Ok(lines);
        }
        Err(e) => Err(e)
    }
}
