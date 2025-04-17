// use serde_json::Value;
use actix_web::http::header::ContentDisposition;
use std::{fs::File, io::{BufReader, BufRead}, env, collections::HashMap};


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


pub fn load_local_env_file() {
    if env::current_exe().unwrap().ends_with("release") {
        return
    }

    let file = File::open(".env").expect(".env file not found");
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


pub fn config() -> HashMap<String, String> {

    if cfg!(debug_assertions) {
        // running in debug/developer mode
        load_local_env_file();
    }
    
    let mut config = HashMap::new();

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("8080".to_string());
    println!("app running at {}:{}", host, port);
    config.insert("host".to_string(), host);
    config.insert("port".to_string(), port);

    let db_addr = env::current_dir().unwrap().join("db/ulla.db");
    let db_url = format!("sqlite://{}", db_addr.display());
    println!("connecting to db on the following address: {}", db_url);
    config.insert("db_url".to_string(), db_url);

    let password = env::var("ADMIN_PASSWORD").expect("failed to fetch admin password from enviroment variable.");
    config.insert("password".to_string(), password);
    
    let static_path = env::var("STATIC_PATH").expect("failed to fetch enviroment variable");
    println!("static path: {}", static_path);
    config.insert("static_path".to_string(), static_path);

    config
}
