use actix_identity::Identity;
use actix_web::{Responder, HttpResponse};
use crate::models::LoginTemplate;
use askama::Template;
use futures_util::StreamExt;

pub fn auth_check(identity: Option<Identity>) {
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
}
