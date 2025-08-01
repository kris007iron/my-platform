extern crate rocket;
use crate::AuthenticatedUser;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, uuid, Document};
use rocket::http::Status;
use rocket::info;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{
    form::Form,
    fs::TempFile,
    futures::StreamExt,
    get, patch, post,
    serde::json::{json, Value},
    FromForm, State,
};
use sanitize_filename::sanitize;
use std::fs;

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

#[post("/api/v1/projects", data = "<upload>")]
pub async fn create_project(
    db: &State<mongodb::Database>,
    upload: Form<Upload<'_>>,
    _user: AuthenticatedUser,
) -> Result<Json<Value>, Custom<Json<Value>>> {
    let collection: mongodb::Collection<Document> = db.collection("projects");
    let mut images: Vec<String> = Vec::new();

    // Get current directory and create target path
    let img_public_path = persist_temp_file(&upload)?;
    let tags;
    //split tags by comma
    if upload.tags.len() > 0 {
        tags = upload.tags[0].split(",").map(|s| s.to_string()).collect();
    } else {
        tags = Vec::new();
    }
    images.push(img_public_path);
    let project = doc! {
        "title": upload.title.clone(),
        "description": upload.description.clone(),
        "link": upload.link.clone(),
        "images": images.clone(),
        "tags": tags.clone(),
    };

    collection.insert_one(project, None).await.unwrap();
    // Insert project data to the database here (omitted for brevity)

    Ok(Json(json!({ "status": "succes" })))
}

fn persist_temp_file(upload: &Form<Upload<'_>>) -> Result<String, Custom<Json<Value>>> {
    let project_path = std::env::current_dir().map_err(|e| {
        Custom(
            Status::InternalServerError,
            Json(json!({"error": format!("Failed to retrieve current directory: {}", e)})),
        )
    })?;
    let persist_to_path = project_path.join("front-end/imgs/projects/");
    info!("Persist to path: {:?}", persist_to_path);

    // Ensure directory exists
    if let Err(e) = fs::create_dir_all(&persist_to_path) {
        return Err(Custom(
            Status::InternalServerError,
            Json(json!({"error": format!("Failed to create directory: {}", e)})),
        ));
    }

    // Get temporary file path
    let temp_path = upload.image.path().ok_or_else(|| {
        Custom(
            Status::InternalServerError,
            Json(json!({"error": "Image path not found"})),
        )
    })?;

    // Extract and sanitize file name
    let filename = upload.image.name().ok_or_else(|| {
        Custom(
            Status::InternalServerError,
            Json(json!({"error": "Failed to get image name"})),
        )
    })?;
    let sanitized_filename = sanitize(&filename);

    // Extract or determine file extension
    let extension = upload
        .image
        .content_type()
        .and_then(|content_type| content_type.extension().map(|ext| ext.to_string()))
        .ok_or_else(|| {
            Custom(
                Status::InternalServerError,
                Json(json!({"error": "Failed to determine file extension from content type"})),
            )
        })?;

    // Create a unique filename with the determined extension
    let unique_filename = format!("{}_{}.{}", sanitized_filename, uuid::Uuid::new(), extension);
    let destination = persist_to_path.join(unique_filename);

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
    //find /imgs and make a substring from there
    let index = image_str.find("/imgs").unwrap();
    let image_str = &image_str[index..];
    Ok(image_str.to_string())
}

#[get("/api/v1/projects/<id>")]
pub async fn get_project(db: &State<mongodb::Database>, id: &str) -> Value {
    let collection: mongodb::Collection<Document> = db.collection("projects");
    let project = collection
        .find_one(
            doc! {
                "_id": ObjectId::parse_str(id).unwrap()
            },
            None,
        )
        .await
        .unwrap()
        .unwrap();
    json!(project)
}

// Test update_project
#[patch("/api/v1/projects/<id>", data = "<upload>")]
pub async fn update_project(
    db: &State<mongodb::Database>,
    id: &str,
    upload: Form<Upload<'_>>,
    // _user: AuthenticatedUser,
) -> Result<Json<Value>, Custom<Json<Value>>> {
    let collection: mongodb::Collection<Document> = db.collection("projects");
    //if data is passed, update it
    let mut project = collection
        .find_one(
            doc! {
                "_id": ObjectId::parse_str(id).unwrap()
            },
            None,
        )
        .await
        .unwrap()
        .unwrap();
    if upload.title.len() > 0 {
        project.insert("title", upload.title.clone());
    }
    if upload.description.len() > 0 {
        project.insert("description", upload.description.clone());
    }
    if upload.link.len() > 0 {
        project.insert("link", upload.link.clone());
    }
    if upload.tags.len() > 0 {
        let tags: Vec<String> = upload.tags[0].split(",").map(|s| s.to_string()).collect();
        project.insert("tags", tags.clone());
    }
    //if upload.image is not empty, update image
    if upload.image.len() > 0 {
        let img_public_path = persist_temp_file(&upload)?;
        let images = vec![img_public_path];
        project.insert("images", images.clone());
    }
    println!("{}", project);
    collection
        .update_one(
            doc! {
                "_id": ObjectId::parse_str(id).unwrap()
            },
            project,
            None,
        )
        .await
        .unwrap();

    Ok(Json(json!({ "status": "succes" })))
}
