use actix_web::{get, web, Responder};

#[get("/{username}")]
async fn admin(user: web::Path<String>) -> impl Responder {
    format!("{} ",user)
}