use std::{io::Error, path::PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub task: String,
    pub budget: String,
    pub hours: u8,
}

pub struct Command {
    pub identifier: String,
    pub arguments: Vec<String>,
}

pub fn clear() {
    let path = cache_file();
    if path.exists() {
        match std::fs::File::create(&path) {
            Ok(_) => log_info("Cleared successfully"),
            Err(err) => log_error(err.to_string().as_str()),
        }
    }
}

pub fn write(entries: Vec<Entry>) {
    let writer = ensure_cache_exists().and_then(|path| Ok(csv::Writer::from_path(path).unwrap()));
    match writer {
        Ok(mut it) => {
            for entry in entries {
                match it.serialize(entry) {
                    Ok(_) => {}
                    Err(err) => log_error(err.to_string().as_str()),
                };
            }
            match it.flush() {
                Ok(_) => (),
                Err(err) => log_error(err.to_string().as_str()),
            };
        }
        Err(err) => log_error(err.to_string().as_str()),
    }
}

pub fn ensure_cache_exists() -> Result<PathBuf, Error> {
    let path = cache_file();
    if path.exists() {
        return Ok(path);
    }
    match std::fs::File::create(&path) {
        Ok(_) => Ok(path),
        Err(err) => Err(err),
    }
}

fn cache_file() -> PathBuf {
    let executable =
        std::env::current_exe().expect("Can't acquire current directory to write file");
    executable.parent().unwrap().join("hours.txt")
}

pub fn read() -> Vec<Entry> {
    let mut read = Vec::new();
    let reader = ensure_cache_exists()
        .and_then(|path| Ok(csv::Reader::from_path(path).expect("Can't find file")));
    match reader {
        Ok(mut reader) => {
            for result in reader.deserialize() {
                match result {
                    Ok(it) => {
                        let entry: Entry = it;
                        read.push(entry);
                    }
                    Err(err) => log_error(err.to_string().as_str()),
                }
            }
        }
        Err(err) => log_error(err.to_string().as_str()),
    }
    return read;
}

pub fn log_error(message: &str) {
    println!("error: {}", message);
}

pub fn log_info(message: &str) {
    println!("info: {}", message);
}
