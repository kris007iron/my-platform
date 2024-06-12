use std::{io, path::PathBuf};
extern crate rocket;
use mongodb::{bson::doc, Client};
use rocket::{fs::NamedFile, get, routes};
use shuttle_runtime::SecretStore;

mod cors;
mod routes;

async fn db_connection(client: &str) -> mongodb::Database {
    Client::with_uri_str(client)
        .await
        .unwrap()
        .database("PortfolioAPI")
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> Option<NamedFile> {
    let project_path = std::env::current_dir().unwrap();
    let build_path = project_path.join("src/front-end");
    NamedFile::open(build_path.join(file)).await.ok()
}

#[get("/")]
async fn index() -> io::Result<NamedFile> {
    let project_path = std::env::current_dir().unwrap();
    let build_path = project_path.join("src/front-end");
    NamedFile::open(build_path.join("index.html")).await
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> shuttle_rocket::ShuttleRocket {
    let secret = get_mongo_secret(secret_store);
    let client = secret.as_str();
    let db = db_connection(client).await;
    let rocket = rocket::build()
        .manage(db)
        .attach(cors::cors::CORS)
        .mount("/", routes![index])
        .mount("/", routes![files])
        .mount("/", routes![routes::projects::get_projects]);
    // .mount("/", routes![get_project])
    // .mount("/", routes![get_posts])
    // .mount("/", routes![create_post])
    // .mount("/", routes![get_post]);
    Ok(rocket.into())
}

fn get_mongo_secret(secret_store: SecretStore) -> String {
    if let Some(secret) = secret_store.get("MONGO_STR") {
        secret
    } else {
        panic!("No secret found for MONGO_STR");
    }
}
