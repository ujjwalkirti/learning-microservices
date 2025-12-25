use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSyllabusRequest {
    pub course_id: u32,
    pub topics: Vec<String>,
    pub duration: u32,
    pub objectives: Vec<String>,
}

pub async fn create_syllabus(req: web::Json<CreateSyllabusRequest>) -> HttpResponse {
    HttpResponse::Created().json(serde_json::json!({
        "id": 1,
        "course_id": req.course_id,
        "topics": &req.topics
    }))
}

pub async fn get_syllabus(course_id: web::Path<u32>) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!([]))
}

pub async fn update_syllabus(id: web::Path<u32>, req: web::Json<CreateSyllabusRequest>) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "id": id.into_inner(),
        "course_id": req.course_id
    }))
}
