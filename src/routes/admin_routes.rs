use actix_web::web::ServiceConfig;
use crate::handlers::{ login, logout, create_project, 
                        upload_background, add_exhibition, delete_background, 
                        delete_exhibition, delete_project, update_pfp,
                        update_cv, update_bio, get_project,
                        add_image_comment};

pub fn admin_routes(cfg: &mut ServiceConfig) {
    cfg.service(create_project)
        .service(upload_background)
        .service(delete_background)
        .service(delete_project)
        .service(add_exhibition)
        .service(delete_exhibition)
        .service(update_pfp)
        .service(update_cv)
        .service(update_bio)
        .service(add_image_comment)
        .service(get_project)
        .service(login)
        .service(logout);
}

