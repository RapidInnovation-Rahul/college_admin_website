use mylib::*;
mod database;
use database::{connect_database};

#[tokio::main]
async fn main()-> std::io::Result<()>{
    let db = connect_database().await;
    println!("database name: {}", db.name());
    HttpServer::new(move||{
        
        App::new()
            .app_data(db.clone())
            .service(admin)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
