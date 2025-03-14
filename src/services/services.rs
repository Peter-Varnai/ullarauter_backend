use std::collections::HashMap;
use crate::helpers::{return_fieldnames, return_filename};
use actix_web::HttpResponse;
use serde_json::to_string;
use crate::models::AppState;
use std::fs::{remove_dir_all, File};
use std::path::Path;
use sqlx::{sqlite::SqliteRow, Row};
use actix_multipart::{Field, Multipart};
use futures_util::stream::TryStreamExt;
use std::io::Write;
use futures_util::StreamExt;


// TODO: save a query by keeping in memory the highest IDs of each db table
pub async fn new_id(
    state: &AppState,
    table: &str,
) -> i32 {
    // Query to fetch all IDs from the database
    let query = format!("SELECT id FROM {}", table);

    let ids: Vec<String> = sqlx::query(&*query)
        .fetch_all(&state.db)
        .await
        .expect("Couldn't fetch ID column from db")
        .iter()
        .map(|row| row.get::<String, _>("id"))
        .collect();

    // Convert IDs to numbers and find the maximum
    ids.iter()
        .filter_map(|id| id.parse::<i32>().ok())     
        .max()                                      
        .map(|max_id| max_id + 1)                  
        .unwrap_or(0)                             
}


pub async fn process_multiform(
    mut payload: Multipart,
    upload_dir: String
) -> HashMap<String, String> {
    let mut filenames = Vec::new();
    let mut text_fields: HashMap<String, String> = HashMap::new();
    std::fs::create_dir_all(&upload_dir).expect("failed to create folder");


    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition = field.content_disposition().cloned();

        if let Some(disposition) = &content_disposition {
            let field_name = return_fieldnames(&content_disposition);
            if disposition.get_filename().is_some() {

                upload_file(&upload_dir, &mut filenames, field).await;

            } else {

                text_fields.insert(field_name, extract_text_data(&mut field).await);

            }
        }
    }

    text_fields.insert("filenames".to_string(), to_string(&filenames).unwrap());

    text_fields
}


pub async fn upload_file(upload_dir: &String, filenames_arr: &mut Vec<String>, mut field: Field) {
    let content_disposition = field.content_disposition().cloned();
    let filename = return_filename(content_disposition);

    // Push the filename into the vector
    filenames_arr.push(filename.clone());

    let file_path = Path::new(&upload_dir).join(&filenames_arr.last().unwrap());

    let mut file = File::create(&file_path).unwrap();
    while let Some(chunk) = field.next().await {
        file.write_all(&chunk.unwrap()).unwrap();
    }
}



pub async fn delete_entry(
    db: &sqlx::SqlitePool,
    table: &str,
    id: &str,
) -> HttpResponse {
    let query = format!("DELETE FROM {} WHERE id = ?", table);
    
    println!("{}", &query);
    match sqlx::query(&query)
        .bind(id)
        .execute(db)
        .await
    {
        Ok(_) => HttpResponse::Found().append_header(("Location", "/admin")).finish(),
        Err(_) => HttpResponse::InternalServerError().body(format!("Failed to delete entry from {}", table)),
    }
}


pub fn delete_file(
    path: String
) -> HttpResponse {
    let path = format!("./static{}", path);
    println!("removing file: {}", path);
    match std::fs::remove_file(&path) {
        Ok(_) => HttpResponse::Found().append_header(("Location", "/admin")).finish(),
        Err(_) => HttpResponse::InternalServerError().body(format!("Failed to delete file {}", path)),
    }
}


pub fn delete_folder(path:String) -> HttpResponse {
    println!("deleting folder: {}", path);
    match remove_dir_all(&path) {
        Ok(_) => HttpResponse::Found().append_header(("Location", "/admin")).finish(),
        Err(_) => HttpResponse::InternalServerError().body(format!("Failed to delete folder: {}", path))
    }
}


pub async fn extract_text_data(field: &mut Field) -> String {
    let mut text_data = Vec::new();
    while let Some(chunk) = field.next().await {
        text_data.extend_from_slice(&chunk.unwrap());
    }
    String::from_utf8_lossy(&text_data).to_string()
}


pub fn extract_rows(
    raw_data: Vec<SqliteRow>,
    keys: Vec<&str>,
) -> Vec<Vec<String>> {
    let extracted_data: Vec<Vec<String>> = raw_data
        .iter()
        .map(|row| {
            let mut extract: Vec<String> = Vec::new();

            for key in &keys {
                extract.push(row.get(key));
            }
            extract
        }).collect();
    extracted_data
}


// pub fn extract_row(
//     raw_data: SqliteRow,
//     keys: Vec<&str>,
// ) -> Vec<String> {
//     let mut extract: Vec<String> = Vec::new();
//
//     for key in &keys {
//         extract.push(raw_data.get(key));
//     }
//     extract
// }


