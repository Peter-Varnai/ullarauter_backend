mod routes;
mod models;
mod handlers;
mod services;
mod helpers;
mod cache;
mod errors;


use actix_files::Files;
use actix_identity::IdentityMiddleware;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{web::Data, rt::spawn, cookie::Key, App, HttpServer, middleware::DefaultHeaders};
use tokio::time::{Duration, sleep};
use sqlx::sqlite::SqlitePoolOptions;
use models::AppState;
use routes::{public_routes, admin_routes};
use crate::{cache::{warm_cache, update_sidebar_exhibs_cache, update_sidebar_cashe},
            helpers::load_env_file};
use std::{env, path::PathBuf};


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let db_addr = env::current_dir().unwrap().join("db/ulla_db.db");
    let db_url = format!("sqlite://{}", db_addr.display());
    let pool = SqlitePoolOptions::new()
        .connect(&db_url)
        // .connect("sqlite://db/ulla_db.db")
        .await
        .expect("Error building a connection pool");
    
    println!("connected to db at: {}", db_url);
    //configure the load_enf_file function to only run in
    //development mode
    load_env_file(".env");

    let secret_key = Key::generate(); // Secure key for signing cookies
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let mut port = env::var("PORT").unwrap_or("8080".to_string());
    let password = env::var("ADMIN_PASSWORD").expect("failed to fetch admin password from enviroment variable.");

    // determine static file path
    let mut env_static_path = env::var("STATIC_PATH").expect("failed to fetch enviroment variable");
    let current_path = env::current_exe().unwrap();

    let static_path = if current_path.parent().unwrap().ends_with("debug") {
        println!("Running in developer mode");
        PathBuf::from("home/nixos/projects/ullaBackend2/static")
    } else {
       PathBuf::from(&env_static_path)
    };

    //reassign host, static if run in debug mode
    if env::current_exe().unwrap().ends_with("debug"){
        println!("running in developer mode");
        port = "4002".to_string();
    }

    let pool_clone = pool.clone(); 

    println!("  {}:{}, static path: {}", host, port, static_path.display());
    // starting daily process of refreshing the relevant exhibitions 
    // on the front page
    spawn(async move {
        loop {
            sleep(Duration::from_secs(60 * 60 * 12)).await;
            update_sidebar_exhibs_cache(&pool_clone).await;
            update_sidebar_cashe(&pool_clone).await;
            println!("front page exhibitions cache refreshed");
        }
    });   


    warm_cache(&pool).await;

    HttpServer::new(move || {
        App::new()
            .wrap(DefaultHeaders::new().add(("X-Forwarded-Proto", "https")))
            .wrap(SessionMiddleware::new(
                    CookieSessionStore::default(),
                    secret_key.clone()))
            .service(Files::new("/static", static_path.clone()))
            .app_data(Data::new(AppState {
                db: pool.clone(),
                pw: password.clone(),
            }))
        .configure(public_routes)
            .wrap(IdentityMiddleware::default()) // Manage identity for login/logout
            .configure(admin_routes)
    })
    .bind(format!("{}:{}", host, &port))?
        .run()
        .await
}
