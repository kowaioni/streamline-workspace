use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Mutex;

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

async fn create_project(data: web::Json<Project>, app_state: web::Data<AppState>) -> HttpResponse {
    let mut projects = app_state.projects.lock().unwrap();
    projects.push(data.into_inner());
    
    HttpResponse::Ok().json(projects.last())
}

async fn add_task(path: web::Path<usize>, data: web::Json<Task>, app_state: web::Data<AppState>) -> HttpResponse {
    let mut projects = app_state.projects.lock().unwrap();
    let project_index = projects.iter().position(|p| p.id == *path).unwrap_or_else(|| projects.len());
    
    if project_index < projects.len() {
        projects[project_index].tasks.push(data.into_inner());
        HttpResponse::Ok().json(&projects[project_index])
    } else {
        HttpResponse::NotFound().body("Project Not Found")
    }
}

async fn assign_task(path: web::Path<(usize, usize)>, username: web::Json<String>, app_state: web::Data<AppState>) -> HttpResponse {
    let mut projects = app_state.projects.lock().unwrap();
    let (project_id, task_id) = path.into_inner();
    
    if let Some(project) = projects.iter_mut().find(|p| p.id == project_id) {
        if let Some(task) = project.tasks.iter_mut().find(|t| t.id == task_id) {
            task.assigned_to = Some(username.into_inner());
            return HttpResponse::Ok().json(task);
        }
    }
    
    HttpResponse::NotFound().body("Task or Project Not Found")
}

async fn track_progress(path: web::Path<(usize, usize)>, status: web::Json<String>, app_state: web::Data<AppState>) -> HttpResponse {
    let mut projects = app_state.projects.lock().unwrap();
    let (project_id, task_id) = path.into_inner();
    
    if let Some(project) = projects.iter_mut().find(|p| p.id == project_id) {
        if let Some(task) = project.tasks.iter_mut().find(|t| t.id == task_id) {
            task.status = status.into_inner();
            return HttpResponse::Ok().json(task);
        }
    }
    
    HttpResponse::NotFound().body("Task or Project Not Found")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); 

    let server_address = env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    let app_data = web::Data::new(AppState {
        projects: Mutex::new(Vec::new()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .route("/create_project", web::post().to(create_project))
            .route("/add_task/{project_id}", web::post().to(add_task))
            .route("/assign_task/{project_id}/{task_id}", web::put().to(assign_task))
            .route("/track_progress/{project_id}/{task_id}", web::put().to(track_progress))
    })
    .bind(&server_address)?
    .run()
    .await
}