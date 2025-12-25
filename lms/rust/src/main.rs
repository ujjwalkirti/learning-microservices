use actix_web::{web, App, HttpServer, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::collections::HashMap;

mod handlers;
use handlers::{auth, cms, syllabus, analytics, pyq};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    pub role: String,
}

pub struct AppState {
    pub users: Mutex<HashMap<String, User>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_state = web::Data::new(AppState {
        users: Mutex::new(HashMap::new()),
    });

    println!("LMS Microservice - Rust starting on 0.0.0.0:3001");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/health", web::get().to(health_check))
            .service(
                web::scope("/api/auth")
                    .route("/register", web::post().to(auth::register))
                    .route("/login", web::post().to(auth::login))
                    .route("/logout", web::post().to(auth::logout))
            )
            .service(
                web::scope("/api/cms")
                    .route("/courses", web::post().to(cms::create_course))
                    .route("/courses", web::get().to(cms::get_courses))
                    .route("/courses/{id}", web::get().to(cms::get_course))
                    .route("/courses/{id}", web::put().to(cms::update_course))
                    .route("/courses/{id}", web::delete().to(cms::delete_course))
            )
            .service(
                web::scope("/api/syllabus")
                    .route("/create", web::post().to(syllabus::create_syllabus))
                    .route("/course/{course_id}", web::get().to(syllabus::get_syllabus))
                    .route("/{id}", web::put().to(syllabus::update_syllabus))
            )
            .service(
                web::scope("/api/analytics")
                    .route("/track", web::post().to(analytics::track_activity))
                    .route("/progress/{user_id}/{course_id}", web::get().to(analytics::get_progress))
                    .route("/course/{course_id}", web::get().to(analytics::get_course_analytics))
            )
            .service(
                web::scope("/api/pyq")
                    .route("/create", web::post().to(pyq::create_pyq))
                    .route("/course/{course_id}", web::get().to(pyq::get_pyqs))
                    .route("/course/{course_id}/year/{year}", web::get().to(pyq::get_pyq_by_year))
            )
    })
    .bind("0.0.0.0:3001")?
    .run()
    .await
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "LMS Microservice - Rust is running"
    }))
}
