use mylib::*;
use std::env;

pub async fn connect_database() -> Database{
    // Load the MongoDB connection string from an environment variable:
   let client_uri: String = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
   println!(" connecting to the database with URL: {}", client_uri);
   // 1. connecting to the database
    let mut _client_options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await.expect("failed to connect to the database!!!");

    // 2. Getting handle to the database
    let client = Client::with_options(_client_options).expect("failed to handle the database");
    
    
    // 3. Get a handle to the database/ set database name
    let _db = client.database("College_Admin_Website");
    
    // 4. Get handle to the collection
    let _user_admin = _db.collection::<Document>("admin_details");

    
    // 5. Insert at least one data to create the DataSet and Collection
    let docs = 
        doc!{
            "username" : "admin1",
            "password" : "admin1@password",
        };
    
    _user_admin.insert_one(docs, None).await.expect("failed to insert data in collections");

    return _db;
}