use askama::Template;
use serde::Serialize;
// use crate::models::SidebarConfig;


#[derive(Template, Serialize)]
#[template(path="sidebar_exhibs.html")]
pub struct SidebarExhibs {
    pub current_exhib: Vec<Vec<String>>,    
}

#[derive(Template, Serialize)]
#[template(path="sidebar_eng.html")]
pub struct SidebarElement_eng_html<'a> {
    pub all_projects: &'a Vec<Vec<String>>,
    pub base_url: &'a String,
    pub current_exhib: &'a String,
}


#[derive(Template, Serialize)]
#[template(path="sidebar_de.html")]
pub struct SidebarElement_de_html<'a> {
    pub all_projects: &'a Vec<Vec<String>>,
    pub base_url: &'a String,
    pub current_exhib: &'a String,
}


#[derive(Template, Serialize)]
#[template(path = "index.html")]
pub struct Index<'a> {
    pub sidebar_html: &'a String,
    // pub current_exhib: Vec<Vec<String>>,
    pub fp: &'a Vec<String>,
    pub fp_url: String,
}


#[derive(Template, Serialize)]
#[template(path = "contact.html")]
pub struct Contact<'a> {
    pub sidebar_html: &'a String,
    pub cv_address_eng: &'a String,
    pub cv_address_de: &'a String,
    pub lang: String,
    pub base_url: String, 
}


#[derive(Template, Serialize)]
#[template(path="bio_exhibs.html")]
pub struct Bio_Exhibs_html<> {
    pub all_exhibitions: Vec<Vec<Vec<String>>>,
}


#[derive(Template, Serialize)]
#[template(path = "bio.html")]
pub struct Bio<'a> {
    pub sidebar_html: &'a String,
    pub exhibs_html: &'a String,
    // pub all_exhibitions: Vec<Vec<Vec<String>>>,
    pub bio_eng: &'a String,
    pub bio_de: &'a String,
    pub pfp_url: &'a String,
    pub lang: String,
    // pub base_url: String, 
}


#[derive(Template, Serialize)]
#[template(path = "project.html")]
pub struct Project<'a> {
    pub sidebar_html:&'a String,
    pub project_title: String,
    pub image_urls: Vec<String>,
    pub image_comments: Vec<Vec<String>>,
    pub current_proj: Vec<String>,
    pub lang: String,
}


#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate;


#[derive(Template, Serialize)]
#[template(path = "admin.html")]
pub struct AdminTemplate {
    pub projects: Vec<Vec<String>>,
    pub exhibitions: Vec<Vec<String>>,
    pub backgrounds: Vec<Vec<String>>,
    pub edit_project: Vec<String>,
    pub personal_details: Vec<String>,
    pub img_comm: Vec<Vec<String>>,
    // pub comments: Vec<Vec<String>>,
}


