// use serde_json::Value;
use actix_web::http::header::ContentDisposition;
use std::{fs::File, io::{BufReader, BufRead}, env};

pub fn return_fieldnames(
    content_disposition: &Option<ContentDisposition>
) -> String {
    content_disposition.clone()
        .and_then(|cd| cd.get_name().map(|name| name.to_string()))
        .unwrap_or_else(|| "unknown".to_string())
}


pub fn return_filename
(content_disposition: Option<ContentDisposition>
) -> String {
    content_disposition.unwrap()
        .get_filename()
        .map(String::from)
        .unwrap_or_else(|| "default_filename".to_string())
}



pub fn format_date(date: &String) -> String {
    format!("{}.{}.{}", &date[..4], &date[4..6], &date[6..])
}


pub fn load_env_file(path: &str) {
    let file = File::open(path).expect(".env file not found");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if line.trim().is_empty() || line.starts_with('#') {
            continue; // Skip empty lines/comments
        }
        if let Some((key, value)) = line.split_once('=') {
            env::set_var(key.trim(), value.trim());
        }
    }
}
