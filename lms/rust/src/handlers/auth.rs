use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub name: String,
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn register(req: web::Json<RegisterRequest>) -> HttpResponse {
    HttpResponse::Created().json(serde_json::json!({
        "message": "User registered successfully"
    }))
}

pub async fn login(req: web::Json<LoginRequest>) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "token": "jwt_token_here",
        "user": {
            "email": &req.email,
            "role": "student"
        }
    }))
}

pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "message": "Logged out successfully"
    }))
}
