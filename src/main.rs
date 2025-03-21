mod routes;
mod models;
mod handlers;
mod services;
mod helpers;
mod cache;
mod errors;


use actix_files::Files;
use actix_identity::{IdentityMiddleware};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_web::{web::Data, rt::spawn, cookie::Key, App, HttpServer};
use tokio::time::{Duration, sleep};
use sqlx::sqlite::SqlitePoolOptions;
use models::AppState;
use routes::{public_routes, admin_routes};
use crate::cache::{warm_cache, update_sidebar_exhibs_cache, update_sidebar_cashe};



#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = SqlitePoolOptions::new()
        .connect("sqlite://db/ulla_db.db")
        .await
        .expect("Error building a connection pool");

    warm_cache(&pool).await;

    let secret_key = Key::generate(); // Secure key for signing cookies

    let pool_clone = pool.clone();

    spawn(async move {
        loop {
            sleep(Duration::from_secs(6)).await;
            update_sidebar_exhibs_cache(&pool_clone).await;
            update_sidebar_cashe(&pool_clone).await;
            println!("sidebar exhib cache called");
        }
    });    


    HttpServer::new(move || {
        App::new()
            .wrap(IdentityMiddleware::default()) // Manage identity for login/logout
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .service(Files::new("/static", "./static")
                // .show_files_listing()
                )
            .app_data(Data::new(AppState {
                db: pool.clone(),
            }))
            .configure(public_routes)
            .configure(admin_routes)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

