use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Course {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub instructor: String,
    pub duration: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateCourseRequest {
    pub title: String,
    pub description: String,
    pub instructor: String,
    pub duration: u32,
}

pub async fn create_course(req: web::Json<CreateCourseRequest>) -> HttpResponse {
    let course = Course {
        id: 1,
        title: req.title.clone(),
        description: req.description.clone(),
        instructor: req.instructor.clone(),
        duration: req.duration,
    };

    HttpResponse::Created().json(course)
}

pub async fn get_courses() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!([]))
}

pub async fn get_course(id: web::Path<u32>) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "id": id.into_inner(),
        "title": "Sample Course"
    }))
}

pub async fn update_course(id: web::Path<u32>, req: web::Json<CreateCourseRequest>) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "id": id.into_inner(),
        "title": &req.title
    }))
}

pub async fn delete_course(id: web::Path<u32>) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Course deleted successfully"
    }))
}
