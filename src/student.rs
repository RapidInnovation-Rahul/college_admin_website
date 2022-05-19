use mylib::*;
use serde_json::json;

#[get("/student/{name}")]
pub async fn student_details(st: web::Path<(String,)>, db: web::Data<Database>) -> impl Responder{
    
    let st_name = st.into_inner().0;
    println!("Showing details for the student named: {} --", st_name);

    let st_collection = db.collection::<Student>("student_details");

    match st_collection.find_one(doc!{"Student_name" : &st_name}, None).await.expect("Failed to search Student name in data_base"){
        Some(a) => {   
                        HttpResponse::Ok().json(json!(a))
                    },
        None => {
            HttpResponse::Ok().json(json!({"massege": format!("{} user not found", st_name)}))
        }
    }
}

#[post("/add_student")]
pub async fn add_student(st: web::Json<Student>, db: web::Data<Database>) -> impl Responder{
    
    let st = Student{
        student_name : st.student_name.to_string(),
        student_roll : st.student_roll,
        student_dept : st.student_dept,
        student_fees_clearance : st.student_fees_clearance,
        student_library_clearance : st.student_library_clearance,
    };

    HttpResponse::Ok()
}

#[post("/{name}/{dept}/{roll}")]
pub async fn del_student(st: web::Path<(String,String,u8)>, db : web::Data<Database>) -> impl Responder{
    let (name, dept, roll) = st.into_inner();
    let st_collection = db.collection::<Student>("student_details");
    let delete_result = st_collection.delete_one(doc!{"Student_name" : &name, "Student_dept" : &dept, "Student_roll" : &roll}, None).await.expect("Failed to delete document");
    println!("Deleted {:?} documents", delete_result);
    HttpResponse::Ok()
}
