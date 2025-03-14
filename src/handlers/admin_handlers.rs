#![allow(unreachable_code)]
use crate::{
    models::{AppState, BioForm, CvUploadForm, ExhibitionForm, DeleteExhibitionRequest, 
        DeleteProjectRequest, DeleteBackgroundRequest, ImgCommentForm, LoginForm},
    cache::{update_sidebar_cashe, update_bio_exhibs_cache, update_sidebar_exhibs_cache}};
use crate::services::{new_id, delete_folder, delete_entry, extract_rows, delete_file, process_multiform};
use actix_web::{Result, web::{Path as Ax_Path, Data, Form, Json}};
use actix_web::{HttpResponse, post, Responder, get, HttpRequest, HttpMessage};
use actix_multipart::Multipart;
use actix_identity::{Identity};
use sqlx::Row;
use crate::models::LoginTemplate;
use askama::Template;

// TODO: change password to enviroment variable!!!
//
#[post("/login")]
async fn login(form: Form<LoginForm>, request: HttpRequest) -> impl Responder {
    if form.password == "passwor" {
        Identity::login(&request.extensions(), "admin".into()).expect("Failed to log in");
        HttpResponse::Found().append_header(("Location", "/admin")).finish()
    } else {
        HttpResponse::Unauthorized().body("Invalid credentials")
    }
}


#[post("/logout")]
async fn logout(user: Identity) -> impl Responder {
    user.logout();
    println!("Log out called");
    HttpResponse::Ok().body("Logged out!")
}


#[post("/create_project")]
async fn create_project(
    state: Data<AppState>, 
    payload: Multipart,
    identity: Option<Identity>,
    ) -> impl Responder {

    if identity.is_none() {
        println!("identity not found, admin logged out");
        let template = LoginTemplate;
        return match template.render() {
            Ok(rendered) => return HttpResponse::Ok().body(rendered),
            Err(_) => {
                return HttpResponse::InternalServerError().finish()
            }
        }
    }


    // TODO: get rid of the pictures folder column in db by making the ID of the row be the name of the
    // folder where the project pictures are stored. Projects made with this route already create their pictures
    // folder using their ID.

    // TODO: Unify the format dates are stored across the backend,
    // exhibitions at the moment store dates as %Y%m%d, projects do: %Y-%m-%d

    let id = new_id(&state, "projects").await.to_string();
    println!("{}", id);
    let upload_dir: String = format!("./static/projects/{}", id);

    let form_data = process_multiform(payload, upload_dir).await;
    println!("form processed");
    match sqlx::query(
        "INSERT INTO projects (id, title_eng, title_de, text_eng, text_de, video, date, \
        pictures_folder, pictures) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)")
        .bind(&id)
        .bind(form_data.get("title_eng"))
        .bind(form_data.get("title_de"))
        .bind(form_data.get("text_eng"))
        .bind(form_data.get("text_de"))
        .bind(form_data.get("video"))
        .bind(form_data.get("date"))
        .bind(&id)
        .bind(&form_data.get("filenames"))
        .execute(&state.db)
        .await
    {
        Ok(_) => {
            update_sidebar_cashe(&state.db).await;
            HttpResponse::Found().append_header(("Location", "/admin")).finish()
        },
        Err(_) => HttpResponse::InternalServerError().finish()
    }
}


#[post("/upload_background")]
async fn upload_background(
    state: Data<AppState>, 
    payload: Multipart,
    identity: Option<Identity>,
    ) -> impl Responder {

    if identity.is_none() {
        println!("identity not found, admin logged out");
        let template = LoginTemplate;
        return match template.render() {
            Ok(rendered) => return HttpResponse::Ok().body(rendered),
            Err(_) => {
                return HttpResponse::InternalServerError().finish()
            }
        }
    }



    let id = new_id(&state, "front_page_projects").await.to_string();
    let upload_dir: String = format!("./static/front_pages/{}", id);
    println!("upload dir: {}", &upload_dir);
    let form_data = process_multiform(payload, upload_dir).await;

    let file_name = form_data.get("filenames")
        .map(|s| &s[2..s.len() - 2])
        .expect("failed to retrieve profile picture"); 

    match sqlx::query(
        "INSERT INTO front_page_projects (id, title_eng, title_de, pictures_folder, image) VALUES ($1, $2, $3, $4, $5)")
        .bind(&id)
        .bind(form_data.get("title_eng"))
        .bind(form_data.get("title_de"))
        .bind(&id)
        .bind(file_name)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Found().append_header(("Location", "/admin")).finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


#[post("/add_exhibition")]
async fn add_exhibition(
    state: Data<AppState>, 
    form: Form<ExhibitionForm>,
    identity: Option<Identity>,
    ) -> impl Responder {
  
    if identity.is_none() {
        println!("identity not found, admin logged out");
        let template = LoginTemplate;
        return match template.render() {
            Ok(rendered) => return HttpResponse::Ok().body(rendered),
            Err(_) => {
                return HttpResponse::InternalServerError().finish()
            }
        }
    }

    let form_data = form.into_inner();

    match sqlx::query(
        "INSERT INTO exhibitions (id, title, title_de, fromm, till, location, link) VALUES ($1, $2, $3, $4, $5, $6, $7)")
        .bind(new_id(&state, "exhibitions").await.to_string())
        .bind(form_data.title_eng)
        .bind(form_data.title_de)
        .bind(form_data.from.unwrap().replace("-", ""))
        .bind(form_data.till.unwrap().replace("-", ""))
        .bind(form_data.location)
        .bind(form_data.link)
        .execute(&state.db)
        .await
    {
        Ok(_) => {
            update_sidebar_exhibs_cache(&state.db).await;
            update_bio_exhibs_cache(&state.db).await;
            update_sidebar_cashe(&state.db).await;
            HttpResponse::Found().append_header(("Location", "/admin")).finish()
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


#[post("/delete_exhibition")]
async fn delete_exhibition(
    state: Data<AppState>, 
    form: Json<DeleteExhibitionRequest>,
    identity: Option<Identity>
    ) -> impl Responder {
  
    if identity.is_none() {
        println!("identity not found, admin logged out");
        let template = LoginTemplate;
        return match template.render() {
            Ok(rendered) => {
                update_sidebar_exhibs_cache(&state.db).await;
                return HttpResponse::Ok().body(rendered)},
            Err(_) => {
                return HttpResponse::InternalServerError().finish()
            }
        }
    }

    delete_entry(&state.db, "exhibitions", &form.id).await;
    update_bio_exhibs_cache(&state.db).await;

    HttpResponse::Found().append_header(("Location", "/admin")).finish()
}


#[post("/delete_background")]
async fn delete_background(
    state: Data<AppState>, 
    form: Json<DeleteBackgroundRequest>,
    identity: Option<Identity>
    ) -> impl Responder {
  
    if identity.is_none() {
        println!("identity not found, admin logged out");
        let template = LoginTemplate;
        return match template.render() {
            Ok(rendered) => return HttpResponse::Ok().body(rendered),
            Err(_) => {
                return HttpResponse::InternalServerError().finish()
            }
        }
    }

    let path = sqlx::query("SELECT pictures_folder, image FROM front_page_projects WHERE id = $1")
        .bind(&form.id)
        .fetch_one(&state.db)
        .await
        .expect("Failed to fetch background file location");

    let pictures_folder: String = path.get("pictures_folder");
    let image: String = path.get("image");

    delete_file(format!("/front_pages/{}/{}", pictures_folder, image));

    delete_entry(&state.db, "front_page_projects", &form.id).await
}


#[post("/delete_project")]
async fn delete_project(
    state: Data<AppState>, 
    form: Json<DeleteProjectRequest>,
    identity: Option<Identity>
    ) -> impl Responder {
   
    if identity.is_none() {
        println!("identity not found, admin logged out");
        let template = LoginTemplate;
        return match template.render() {
            Ok(rendered) => return HttpResponse::Ok().body(rendered),
            Err(_) => {
                return HttpResponse::InternalServerError().finish()
            }
        }
    }


    let path = sqlx::query("SELECT pictures_folder FROM projects WHERE id = $1")
        .bind(&form.id)
        .fetch_one(&state.db)
        .await
        .expect("Failed to fetch pictures_folder to delete");
    let pictures_folder: String = path.get("pictures_folder");
    let folder_path: String = format!("./static/projects/{}", pictures_folder);
    println!("project folder to delete: {}", &folder_path);
    println!("project id to delete: {}", &form.id);

    delete_folder(folder_path);
    
    delete_entry(&state.db, "projects", &form.id).await;

    HttpResponse::Found().append_header(("Location", "/admin")).finish()
} 


#[post("/update_profile_picture")]
async fn update_pfp(
    state: Data<AppState>, 
    payload: Multipart,
    identity: Option<Identity>
    ) -> impl Responder {
   
    if identity.is_none() {
        println!("identity not found, admin logged out");
        let template = LoginTemplate;
        return match template.render() {
            Ok(rendered) => return HttpResponse::Ok().body(rendered),
            Err(_) => {
                return HttpResponse::InternalServerError().finish()
            }
        }
    }

    let pfp_query = sqlx::query("SELECT pfp_address FROM personal_details")
        .fetch_one(&state.db)
        .await
        .expect("Failed to fetch projects from db");
    let pfp_address: String = pfp_query.get("pfp_address");

    let path = format!("/personal_details/{}", pfp_address);
    delete_file(path);
    
    let upload_dir: String = format!("./static/personal_details");
    let form_data = process_multiform(payload, upload_dir).await;
    
    let file_name = form_data.get("filenames")
        .map(|s| &s[2..s.len() - 2])
        .expect("failed to retrieve profile picture");    

    match sqlx::query("UPDATE personal_details SET pfp_address = $1 WHERE id = 1")
        .bind(file_name)
        .execute(&state.db)
        .await
    {
        Ok(_) => {
            update_sidebar_cashe(&state.db);
            HttpResponse::Found().append_header(("Location", "/admin")).finish()
        },
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


#[post("/update_cv")]
async fn update_cv(
    form: Form<CvUploadForm>, 
    state: Data<AppState>,
    identity: Option<Identity>
    ) -> impl Responder {
 
    if identity.is_none() {
        println!("identity not found, admin logged out");
        let template = LoginTemplate;
        return match template.render() {
            Ok(rendered) => return HttpResponse::Ok().body(rendered),
            Err(_) => {
                return HttpResponse::InternalServerError().finish()
            }
        }
    }

    let form_data = form.into_inner();

    match sqlx::query("UPDATE personal_details SET cv_eng_address = $1, cv_de_address = $2 WHERE id = 1")
        .bind(form_data.cv_eng_address)
        .bind(form_data.cv_de_address)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Found().append_header(("Location", "/admin")).finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


#[post("/update_bio")]
async fn update_bio(
    form: Form<BioForm>, 
    state: Data<AppState>,
    identity: Option<Identity>
    ) -> impl Responder {
    
    if identity.is_none() {
        println!("identity not found, admin logged out");
        let template = LoginTemplate;
        return match template.render() {
            Ok(rendered) => return HttpResponse::Ok().body(rendered),
            Err(_) => {
                return HttpResponse::InternalServerError().finish()
            }
        }
    }

    let form_data = form.into_inner();

    match sqlx::query("UPDATE personal_details SET bio_eng = $1, bio_de = $2 WHERE id = 1")
        .bind(form_data.bio_eng)
        .bind(form_data.bio_de)
        .execute(&state.db)
        .await
    {
        Ok(_) => HttpResponse::Found().append_header(("Location", "/admin")).finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


// TODO: Consider always returning json serialized data
#[get("/get_project/{project_id}")]
async fn get_project(
    project_id: Ax_Path<String>, 
    state: Data<AppState>,
    identity: Option<Identity>
    ) -> impl Responder {
  
    if identity.is_none() {
        println!("identity not found, admin logged out");
        let template = LoginTemplate;
        return match template.render() {
            Ok(rendered) => return HttpResponse::Ok().body(rendered),
            Err(_) => {
                return HttpResponse::InternalServerError().finish()
            }
        }
    }

    let projects_query = sqlx::query("SELECT * FROM projects")
        .fetch_all(&state.db)
        .await
        .expect("Failed to fetch projects from db");
    let projects_columns = vec!["title_eng", "text_eng", "text_de", "video", "pictures",
                                "title_de", "pictures_folder", "date", "id"];
    // 0:"title_eng", 1:"text_eng", 2:"text_de", 3:"video", 4:"pictures",
    // 5:"title_de", 6:"pictures_folder", 7:"date", 8: "id"
    let projects = extract_rows(projects_query, projects_columns);

    let project_id = project_id.into_inner();
    let project = projects.iter().find(|p| p[8] == project_id);

    HttpResponse::Ok().json(project)
}


#[post("/add_comment")]
async fn add_image_comment(
    form: Form<ImgCommentForm>,
    state: Data<AppState>,
    identity: Option<Identity>
    ) -> impl Responder {
   
    if identity.is_none() {
        println!("identity not found, admin logged out");
        let template = LoginTemplate;
        return match template.render() {
            Ok(rendered) => return HttpResponse::Ok().body(rendered),
            Err(_) => {
                return HttpResponse::InternalServerError().finish()
            }
        }
    }

    let form_data = form.into_inner();

    let folder = form_data.folder;
    let file = form_data.file;

    let comment_exists_query: Result<bool, sqlx::Error> = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM image_comments WHERE picture_folder = $1 AND file_name = $2)"
    )
        .bind(&folder)
        .bind(&file)
        .fetch_one(&state.db)
        .await;

    match comment_exists_query {
        Ok(result) => {
            if result {       
                match sqlx::query("UPDATE image_comments SET eng_comment = $1, de_comment = $2 \
                    WHERE picture_folder = $3 AND file_name = $4")
                    .bind(form_data.eng_comment)
                    .bind(form_data.de_comment)
                    .bind(&folder)
                    .bind(&file)
                    .execute(&state.db)
                    .await
                    {
                        Ok(_) => HttpResponse::Found().append_header(("Location", "/admin")).finish(),
                        Err(_) => HttpResponse::InternalServerError().finish(),
                    } 
            } else {
                match sqlx::query(
                    "INSERT INTO image_comments (id, eng_comment, de_comment, picture_folder, file_name) \
                        VALUES ($1, $2, $3, $4, $5)")
                    .bind(new_id(&state, "image_comments").await.to_string())
                    .bind(form_data.eng_comment)
                    .bind(form_data.de_comment)
                    .bind(&folder)
                    .bind(&file)
                    .execute(&state.db)
                    .await
                    {
                        Ok(_) => HttpResponse::Found().append_header(("Location", "/admin")).finish(),
                        Err(_) => HttpResponse::InternalServerError().finish(),
                    }
            }
        }
        Err(_) => {
            HttpResponse::InternalServerError().body("Failed to check db")
        }
    }
}


