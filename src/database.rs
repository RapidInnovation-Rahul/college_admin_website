use mylib::*;
use std::env;

pub async fn connect_database() -> Database{
    // Load the MongoDB connection string from an environment variable:
   let client_uri: String = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    // 1. connecting to the database
    let mut _client_options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await.expect("failed to connect to the database!!!");

    // 2. Getting handle to the database
    let client = Client::with_options(_client_options).expect("failed to handle the database");
    
    
    // 3. Get a handle to the database/ set database name
    let _db = client.database("College_Admin_Website");
    let _user_admin = _db.collection::<Admin>("admin_details");
    
    return _db;
}