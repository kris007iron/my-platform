extern crate rocket;
use crate::AuthenticatedUser;
use mongodb::bson::{doc, uuid, Document};
use rocket::{
    form::Form,
    fs::TempFile,
    futures::StreamExt,
    get,
    http::Status,
    info, post,
    response::status::Custom,
    serde::json::{json, Json, Value},
    FromForm, State,
};
use sanitize_filename::sanitize;
use std::fs;

#[derive(FromForm)]
pub struct Upload<'r> {
    title: String,
    pub_date: String,
    link: String,
    thumbnail: TempFile<'r>,
}

#[get("/api/v1/posts")]
pub async fn get_posts(db: &State<mongodb::Database>) -> Value {
    let collection: mongodb::Collection<Document> = db.collection("posts");
    let mut cursor: mongodb::Cursor<Document> = collection.find(doc! {}, None).await.unwrap();
    let mut posts: Vec<Value> = Vec::new();
    while let Some(post) = cursor.next().await {
        match post {
            Ok(post) => posts.push(json!(post)),
            Err(e) => println!("Error: {}", e),
        }
    }
    json!(posts)
}

#[post("/api/v1/posts", data = "<upload>")]
pub async fn create_post(
    db: &State<mongodb::Database>,
    upload: Form<Upload<'_>>,
    _user: AuthenticatedUser,
) -> Value {
    let collection: mongodb::Collection<Document> = db.collection("posts");
    let thumbnail = persist_temp_file(&upload).unwrap();
    let post = doc! {
        "thumbnail": thumbnail,
        "title": upload.title.to_string(),
        "pubDate": upload.pub_date.to_string(),
        "link": upload.link.to_string(),
    };
    collection.insert_one(post, None).await.unwrap();
    json!({"status": "ok"})
}

fn persist_temp_file(upload: &Form<Upload<'_>>) -> Result<String, Custom<Json<Value>>> {
    let project_path = std::env::current_dir().map_err(|e| {
        Custom(
            Status::InternalServerError,
            Json(json!({"error": format!("Failed to retrieve current directory: {}", e)})),
        )
    })?;
    let persist_to_path = project_path.join("src/front-end/imgs/posts/");
    info!("Persist to path: {:?}", persist_to_path);

    // Ensure directory exists
    if let Err(e) = fs::create_dir_all(&persist_to_path) {
        return Err(Custom(
            Status::InternalServerError,
            Json(json!({"error": format!("Failed to create directory: {}", e)})),
        ));
    }

    // Get temporary file path
    let temp_path = upload.thumbnail.path().ok_or_else(|| {
        Custom(
            Status::InternalServerError,
            Json(json!({"error": "thumbnail path not found"})),
        )
    })?;

    // Extract and sanitize file name
    let filename = upload.thumbnail.name().ok_or_else(|| {
        Custom(
            Status::InternalServerError,
            Json(json!({"error": "Failed to get thumbnail name"})),
        )
    })?;
    let sanitized_filename = sanitize(&filename);

    // Extract or determine file extension
    let extension = upload
        .thumbnail
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

    // Convert the thumbnail path to a string
    let thumbnail_str = destination.to_str().ok_or_else(|| {
        Custom(
            Status::InternalServerError,
            Json(json!({"error": "Failed to convert thumbnail path to string"})),
        )
    })?;
    //find /imgs and make a substring from there
    let index = thumbnail_str.find("/imgs").unwrap();
    let thumbnail_str = &thumbnail_str[index..];
    Ok(thumbnail_str.to_string())
}
