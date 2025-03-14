// use serde_json::Value;
use actix_web::http::header::ContentDisposition;


// pub fn parse_json_from_vector(selected_project: &mut Vec<String>) -> Option<Value> {
//     // Check if the 4th element exists
//     if let Some(stringified_json) = selected_project.get(4) {
//         match serde_json::from_str::<Value>(stringified_json) {
//             Ok(json_value) => Some(json_value),
//             Err(err) => {
//                 eprintln!("Failed to parse JSON: {}", err);
//                 None
//             }
//         }
//     } else {
//         eprintln!("4th element does not exist in the vector.");
//         None
//     }
// }



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

