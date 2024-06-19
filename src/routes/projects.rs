extern crate rocket;
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

#[post("/api/v1/projects", data = "<upload>")]
pub async fn create_project(
    db: &State<mongodb::Database>,
    mut upload: Form<Upload<'_>>,
    user: AuthenticatedUser,
) -> Result<Json<Value>, Custom<Json<Value>>> {
    let collection: mongodb::Collection<Document> = db.collection("projects");
    let mut image: Vec<String> = Vec::new();
    let project_path = std::env::current_dir().map_err(|e| {
        (Custom(
            Status::InternalServerError,
            Json(json!({"error": e.to_string()})),
        ));
    });
    let persist_to_path = project_path.unwrap().join("src/front-end/imgs/projects/");
    info!("{:?}", persist_to_path);
    if let Err(e) = upload.image.persist_to(&persist_to_path).await {
        return Err(Custom(
            Status::InternalServerError,
            Json(json!({"error": e.to_string()})),
        ));
    }
    let image_path = upload.image.path().ok_or_else(|| {
        Custom(
            Status::InternalServerError,
            Json(json!({"error": "Image path not found"})),
        )
    });
    let image_path = image_path.unwrap();
    image.push(image_path.to_str().unwrap().to_string());
    Ok(Json(json!({
        "title": upload.title.clone(),
        "description": upload.description.clone(),
        "link": upload.link.clone(),
        "image": image,
        "tags": upload.tags.clone(),
        "user": user.username,
    })))
}
