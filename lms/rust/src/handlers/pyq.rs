use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePyqRequest {
    pub course_id: u32,
    pub year: u32,
    pub questions: Vec<String>,
    pub duration: u32,
    pub total_marks: u32,
}

pub async fn create_pyq(req: web::Json<CreatePyqRequest>) -> HttpResponse {
    HttpResponse::Created().json(serde_json::json!({
        "id": 1,
        "course_id": req.course_id,
        "year": req.year
    }))
}

pub async fn get_pyqs(course_id: web::Path<u32>) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!([]))
}

pub async fn get_pyq_by_year(path: web::Path<(u32, u32)>) -> HttpResponse {
    let (course_id, year) = path.into_inner();
    HttpResponse::Ok().json(serde_json::json!({
        "course_id": course_id,
        "year": year
    }))
}
