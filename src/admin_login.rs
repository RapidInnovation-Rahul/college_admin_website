use mylib::*;
// use actix_web::{post, web, Responder};

#[post("/admin_login")]
async fn admin(_user: web::Form<Admin>, _db: web::Data<Database>) -> impl Responder {
    let _username = _user.admin_username.to_string();
    let _password = _user.admin_password.to_string();
    // println!("your username: {} and password : {}", _username, _password);

    let _user_admin = _db.collection::<Document>("admin_details");
    match _user_admin.find_one(doc!{"username": &_username, "password" : &_password}, None).await.expect("Failed to search username in data_base"){
        Some(_a) => {   
                        println!("{}: WELCOME!", _username);
                    },
        None => {println!("{} this user does not exist", _username)},
    }
    HttpResponse::Ok()
}