use std::{io, path::PathBuf};
extern crate rocket;

use anyhow::Context;

use mongodb::{bson::doc, Client};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::{fs::NamedFile, get, routes};
use rocket::{Build, Rocket};
use serde::Deserialize;
use shuttle_runtime::SecretStore;
use utils::jwt::verify_token;

mod cors;
mod routes;
mod utils;

#[derive(Debug, Deserialize)]
struct AuthenticatedUser {
    _username: String,
}

struct MyState {
    jwt_token: Vec<u8>,
    username: String,
    hashed_password: String,
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
                        _username: data.claims.username,
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

#[get("/login")]
async fn login_s() -> io::Result<NamedFile> {
    let project_path = std::env::current_dir().unwrap();
    let build_path = project_path.join("src/front-end");
    NamedFile::open(build_path.join("login.html")).await
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
    let hashed_password: String = secret_store
        .get("USER_PASSWORD_HSH")
        .context("hash not found")?
        .to_string();
    let username: String = secret_store
        .get("USER_NAME")
        .context("username not found")?
        .to_string();
    let state = MyState {
        jwt_token,
        username,
        hashed_password,
    };
    let client = secret.as_str();
    let db = db_connection(client).await;
    let rocket = rocket::build()
        .manage(db)
        .manage(state)
        .attach(cors::cors::CORS)
        .mount("/", routes![index])
        .mount("/", routes![files])
        .mount("/", routes![login_s])
        .mount("/", routes![routes::admin::login])
        .mount("/", routes![routes::posts::get_posts])
        .mount("/", routes![routes::posts::create_post])
        .mount("/", routes![routes::projects::create_project])
        .mount("/", routes![routes::projects::get_projects])
        .mount("/", routes![routes::projects::update_project])
        .mount("/", routes![routes::projects::get_project]);
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

fn build_rocket(db: mongodb::Database, state: MyState) -> Rocket<Build> {
    rocket::build()
        .manage(db)
        .manage(state)
        .attach(cors::cors::CORS)
        .mount("/", routes![index])
        .mount("/", routes![files])
        .mount("/", routes![login_s])
        .mount("/", routes![routes::admin::login])
        .mount("/", routes![routes::posts::get_posts])
        .mount("/", routes![routes::posts::create_post])
        .mount("/", routes![routes::projects::create_project])
        .mount("/", routes![routes::projects::get_projects])
        .mount("/", routes![routes::projects::update_project])
        .mount("/", routes![routes::projects::get_project])
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;
    use tokio;

    async fn setup() -> Client {
        let mongo_str = std::env::var("MONGO_STR").expect("MONGO_STR must be set");
        let jwt_token = std::env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set")
            .as_bytes()
            .to_vec();
        let username = std::env::var("USER_NAME").expect("USER_NAME must be set");
        let hashed_password =
            std::env::var("USER_PASSWORD_HSH").expect("USER_PASSWORD_HSH must be set");

        // Establish a database connection
        let db = db_connection(&mongo_str).await;

        // Initialize state with the read environment variables
        let state = MyState {
            jwt_token,
            username,
            hashed_password,
        };
        let rocket = build_rocket(db, state);
        Client::tracked(rocket)
            .await
            .expect("valid rocket instance")
    }

    #[tokio::test]
    async fn test_get_project() {
        let client = setup().await;
        let response = client
            .get("/api/v1/projects/6543ea5d875bc6bcda7d9218")
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
    }

    #[tokio::test]
    async fn test_get_projects() {
        let client = setup().await;
        let response = client.get("/api/v1/projects").dispatch().await;
        assert_eq!(response.status(), Status::Ok);
    }

    #[tokio::test]
    async fn test_get_posts() {
        let client = setup().await;
        let response = client.get("/api/v1/posts").dispatch().await;
        assert_eq!(response.status(), Status::Ok);
    }
}
