extern crate rocket;
use mongodb::bson::{doc, Document};
use rocket::{
    futures::StreamExt,
    get, /*, post */
    serde::json::{json, Value},
    State,
};

#[path = "../utils/jwt.rs"]
mod jwt;

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

/*#[post("/api/v1/projects", data = "<project>")]
pub async fn create_post(db: &State<mongodb::Database>, project: Value) -> Value {
    let collection: mongodb::Collection<Document> = db.collection("projects");
    let project = project.as_object().unwrap();
    let project = doc! {
        "title": project.get("title").unwrap(),
        "description": project.get("description").unwrap(),
        "link": project.get("link").unwrap(),
        "image": project.get("image").unwrap(),
    };
    collection.insert_one(project, None).await.unwrap();
    json!({"status": "ok"})
}*/
