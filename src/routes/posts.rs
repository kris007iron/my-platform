extern crate rocket;
use crate::AuthenticatedUser;
use mongodb::bson::{doc, Document};
use rocket::{
    futures::StreamExt,
    get, post,
    serde::json::{json, Value},
    State,
};

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

#[post("/api/v1/posts")]
pub fn create_post(db: &State<mongodb::Database>, user: AuthenticatedUser) -> Value {
    json!({"status": "ok"})
}
