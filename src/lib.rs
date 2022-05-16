pub use actix_web::{App, HttpServer};

pub use tokio;

pub use mongodb::{Client, Database, options::{ClientOptions, ResolverConfig}};
pub use mongodb::bson::{doc,Document};

pub mod admin_login;
pub use admin_login::{admin};

pub use serde::{Serialize, Deserialize};

pub mod data_types;
pub use data_types::{Admin};