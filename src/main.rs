use mylib::*;
mod database;
use database::{connect_database};
mod admin_login;
use admin_login::{admin};
mod student;
use student::{student_details};

#[tokio::main]
async fn main()-> std::io::Result<()>{
    let db = connect_database().await;
    let db = web::Data::new(db);
    // println!("database name: {}", db.name());
    
    
    // println!("collection name: {}", );
    HttpServer::new(move||{
        
        App::new()
            .app_data(db.clone())
            .service(admin)
            .service(student_details)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
