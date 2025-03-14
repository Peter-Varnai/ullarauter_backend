use sqlx::{Pool, Sqlite};
use serde::Deserialize;


// THE USE THE current_proj ELEMENT FULFILLS //
//  SHOULD BE REPLACED BY A JAVASCRIPT LOGIC!!! //

// FLASK HTML FIELDS I GOT RID OF:
// - sidebar_config
//  - all_exhibitions


pub struct AppState {
    pub db: Pool<Sqlite>,
}


// #[derive(Serialize)]
// pub struct SidebarConfig {
//     pub all_projects: Vec<Vec<String>>,
//     pub base_url: String,
//     pub current_exhib: Vec<Vec<String>>,
//     pub language: String,
//     pub admin: bool,
// }


#[derive(Deserialize)]
pub struct AdminQuery {
    pub edit_project: Option<String>,
}


#[derive(Deserialize)]
pub struct SetLangRequest {
    pub(crate) language: String,
}


#[derive(Deserialize)]
pub struct DeleteExhibitionRequest {
    pub(crate) id: String,
}

#[derive(Deserialize)]
pub struct DeleteBackgroundRequest {
    pub(crate) id: String,
}

#[derive(Deserialize)]
pub struct DeleteProjectRequest {
    pub(crate) id: String,
}
