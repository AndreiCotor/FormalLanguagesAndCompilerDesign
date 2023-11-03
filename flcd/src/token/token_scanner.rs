use std::fs::{File};
use std::io::{BufRead, BufReader};

pub fn scan_token_file(file_name: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(file_name).unwrap();
    let buf_reader = BufReader::new(file);

    buf_reader.lines().collect()
}