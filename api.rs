use actix_web::{web, App, HttpResponse, HttpServer, Responder, http::StatusCode};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Mutex;
use actix_web::error::ResponseError;
use std::fmt;

#[derive(Debug)]
enum AppError {
    NotFound(String),
    LockFail(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppError::NotFound(ref message) => write!(f, "{}", message),
            AppError::LockFail(ref message) => write!(f, "{}", message),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match *self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::LockFail(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Project {
    id: usize,
    name: String,
    tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    id: usize,
    name: String,
    assigned_to: Option<String>,
    status: String,
}

struct AppState {
    projects: Mutex<Vec<Project>>,
}

async fn create_project(data: web::Json<Project>, app_state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let mut projects = app_state.projects.lock().map_err(|_| AppError::LockFail("Lock failed".into()))?;
    projects.push(data.into_inner());
    Ok(HttpResponse::Ok().json(projects.last()))
}

async fn update_task_name(path: web::Path<(usize, usize)>, new_name: web::Json<String>, app_state: web::Data<AppState>) -> Result<HttpResponse, AppError> {
    let mut projects = app_state.projects.lock().map_err(|_| AppError::LockFail("Lock failed".into()))?;
    let (project_id, task_id) = path.into_inner();
    
    if let Some(project) = projects.iter_mut().find(|p| p.id == project_id) {
        if let Some(task) = project.tasks.iter_mut().find(|t| t.id == task_id) {
            task.name = new_name.into_inner();
            return Ok(HttpResponse::Ok().json(task));
        }
    }
    Err(AppError::NotFound("Task or Project Not Found".into()))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    let app_data = web::Data::new(AppState {
        projects: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(actix_web::middleware::Logger::default())
            .route("/update_task_name/{project_id}/{task_id}", web::put().to(update_task_name))
    })
    .bind(&server_address)?
    .run()
    .await
}