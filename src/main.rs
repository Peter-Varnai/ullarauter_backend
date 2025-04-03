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
            helpers::config};
use std::{env, path::PathBuf};


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = config();
    let host = config.get("host").clone();
    let secret_key = Key::generate();
    let port = config.get("port").unwrap();
    let host = config.get("host").unwrap();
    let db_url = config.get("db_url").unwrap();
    let password = config.get("password").unwrap();
    let static_path = config.get("static_path").unwrap();

    let pool = SqlitePoolOptions::new()
        .connect(&db_url)
        // .connect("sqlite://db/ulla_db.db")
        .await
        .expect("Error building a connection pool");

    let pool_clone = pool.clone(); 


    warm_cache(&pool).await;

    spawn(async move {
        loop {
            sleep(Duration::from_secs(60 * 60 * 12)).await;
            update_sidebar_exhibs_cache(&pool_clone).await;
            update_sidebar_cashe(&pool_clone).await;
            println!("front page exhibitions cache refreshed");
        }
    });   

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
