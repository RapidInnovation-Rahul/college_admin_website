use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Admin{
    pub admin_username: String,
    pub admin_password : String,
}