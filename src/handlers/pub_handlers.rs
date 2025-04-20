#[warn(unreachable_code)]
#[warn(unused_assignments)]
use actix_web::{Responder,  get, post, HttpResponse,
            web::{Data, Path, Json, Query}};
use crate::models::AppState;
use serde_json::{from_str, json};
use rand::thread_rng;
use crate::{models::{Index, AdminQuery, LoginTemplate, AdminTemplate, SetLangRequest,
            Bio, Project, Contact},
            services::{db_get_projects, db_get_fp_projects,
            db_get_details, db_get_comments, db_get_project, db_get_exhibitions},            
            cache::{SIDEBAR_LOCK, BIO_EXHIBS_LOCK},
            errors::HandlerError, helpers::filter_empty_strings,
            };
use actix_identity::Identity;
use actix_session::Session;
use rand::Rng;
use askama::Template;



#[get("/")]
pub async fn index(
    session: Session, 
    state: Data<AppState>
    ) -> Result<HttpResponse, HandlerError> {
    let front_page_proj = db_get_fp_projects(&state.db).await;

    let lang = session.get::<String>("lang").unwrap().unwrap_or("eng".to_string());
    let sidebar_html = &SIDEBAR_LOCK.read().unwrap()[&lang];

    let fp_picture = if front_page_proj.len() > 1 {
        let mut rng = thread_rng();
        let i = rng.gen_range(1..(front_page_proj.len() - 1));
        &front_page_proj[i]
    } else {
        &front_page_proj[0]
    };

    let index_template = Index {
        sidebar_html, 
        fp: fp_picture,
        fp_url: format!("/static/front_pages/{}/{}", fp_picture[3], fp_picture[0]),
    };
    
    let html = index_template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}


#[get("/contact")]
pub async fn contact(
    session: Session,
    state: Data<AppState>
    ) -> Result<HttpResponse, HandlerError> {

    let p_details_future = db_get_details(&state.db);
    
    let lang = session.get::<String>("lang").unwrap().unwrap_or("eng".to_string());
    let sidebar_html = &SIDEBAR_LOCK.read().unwrap()[&lang];


    let p_details = p_details_future.await; 

    let cv_address_eng = &p_details[3];
    let cv_address_de = &p_details[4];

    let contact_template = Contact {
        // base_url: &state.url,
        sidebar_html,
        cv_address_eng ,
        lang,
        cv_address_de,
    };

    let html = contact_template.render()?;
    
    Ok(HttpResponse::Ok().body(html))
}


#[get("/bio")]
pub async fn bio(
    session: Session,
    state: Data<AppState>
) ->  Result<HttpResponse, HandlerError> {

    let p_details_future = db_get_details(&state.db);
    let exhibs_html = &BIO_EXHIBS_LOCK.read().unwrap();

    let lang = session.get::<String>("lang").unwrap().unwrap_or("eng".to_string());
    let sidebar_html = &SIDEBAR_LOCK.read().unwrap()[&lang];

    let p_details = p_details_future.await; 
    
    let bio_template = Bio {
        sidebar_html,
        lang,
        exhibs_html,
        bio_eng: &p_details[0],
        bio_de: &p_details[1],
        pfp_url: &p_details[2],
    };

    let html = bio_template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}


#[get("/project/{project}")]
pub async fn project(
    session: Session, 
    project: Path<String>, 
    state: Data<AppState>
) -> Result<HttpResponse, HandlerError> {
    let id = project.into_inner();
  
    let lang = session.get::<String>("lang").unwrap().unwrap_or("eng".to_string());
    let sidebar_html = &SIDEBAR_LOCK.read().unwrap()[&lang];

    let selected_project = db_get_project(&state.db, id).await;
    let images: Vec<String> = from_str(&selected_project[4])?;
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

    let pic_urls: Vec<String> = from_str(&*selected_project[4])?;

    let checked_urls = filter_empty_strings(pic_urls);

    for url in &checked_urls {
        println!("pic urls:{}", url);
    }

    
    let project_template = Project {
        sidebar_html,
        lang,
        project_title: "Test".to_string(),
        image_urls: checked_urls,
        image_comments: img_comm,
        current_proj: selected_project.clone(),
    };

    // TODO: currently project urls can contain spaces, a mechanism
    // need to be implemented that replaces spaces with "_" and
    // without breaking the project querying
    
    let html = project_template.render()?;

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}
 

#[post("/set_language")]
async fn set_language(
    lang: Json<SetLangRequest>, 
    session: Session
) -> HttpResponse {
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
) -> Result<HttpResponse, HandlerError> {
    
    if identity.is_none() {
        let template = LoginTemplate;
        let login_rendered = template.render()?;
        return Ok(HttpResponse::Ok().body(login_rendered));
    }

    let mut edit_project = vec!["[\"empty\"]".to_string(); 9];
    let mut img_comm = vec![vec!["".to_string(); 5]; 8];

    if let Some(id) = &query.edit_project {
        edit_project = db_get_project(&state.db, id.to_string()).await;
        let comments = db_get_comments(&state.db, &edit_project[6]).await;
        let images: Vec<String> = from_str(&edit_project[4]).expect("failed to convert to json");
        let checked_images = filter_empty_strings(images);
        img_comm = Vec::new();

        for img in &checked_images {
            let mut found = false; // Track if any comment was found for the image
            for comm in &comments {
                if img == &comm[0] {
                    img_comm.push(comm.clone());
                    found = true; // Mark that a comment was found
                }
            }

            if !found {
                img_comm.push(vec![img.clone(), "".to_string(), "".to_string()]);
            }
        }
    }

    let projects = db_get_projects(&state.db).await;
    let exhibitions = db_get_exhibitions(&state.db).await;
    let backgrounds = db_get_fp_projects(&state.db).await;
    let personal_details = db_get_details(&state.db).await;

    let template = AdminTemplate {
        projects,
        exhibitions,
        backgrounds,
        edit_project,
        personal_details,
        img_comm,
    };

    let admin_rendered = template.render()?;

    Ok(HttpResponse::Ok().body(admin_rendered))
}
