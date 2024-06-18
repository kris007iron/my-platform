use std::{io, path::PathBuf};
extern crate rocket;

use anyhow::Context;
use mongodb::{bson::doc, Client};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::json::Json;
use rocket::{fs::NamedFile, get, post, routes, State};
use serde::Deserialize;
use shuttle_runtime::SecretStore;
use utils::jwt::{create_token, verify_token, Claims};

mod cors;
mod routes;
mod utils;

#[derive(Debug, Deserialize)]
struct AuthenticatedUser {
    username: String,
}

struct MyState {
    jwt_token: Vec<u8>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, ()> {
        let keys: Vec<_> = request.headers().get("Authorization").collect();
        if keys.is_empty() {
            return Outcome::Error((Status::Unauthorized, ()));
        }

        let token = keys[0].trim_start_matches("Bearer ");
        let state = request.rocket().state::<MyState>();

        match state {
            Some(state) => {
                // Use `state.jwt_token` here if needed for token verification
                match verify_token(token, &state.jwt_token) {
                    Ok(data) => Outcome::Success(AuthenticatedUser {
                        username: data.claims.username,
                    }),
                    Err(_) => Outcome::Error((Status::Unauthorized, ())),
                }
            }
            None => Outcome::Error((Status::InternalServerError, ())),
        }
    }
}

async fn db_connection(client: &str) -> mongodb::Database {
    Client::with_uri_str(client)
        .await
        .unwrap()
        .database("PortfolioAPI")
}

/*Routes for testing

#[derive(Deserialize)]
struct InputData {
    name: String,
    age: u32,
}

#[post("/submit", data = "<input_data>")]
fn submit(input_data: Json<InputData>, user: AuthenticatedUser) -> String {
    format!(
        "Received data from user: {}. Name: {}, Age: {}",
        user.username, input_data.name, input_data.age
    )
}

// Generate JWT for testing purposes
#[post("/login", data = "<user>")]
async fn login(
    user: Json<AuthenticatedUser>,
    state: &State<MyState>,
) -> Result<Json<String>, Status> {
    match create_token(&user.username, &state.jwt_token) {
        Ok(token) => Ok(Json(token)),
        Err(_) => Err(Status::InternalServerError),
    }
}

End of routes for testing*/

// TODO: when developing SPA in vue add regex to let builtin router handle the routes
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
    let secret = get_mongo_secret(&secret_store);
    let jwt_token: Vec<u8> = secret_store
        .get("JWT_SECRET")
        .context("secret not found")?
        .as_bytes()
        .to_vec();
    let state = MyState { jwt_token };
    let client = secret.as_str();
    let db = db_connection(client).await;
    let rocket = rocket::build()
        .manage(db)
        .manage(state)
        .attach(cors::cors::CORS)
        .mount("/", routes![index])
        .mount("/", routes![files])
        .mount("/", routes![routes::posts::get_posts])
        .mount("/", routes![routes::projects::get_projects]);
        //.mount("/", routes![submit])
        //.mount("/", routes![login]);
    // .mount("/", routes![get_project])
    // .mount("/", routes![get_posts])
    // .mount("/", routes![create_post])
    // .mount("/", routes![get_post]);
    Ok(rocket.into())
}

fn get_mongo_secret(secret_store: &SecretStore) -> String {
    if let Some(secret) = secret_store.get("MONGO_STR") {
        secret
    } else {
        panic!("No secret found for MONGO_STR");
    }
}
