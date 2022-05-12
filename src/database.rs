use mongodb::{Client,Database, options::{ClientOptions, ResolverConfig}};
use std::env;

pub async fn database() -> Database{
    // Load the MongoDB connection string from an environment variable:
   let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
    // 1. connecting to the database
    let mut _client_options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await.expect("failed to connect to the database!!!");

    // 2. Getting handle to the database
    let client = Client::with_options(_client_options).expect("failed to handle the database");
    // 3. Get a handle to the database
    let db = client.database("College_Admin_Website");
    // let collection1 = db.collection::<>


    // printing the databases
    println!("Databases:");
    for name in client.list_database_names(None, None).await.expect("failed to print the database") {
        println!("- {}", name);
    }


    return db
}