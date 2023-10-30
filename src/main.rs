use mongodb::{
    bson::{doc, Document},
    Client,
};
use rocket::{
    futures::StreamExt,
    get, post, routes,
    serde::json::{json, Json, Value},
    State,
};

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

#[post("/api/v1/projects", data = "<project>")]
async fn create_project(db: &State<mongodb::Database>, project: Json<Document>) -> Value {
    let collection: mongodb::Collection<Document> = db.collection("projects");
    let project_to_ins: Document = project.into_inner();
    let result = collection.insert_one(project_to_ins, None).await;
    match result {
        Ok(result) => json!(result),
        Err(e) => json!(e.to_string()),
    }
}

async fn db_connection() -> mongodb::Database {
    Client::with_uri_str("mongodb+srv://kris007iron:vBaCsQhabPmfs47p@cluster0.httk5bz.mongodb.net/?retryWrites=true&w=majority").await.unwrap().database("PortfolioAPI")
}

#[shuttle_runtime::main]
async fn main(/*#[shuttle_shared_db::MongoDb] db: Database*/) -> shuttle_rocket::ShuttleRocket {
    //vBaCsQhabPmfs47p for mongodb driver if shuttle shared db does not work or does not work on tests;
    let db = db_connection().await;
    let rocket = rocket::build()
        .manage(db)
        .mount("/", routes![index])
        .mount("/", routes![get_projects])
        .mount("/", routes![create_project]);
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
