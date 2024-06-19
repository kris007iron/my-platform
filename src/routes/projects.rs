extern crate rocket;
use std::fs;
use std::path::Path;

use crate::AuthenticatedUser;
use mongodb::bson::{doc, Document};
use rocket::http::Status;
use rocket::info;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{
    form::Form,
    fs::TempFile,
    futures::StreamExt,
    get, post,
    serde::json::{json, Value},
    FromForm, State,
};

#[path = "../utils/jwt.rs"]
mod jwt;

#[derive(FromForm)]
pub struct Upload<'r> {
    title: String,
    description: String,
    link: String,
    image: TempFile<'r>,
    tags: Vec<String>,
}

#[get("/api/v1/projects")]
pub async fn get_projects(db: &State<mongodb::Database>) -> Value {
    let collection: mongodb::Collection<Document> = db.collection("projects");
    let mut cursor: mongodb::Cursor<Document> = collection.find(doc! {}, None).await.unwrap();
    let mut projects: Vec<Value> = Vec::new();
    while let Some(project) = cursor.next().await {
        match project {
            Ok(project) => projects.push(json!(project)),
            Err(e) => println!("Error: {}", e),
        }
    }
    json!(projects)
}

//TODO: test this route
#[post("/api/v1/projects", data = "<upload>")]
pub async fn create_project(
    db: &State<mongodb::Database>,
    mut upload: Form<Upload<'_>>,
    user: AuthenticatedUser,
) -> Result<Json<Value>, Custom<Json<Value>>> {
    let collection: mongodb::Collection<Document> = db.collection("projects");
    let mut image: Vec<String> = Vec::new();

    // Get current directory and create target path
    let project_path = std::env::current_dir().map_err(|e| {
        Custom(
            Status::InternalServerError,
            Json(json!({"error": format!("Failed to retrieve current directory: {}", e)})),
        )
    })?;
    let persist_to_path = project_path.join("src/front-end/imgs/projects/");
    info!("{:?}", persist_to_path);

    // Ensure directory exists
    if let Err(e) = fs::create_dir_all(&persist_to_path) {
        return Err(Custom(
            Status::InternalServerError,
            Json(json!({"error": format!("Failed to create directory: {}", e)})),
        ));
    }

    // Persist the image
    let temp_path = upload.image.path().ok_or_else(|| {
        Custom(
            Status::InternalServerError,
            Json(json!({"error": "Image path not found"})),
        )
    })?;

    // Extract file name and extension
    let filename = upload.image.name().ok_or_else(|| {
        Custom(
            Status::InternalServerError,
            Json(json!({"error": "Failed to get image name"})),
        )
    })?;
    let extension = Path::new(&filename)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("");

    // Create destination path with the same extension
    let destination = persist_to_path.join(format!("{}.{}", filename, extension));

    // Attempt to copy the file
    fs::copy(&temp_path, &destination).map_err(|e| {
        Custom(
            Status::InternalServerError,
            Json(json!({"error": format!("Failed to copy file: {}", e)})),
        )
    })?;

    // Remove the temporary file
    fs::remove_file(&temp_path).map_err(|e| {
        Custom(
            Status::InternalServerError,
            Json(json!({"error": format!("Failed to remove temp file: {}", e)})),
        )
    })?;

    // Convert the image path to a string
    let image_str = destination.to_str().ok_or_else(|| {
        Custom(
            Status::InternalServerError,
            Json(json!({"error": "Failed to convert image path to string"})),
        )
    })?;

    image.push(image_str.to_string());

    // Assuming further operations and database interactions are successful
    // Here you might want to insert a new document into the database

    Ok(Json(json!({ "status": image[0] })))
}
