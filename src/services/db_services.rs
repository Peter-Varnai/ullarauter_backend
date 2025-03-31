use crate::services::{extract_row, extract_rows};
use sqlx::{Pool, Sqlite};


pub async fn db_get_projects(db: &Pool<Sqlite>) -> Vec<Vec<String>> { 

    let projects_query = sqlx::query("SELECT * FROM projects")
        .fetch_all(db)
        .await
        .expect("Failed to fetch projects from db");
    let projects_columns = vec!["title_eng", "text_eng", "text_de", "video", "pictures",
                                "title_de", "pictures_folder", "date", "id"];
    // 0:"title_eng", 1:"text_eng", 2:"text_de", 3:"video", 4:"pictures",
    // 5:"title_de", 6:"pictures_folder", 7:"date", 8: "id"
    extract_rows(projects_query, projects_columns)
}


pub async fn db_get_project(db: &Pool<Sqlite>, id: String) -> Vec<String> { 

    let projects_query = sqlx::query("SELECT * FROM projects WHERE id = $1")
        .bind(id)
        .fetch_one(db)
        .await
        .expect("Failed to fetch projects from db");
    let projects_columns = vec!["title_eng", "text_eng", "text_de", "video", "pictures",
                                "title_de", "pictures_folder", "date", "id"];
    // 0:"title_eng", 1:"text_eng", 2:"text_de", 3:"video", 4:"pictures",
    // 5:"title_de", 6:"pictures_folder", 7:"date", 8: "id"
    extract_row(projects_query, projects_columns)
}


pub async fn db_get_fp_projects(db: &Pool<Sqlite>) -> Vec<Vec<String>> {
   
    let front_pate_query = sqlx::query("SELECT * FROM front_page_projects")
        .fetch_all(db)
        .await
        .expect("Failed to fetch front page projects from db");
    let front_page_columns = vec!["image", "title_eng", "title_de", "pictures_folder"];
                 // 0:"image", 1:"title_eng", 2:"title_de", 3:"pictures_folder"
    extract_rows(front_pate_query, front_page_columns)
}


pub async fn db_get_details(db: &Pool<Sqlite>) -> Vec<String> {
    let p_details_query = sqlx::query(
        "SELECT bio_eng, bio_de, pfp_address, cv_eng_address, cv_de_address FROM personal_details")
        .fetch_all(db)
        .await
        .expect("Failed to fetch personal details from db");
    let p_details_columns = vec!["bio_eng", "bio_de", "pfp_address",
                                 "cv_eng_address", "cv_de_address"];
             // 0:bio_eng, 1:bio_de, 2:pfp_address, 3:cv_eng_address, 4:cv_de_address
    extract_rows(p_details_query, p_details_columns).remove(0)
}


pub async fn db_get_comments(db: &Pool<Sqlite>, selected_project: &str) -> Vec<Vec<String>> {
    let image_comments_query = sqlx::query("SELECT file_name, eng_comment, de_comment FROM \
        image_comments WHERE picture_folder = $1")
                               .bind(selected_project) 
                               .fetch_all(db)
                               .await
                               .expect("Failed to fetch projects from db");
    let comment_columns = vec!["file_name", "eng_comment", "de_comment"];
    // 0:"id", 1:"picture_folder", 2:"file_name", 3:"eng_comment", 4:"de_comment",
    extract_rows(image_comments_query, comment_columns)
}


pub async fn db_get_exhibitions(db: &Pool<Sqlite>) -> Vec<Vec<String>> { 
    let exhibitions_query = sqlx::query("SELECT title, fromm, till, link , id FROM exhibitions")
        .fetch_all(db)
        .await
        .expect("Failed to fetch exhibitions from db");
    let exhib_columns = vec!["title", "fromm", "till", "link", "id"];
    // 0:"title", 1:"fromm", 2:"till" 3:"link" 4:"id"
    extract_rows(exhibitions_query, exhib_columns)
}

