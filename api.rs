use actix_web::{web, App, HttpResponse, HttpServer, Responder, http::StatusCode};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Mutex;
use actix_web::error::ResponseError;
use std::fmt;

#[derive(Debug)]
enum WorkspaceError {
    NotFound(String),
    MutexLockFailure(String),
}

impl fmt::Display for WorkspaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WorkspaceError::NotFound(ref message) => write!(f, "{}", message),
            WorkspaceError::MutexLockFailure(ref message) => write!(f, "{}", message),
        }
    }
}

impl ResponseError for WorkspaceError {
    fn status_code(&self) -> StatusCode {
        match *self {
            WorkspaceError::NotFound(_) => StatusCode::NOT_FOUND,
            WorkspaceError::MutexLockFailure(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct ProjectDetail {
    id: usize,
    name: String,
    task_list: Vec<TaskDetail>,
}

#[derive(Serialize, Deserialize, Clone)]
struct TaskDetail {
    id: usize,
    title: String,
    assignee: Option<String>, 
    current_status: String,
}

struct SharedState {
    project_registry: Mutex<Vec<ProjectDetail>>,
}

async fn add_project(project_new: web::Json<ProjectDetail>, application_state: web::Data<SharedState>) -> Result<HttpResponse, WorkspaceError> {
    let mut project_collection = application_state.project_registry.lock().map_err(|_| WorkspaceError::MutexLockFailure("Failed to acquire mutex lock".into()))?;
    project_collection.push(project_new.into_inner());
    Ok(HttpResponse::Ok().json(project_collection.last()))
}

async fn modify_task_title(task_identifiers: web::Path<(usize, usize)>, new_title: web::Json<String>, application_state: web::Data<SharedState>) -> Result<HttpResponse, WorkspaceError> {
    let mut project_collection = application_state.project_registry.lock().map_err(|_| WorkspaceError::MutexLockFailure("Failed to acquire mutex lock".into()))?;
    let (project_identifier, task_identifier) = task_identifiers.into_inner();
    
    if let Some(project) = project_collection.iter_mut().find(|project| project.id == project_identifier) {
        if let Some(task) = project.task_list.iter_mut().find(|task| task.id == task_identifier) {
            task.title = new_title.into_inner();
            return Ok(HttpResponse::Ok().json(task));
        }
    }
    Err(WorkspaceError::NotFound("Project or Task not found.".into()))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let server_endpoint = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    let shared_app_state = web::Data::new(SharedState {
        project_registry: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(shared_app_state.clone())
            .wrap(actix_web::middleware::Logger::default())
            .route("/project/task/modify/{project_id}/{task_id}", web::put().to(modify_task_title))
    })
    .bind(&server_endpoint)?
    .run()
    .await
}