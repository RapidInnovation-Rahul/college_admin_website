use mylib::*;
use std::env;

pub async fn connect_database() -> Database{
    // Load the MongoDB connection string from an environment variable:
    dotenv().ok();
   let client_uri: String = env::var("MONGODB_URI").unwrap();
   println!(" connecting to the database with URL: {}", client_uri);
   // 1. connecting to the database
    let mut _client_options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await.expect("failed to connect to the database!!!");

    // 2. Getting handle to the database
    let client = Client::with_options(_client_options).expect("failed to handle the database");
    
    
    // 3. Get a handle to the database/ set database name
    let _db = client.database("College_Admin_Website");
    
    // 4. Get handle to the collection
    // let student_collection = _db.collection::<Student>("student_details");

    
    // 5. Insert at least one data to create the DataSet and Collection
    // let st = vec![
        // Student{
        //     student_name : "Ankita kumari".to_string(),
        //     student_roll : 69,
        //     student_dept : Dept::CHEMICAL,
        //     student_fees_clearance : true,
        //     student_library_clearance : false,
        // },
    //     Student{
    //         student_name : "Faizal".to_string(),
    //         student_roll : 12,
    //         student_dept : Dept::ELECTRICAL,
    //         student_fees_clearance : false,
    //         student_library_clearance : true,
    //     },

    // ];
    // student_collection.insert_many(st, None).await.expect("failed to insert data in collections");

    return _db;
}