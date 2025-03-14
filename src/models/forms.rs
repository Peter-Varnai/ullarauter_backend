use serde::{Serialize, Deserialize};


// ADMIN EDITS PATHS
// edit sites


#[derive(serde::Deserialize)]
pub struct LoginForm {
    pub password: String,
}


#[derive(Deserialize, Serialize)]
pub struct BioForm {
    pub bio_eng: Option<String>,
    pub bio_de: Option<String>,
}


#[derive(Deserialize, Serialize)]
pub struct CvUploadForm {
    pub cv_eng_address: String,
    pub cv_de_address: String,
}


#[derive(Deserialize, Serialize)]
pub struct ExhibitionForm {
    pub title_eng: String,
    pub title_de: String,
    pub from: Option<String>,  // Adjust to date type if necessary
    pub till: Option<String>,
    pub location: Option<String>,
    pub link: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub struct ImgCommentForm {
    pub folder: String,
    pub file: String,
    pub eng_comment: String,
    pub de_comment: String,
}


