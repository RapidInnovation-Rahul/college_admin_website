use mylib::*;
// use actix_web::{post, web, Responder};

#[post("/admin_login")]
async fn admin(_user: web::Form<Admin>, _db: web::Data<Database>) -> impl Responder {
    let _username = _user.admin_username.to_string();
    let _password = _user.admin_password.to_string();
    println!("your username: {} and password : {}", _username, _password);
    HttpResponse::Ok().json(_username)
}