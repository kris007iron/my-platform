use mongodb::{Database, bson::{doc, Document}};
use rocket::{get, routes, serde::json::{json, Value}, State, futures::StreamExt,};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/api/v1/projects")]
async fn get_projects(db: &State<mongodb::Database>) -> Value {
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

#[shuttle_runtime::main]
async fn main(#[shuttle_shared_db::MongoDb] db:Database) -> shuttle_rocket::ShuttleRocket {
    //vBaCsQhabPmfs47p for mongodb driver if shuttle shared db does not work or does not work on tests;
    let rocket = rocket::build().manage(db).mount("/", routes![index]).mount("/", routes![get_projects]);
    Ok(rocket.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_index() {
        let rocket = rocket::build().mount("/", routes![index]);
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some("Hello, world!".into()));
    }
}