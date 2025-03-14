use actix_web::web::ServiceConfig;
use crate::handlers::{ index, set_language, contact, bio, project, session_info, admin };

pub fn public_routes(cfg: &mut ServiceConfig) {
    cfg.service(index)
        .service(contact)
        .service(bio)
        .service(project)
        .service(session_info)
        .service(set_language)
        .service(admin);
}

