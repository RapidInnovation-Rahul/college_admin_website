use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Admin{
    pub admin_username : String,
    pub admin_password : String,
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Student{
    pub student_name : String,
    pub student_roll : u8,
    pub student_dept : Dept,
    pub student_fees_clearance : bool,
    pub student_library_clearance : bool
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Dept{
    CSE,
    ELECTRICAL,
    MECHANICAL,
    CHEMICAL,
    CIVIL,
    PassOut,
}