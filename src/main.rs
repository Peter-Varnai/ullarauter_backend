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
use actix_web::{web::Data, rt::spawn, cookie::Key, App, HttpServer};
use tokio::time::{Duration, sleep};
use sqlx::sqlite::SqlitePoolOptions;
use models::AppState;
use routes::{public_routes, admin_routes};
use crate::{cache::{warm_cache, update_sidebar_exhibs_cache, update_sidebar_cashe},
            helpers::load_env_file};
use std::env;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = SqlitePoolOptions::new()
        .connect("sqlite://db/ulla_db.db")
        .await
        .expect("Error building a connection pool");

    //configure the load_enf_file function to only run in
    //development mode
    load_env_file(".env");

    let secret_key = Key::generate(); // Secure key for signing cookies
    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("8080".to_string());
    let password = env::var("ADMIN_PASSWORD").expect("failed to fetch admin password from enviroment variable.");
    
    let pool_clone = pool.clone(); 

    println!("{}:{}-{}", host, port, password);
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
            .wrap(IdentityMiddleware::default()) // Manage identity for login/logout
            .wrap(SessionMiddleware::new(
                    CookieSessionStore::default(),
                    secret_key.clone()))
            .service(Files::new("/static", "./static"))
            .app_data(Data::new(AppState {
                db: pool.clone(),
            }))
        .configure(public_routes)
            .configure(admin_routes)
    })
    .bind(format!("{}:{}", host, &port))?
        .run()
        .await
}
