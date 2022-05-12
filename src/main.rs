use actix_web::{App, HttpServer};
use tokio;
mod admin_login;
use admin_login::{admin};
mod database;
use database::{database};

#[tokio::main]
async fn main()-> std::io::Result<()>{
    let _db = database();
    HttpServer::new(move||{
        App::new()
        // .app_data(db.clone())
        .service(admin)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
