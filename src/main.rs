use std::{io, path::PathBuf};
extern crate rocket;
use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Client,
};
use rocket::{
    fairing::{Fairing, Info, Kind},
    fs::NamedFile,
    futures::StreamExt,
    get,
    http::{Header, Method, Status},
    /*post,*/ routes,
    serde::json::{json, /*Json,*/ Value},
    Request, Response, State,
};
use shuttle_secrets::SecretStore;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));

        if _request.method() == Method::Options {
            response.set_status(Status::Ok);
        }
    }
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

#[allow(dead_code)]
#[get("/api/v1/projects/<id>")]
async fn get_project(db: &State<mongodb::Database>, id: String) -> Value {
    let collection: mongodb::Collection<Document> = db.collection("projects");
    let obj_id = ObjectId::parse_str(id);
    let result = collection
        .find_one(doc! {"_id": obj_id.unwrap()}, None)
        .await;
    match result {
        Ok(result) => match result {
            Some(result) => json!(result),
            None => json!("No project found"),
        },
        Err(e) => json!(e.to_string()),
    }
}

// #[post("/api/v1/projects", data = "<project>")]
// async fn create_project(db: &State<mongodb::Database>, project: Json<Document>) -> Value {
//     let collection: mongodb::Collection<Document> = db.collection("projects");
//     let project_to_ins: Document = project.into_inner();
//     let result = collection.insert_one(project_to_ins, None).await;
//     match result {
//         Ok(result) => json!(result),
//         Err(e) => json!(e.to_string()),
//     }
// }

// #[get("/api/v1/posts")]
// async fn get_posts(db: &State<mongodb::Database>) -> Value {
//     let collection: mongodb::Collection<Document> = db.collection("posts");
//     let mut cursor: mongodb::Cursor<Document> = collection.find(doc! {}, None).await.unwrap();
//     let mut posts: Vec<Value> = Vec::new();
//     while let Some(post) = cursor.next().await {
//         match post {
//             Ok(post) => posts.push(json!(post)),
//             Err(e) => println!("Error: {}", e),
//         }
//     }
//     json!(posts)
// }

// #[get("/api/v1/posts/<id>")]
// async fn get_post(db: &State<mongodb::Database>, id: String) -> Value {
//     let collection: mongodb::Collection<Document> = db.collection("posts");
//     let obj_id = ObjectId::parse_str(id);
//     let result = collection
//         .find_one(doc! {"_id": obj_id.unwrap()}, None)
//         .await;
//     match result {
//         Ok(result) => match result {
//             Some(result) => json!(result),
//             None => json!("No post found"),
//         },
//         Err(e) => json!(e.to_string()),
//     }
// }

// #[post("/api/v1/posts", data = "<post>")]
// async fn create_post(db: &State<mongodb::Database>, post: Json<Document>) -> Value {
//     let collection: mongodb::Collection<Document> = db.collection("posts");
//     let post_to_ins: Document = post.into_inner();
//     let result = collection.insert_one(post_to_ins, None).await;
//     match result {
//         Ok(result) => json!(result),
//         Err(e) => json!(e.to_string()),
//     }
// }

async fn db_connection(client: &str) -> mongodb::Database {
    Client::with_uri_str(client)
        .await
        .unwrap()
        .database("PortfolioAPI")
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    let project_path = std::env::current_dir().unwrap();
    //get build path inside src
    let build_path = project_path.join("src/front-end");
    NamedFile::open(build_path.join(file)).await.ok()
}

#[get("/")]
async fn index() -> io::Result<NamedFile> {
    let project_path = std::env::current_dir().unwrap();
    //get build path inside src
    let build_path = project_path.join("src/front-end");
    NamedFile::open(build_path.join("index.html")).await
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_rocket::ShuttleRocket {
    let secret = if let Some(secret) = secret_store.get("MONGO_STR") {
        secret
    } else {
        panic!("No secret found for MY_API_KEY");
    };
    let client = secret.as_str();
    //vBaCsQhabPmfs47p for mongodb driver if shuttle shared db does not work or does not work on tests;
    let db = db_connection(client).await;
    let rocket = rocket::build()
        .manage(db)
        .attach(CORS)
        .mount("/", routes![index])
        .mount("/", routes![files])
        .mount("/", routes![get_projects]);
    // .mount("/", routes![create_project])
    // .mount("/", routes![get_project])
    // .mount("/", routes![get_posts])
    // .mount("/", routes![create_post])
    // .mount("/", routes![get_post]);
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
