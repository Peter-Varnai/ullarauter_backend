#[warn(unreachable_code)]
#[warn(unused_assignments)]
use actix_web::{Responder,  get, post, HttpResponse, web::{Data, Path, Json, Query}};
use sqlx::Row;
use crate::models::AppState;
use serde_json::{from_str, json};
use rand::thread_rng;
use crate::{models::{Index, AdminQuery, LoginTemplate, AdminTemplate, SetLangRequest,
    Bio, Project, Contact},
            services::{extract_rows, db_get_projects, db_get_fp_projects,
            db_get_details, db_get_comments},            
            cache::{SIDEBAR_LOCK, BIO_EXHIBS_LOCK},
            };
use std::env;
use actix_identity::{Identity};
use actix_session::Session;
use rand::Rng;
use askama::Template;



#[get("/")]
pub async fn index(session: Session, state: Data<AppState>) -> impl Responder {

    let front_page_proj = db_get_fp_projects(&state.db).await;

    let mut sidebar_html = String::new();
    if let Some(lang) = session.get::<String>("lang").unwrap() {
        if lang == "eng" {sidebar_html = SIDEBAR_LOCK.read().unwrap()["eng"].to_string();}
        else {sidebar_html = SIDEBAR_LOCK.read().unwrap()["de"].to_string();}
    } else {
        session.insert("lang", "eng");
        sidebar_html = SIDEBAR_LOCK.read().unwrap()["eng"].to_string();
    }

    let fp_picture: &Vec<String>;

    if front_page_proj.len() > 1 {
        let mut rng = thread_rng();
        let i = rng.gen_range(1..(front_page_proj.len() - 1));

        fp_picture = &front_page_proj[i];
    } else {
        fp_picture = &front_page_proj[0];
    };

    
    let index_template = Index {
        sidebar_html,
        fp: fp_picture,
        fp_url: format!("/static/front_pages/{}/{}", fp_picture[3], fp_picture[0]),
    };

    match index_template.render() {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(_) => HttpResponse::InternalServerError().body("Failed to render index template.")
    }
}


#[get("/contact")]
pub async fn contact(session: Session,state: Data<AppState>) -> impl Responder {

    let projects = db_get_projects(&state.db); 
    let p_details_future = db_get_details(&state.db);

    let mut sidebar_html = String::new();
    let mut language = String::new();
    if let Some(lang) = session.get::<String>("lang").unwrap() {
        if lang == "eng" {
            sidebar_html = SIDEBAR_LOCK.read().unwrap()["eng"].to_string();
            language = "eng".to_string();
        }
        else {
            sidebar_html = SIDEBAR_LOCK.read().unwrap()["de"].to_string();
             language = "de".to_string();
        }
    } else {
        session.insert("lang", "eng");
        language = "eng".to_string();
        sidebar_html = SIDEBAR_LOCK.read().unwrap()["eng"].to_string();
    }

    let p_details = p_details_future.await; 

    let cv_address_eng = &p_details[3];
    let cv_address_de = &p_details[4];
    
    let contact_template = Contact {
        base_url: env::var("BASE_URL").unwrap_or("http://127.0.0.1:8080".to_string()),
        sidebar_html,
        cv_address_eng ,
        language,
        cv_address_de,
    };

    match contact_template.render() {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(_) => HttpResponse::InternalServerError().body("Failed to render contact template.")
    }
}


#[get("/bio")]
pub async fn bio(session: Session,state: Data<AppState>) -> impl Responder {

    let projects = db_get_projects(&state.db);
    let p_details_future = db_get_details(&state.db);
    let exhibs_html = BIO_EXHIBS_LOCK.read().unwrap().to_string();

    let mut sidebar_html = String::new();
    let mut language = String::new();
    if let Some(lang) = session.get::<String>("lang").unwrap() {
        if lang == "eng" {
            sidebar_html = SIDEBAR_LOCK.read().unwrap()["eng"].to_string();
            language = "eng".to_string();
        }
        else {
            sidebar_html = SIDEBAR_LOCK.read().unwrap()["de"].to_string();
             language = "de".to_string();
        }
    } else {
        session.insert("lang", "eng");
        language = "eng".to_string();
        sidebar_html = SIDEBAR_LOCK.read().unwrap()["eng"].to_string();
    }

    let p_details = p_details_future.await; 
    
    let bio_template = Bio {
        sidebar_html,
        language,
        exhibs_html,
        bio_eng: &p_details[0],
        bio_de: &p_details[1],
        pfp_url: &p_details[2],
    };

    match bio_template.render() {
        Ok(html) => HttpResponse::Ok().body(html),
        Err(_) => HttpResponse::InternalServerError().body("Failed to render bio template.")
    }
}


#[get("/project/{project}")]
pub async fn project(session: Session, project: Path<String>, state: Data<AppState>) -> impl Responder {
    let id = project.into_inner();

    let mut sidebar_html = String::new();
    let mut language = String::new();
    if let Some(lang) = session.get::<String>("lang").unwrap() {
        if lang == "eng" {
            sidebar_html = SIDEBAR_LOCK.read().unwrap()["eng"].to_string();
            language = "eng".to_string();
        }
        else {
            sidebar_html = SIDEBAR_LOCK.read().unwrap()["de"].to_string();
             language = "de".to_string();
        }
    } else {
        session.insert("lang", "eng");
        language = "eng".to_string();
        sidebar_html = SIDEBAR_LOCK.read().unwrap()["eng"].to_string();
    }


    let projects = db_get_projects(&state.db).await;
    let selected_project = projects
        .iter()
        .find(|proj| proj[8] == id)
        .expect("Project wasn't found")
        .clone();

    let images: Vec<String> = from_str(&selected_project[4]).expect("failed to convert to json");
    let comments = db_get_comments(&state.db, &selected_project[6]).await;
    let mut img_comm = Vec::new();

    for img in &images {
        let mut found = false; 
        for comm in &comments {
            if img == &comm[0] {
                img_comm.push(comm.clone());
                found = true; 
            }
        }
        if !found {
            img_comm.push(vec![img.clone(), "".to_string(), "".to_string()]);
        }
    }


    let pic_urls = from_str(&*selected_project[4]).expect("Failed to convert to JSON");


    let project_template = Project {
        sidebar_html,
        language,
        project_title: "Test".to_string(),
        image_urls: pic_urls,
        image_comments: img_comm,
        current_proj: selected_project.clone(),
    };


    // TODO: currently project urls can contain spaces, a mechanism
    // need to be implemented that replaces spaces with "_" and
    // without breaking the project querying

    match project_template.render() {
        Ok(html) => HttpResponse::Ok().content_type("text/html").body(html),
        Err(_) => HttpResponse::InternalServerError().body("Failed to render picture template.")
    }
}
 

#[post("/set_language")]
async fn set_language(lang: Json<SetLangRequest>, session: Session) -> impl Responder {
    println!("lang change called");
    session.insert("lang", lang.language.clone()).unwrap();
    HttpResponse::Ok().body("finished") 
}


#[get("/session-info")]
async fn session_info(identity: Option<Identity>, session: Session) -> impl Responder {
    
    let mut lang = String::new();
    let mut user = String::new();

    if let Some(language) = session.get::<String>("lang").unwrap() {
        lang = language;
        session.insert("lang", &lang).unwrap();
    } else {
        session.insert("lang", "eng").unwrap();
    } 

    if let Some(usr) = identity {
        user = usr.id().unwrap_or_default();
        session.insert("user", "admin").unwrap();
    } else {
        session.insert("user", "viewer").unwrap();
    }
    
    HttpResponse::Ok().json(json!({
                    "logged_in": user,
                    "language": lang
    }))
}

            
#[get("/admin")]
async fn admin(
    identity: Option<Identity>,
    state: Data<AppState>, 
    query: Query<AdminQuery>
) -> impl Responder {
    let mut edit_project = vec!["[\"empty\"]".to_string(); 9];
    let mut img_comm = vec![vec!["".to_string(); 5]; 8];


    if identity.is_none() {
        let template = LoginTemplate;
        return match template.render() {
            Ok(rendered) => return HttpResponse::Ok().body(rendered),
            Err(_) => {
                return HttpResponse::InternalServerError().finish();
            }
        }
    }

    if let Some(project_id) = &query.edit_project {
        let projects_query = sqlx::query("SELECT * FROM projects WHERE id = $1")
            .bind(&project_id)
            .fetch_one(&state.db)
            .await
            .expect("Failed to fetch projects from db");
        let projects_columns = vec!["title_eng", "text_eng", "text_de", "video", "pictures",
        "title_de", "pictures_folder", "date", "id"];
        // 0:"title_eng", 1:"text_eng", 2:"text_de", 3:"video", 4:"pictures",
        // 5:"title_de", 6:"pictures_folder", 7:"date", 8: "id"


        // TODO: the image comments array contains at the moment all the picture file names and the
        // comments, this is not working if there are no comments assigned to a picture yet!!!
       
        let mut image_comments_folder = String::new(); 
        edit_project = Vec::new();
        for key in &projects_columns {
            let value: String = projects_query.get(key);
            if *key == "pictures_folder" {
                image_comments_folder = value.clone()
            }
            edit_project.push(value);
        }


        let image_comments_query = sqlx::query("SELECT file_name, eng_comment, de_comment FROM \
        image_comments WHERE picture_folder = $1")
                                   .bind(image_comments_folder) 
                                   .fetch_all(&state.db)
                                   .await
                                   .expect("Failed to fetch projects from db");
        let comment_columns = vec!["file_name", "eng_comment", "de_comment"];
        // 0:"id", 1:"picture_folder", 2:"file_name", 3:"eng_comment", 4:"de_comment",
        let comments = extract_rows(image_comments_query, comment_columns);


        let images: Vec<String> = from_str(&edit_project[4]).expect("failed to convert to json");

        img_comm = Vec::new();



        for img in &images {
            let mut found = false; // Track if any comment was found for the image
            for comm in &comments {
                if img == &comm[0] {
                    img_comm.push(comm.clone());
                    found = true; // Mark that a comment was found
                }
            }
            // After checking all comments, add an empty entry if none were found
            if !found {
                img_comm.push(vec![img.clone(), "".to_string(), "".to_string()]);
            }
        }
    }

    let projects_query = sqlx::query("SELECT title_eng, id FROM projects")
        .fetch_all(&state.db)
        .await
        .expect("Failed to fetch projects from db");
    let projects_columns = vec!["title_eng", "id"];
    // 0:"title_eng", 1:"text_eng", 2:"text_de", 3:"video", 4:"pictures",
    // 5:"title_de", 6:"pictures_folder", 7:"date", 8: "id"
    let projects = extract_rows(projects_query, projects_columns);


    let front_page_query = sqlx::query("SELECT * FROM front_page_projects")
        .fetch_all(&state.db)
        .await
        .expect("Failed to fetch front page projects from db");
    let front_page_columns = vec!["image", "title_eng", "title_de", "pictures_folder", "id"];
    // 0:"image", 1:"title_eng", 2:"title_de", 3:"pictures_folder", 4:"id"
    let mut backgrounds = extract_rows(front_page_query, front_page_columns);
    backgrounds.drain(..1);


    let exhibitions_query = sqlx::query("SELECT title, fromm, till, link , id FROM exhibitions")
        .fetch_all(&state.db)
        .await
        .expect("Failed to fetch exhibitions from db");
    let exhib_columns = vec!["title", "fromm", "till", "link", "id"];
    // 0:"title", 1:"fromm", 2:"till" 3:"link" 4:"id"
    let exhibitions: Vec<Vec<String>> = extract_rows(exhibitions_query, exhib_columns);


    let p_details_query = sqlx::query("SELECT bio_eng, bio_de, pfp_address, \
        cv_eng_address, cv_de_address FROM personal_details")
                          .fetch_all(&state.db)
                          .await
                          .expect("Failed to fetch personal details from db");
    let p_details_columns = vec!["bio_eng", "bio_de", "pfp_address",
    "cv_eng_address", "cv_de_address"];
    // 0:bio_eng, 1:bio_de, 2:pfp_address, 3:cv_eng_address, 4:cv_de_address
    let personal_details = extract_rows(p_details_query, p_details_columns).remove(0);


    let template = AdminTemplate {
        projects,
        exhibitions,
        backgrounds,
        edit_project,
        personal_details,
        img_comm,
    };

    match template.render() {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}






















