pub use actix_web::{App, web, post, get, Responder, HttpServer, Result, HttpResponse};

pub use tokio;

pub use mongodb::{Client, Database, options::{ClientOptions, ResolverConfig}};
pub use mongodb::bson::{doc,Document};

pub use serde::{Serialize, Deserialize};

pub mod data_types;
pub use data_types::{Admin};