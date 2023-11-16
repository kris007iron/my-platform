extern crate rocket;
use mongodb::bson::{doc, Document};
use rocket::{
    futures::StreamExt,
    get,
    serde::json::{json, Value},
    State,
};

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
