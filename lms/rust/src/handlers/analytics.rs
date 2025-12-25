use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackActivityRequest {
    pub user_id: String,
    pub course_id: u32,
    pub action: String,
    pub timestamp: String,
}

pub async fn track_activity(req: web::Json<TrackActivityRequest>) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Activity tracked"
    }))
}

pub async fn get_progress(path: web::Path<(String, u32)>) -> HttpResponse {
    let (user_id, course_id) = path.into_inner();
    HttpResponse::Ok().json(serde_json::json!({
        "user_id": user_id,
        "course_id": course_id,
        "progress": 0.0
    }))
}

pub async fn get_course_analytics(course_id: web::Path<u32>) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "course_id": course_id.into_inner(),
        "total_users": 0,
        "total_activities": 0
    }))
}
