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

pub(crate) fn get_white_list() -> Option<Vec<String>> {
    let args = Args::parse();
    let file_path = args.white_list_file;
    if let Some(file) = file_path {
        match read_lines_from_file(&file) {
            Ok(lines) => {
                log::debug!("Entries in whitelist file");
                for line in &lines {
                    log::debug!("- {}", line);
                }

                return Some(lines);
            }
            Err(e) => {
                log::error!("error reading file: {}", e);
                None
            }
        }
    } else {
        log::debug!("no whitelist");
        return None;
    }
}
