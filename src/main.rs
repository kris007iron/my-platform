use rocket::fs::relative;
use std::path::Path;
use std::{io, path::PathBuf};
extern crate rocket;

use anyhow::Context;

use mongodb::{bson::doc, Client};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::{fs::NamedFile, get, routes};

use serde::Deserialize;
use shuttle_runtime::SecretStore;
use utils::jwt::verify_token;

use tracing::info;

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
    let mut file = Path::new(relative!("front-end")).join(file);
    if file.is_dir() {
        file.push("index.html");
    }
    info!("{:?}", file);
    NamedFile::open(file).await.ok()
}

#[get("/")]
async fn index() -> io::Result<NamedFile> {
    let project_path = std::env::current_dir().unwrap();
    let build_path = project_path.join("front-end");
    NamedFile::open(build_path.join("index.html")).await
}

#[get("/login")]
async fn login_s() -> io::Result<NamedFile> {
    let project_path = std::env::current_dir().unwrap();
    let build_path = project_path.join("front-end");
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

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;
    use rocket::{Build, Rocket};
    use tokio;

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

    async fn setup() -> Client {
        //dotenv::dotenv().ok();
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

    // #[tokio::test]
    // async fn test_update_project() {
    //     use rocket::http::ContentType;
    //     use std::fs::File;
    //     use std::io::{Cursor, Read};

    //     let client = setup().await;

    //     let mut file =
    //         File::open("src/front-end/imgs/projects/foodie.png").expect("Image file not found");
    //     let mut image_data = Vec::new();
    //     file.read_to_end(&mut image_data)
    //         .expect("Failed to read image file");

    //     // Encode image data as a base64 string
    //     let image_base64 = base64::encode(image_data);

    //     // Creating the multipart form data as a String
    //     let form_data = format!(
    //         "--boundary\r\n\
    //     Content-Disposition: form-data; name=\"title\"\r\n\r\n\
    //     Updated Project Title\r\n\
    //     --boundary\r\n\
    //     Content-Disposition: form-data; name=\"description\"\r\n\r\n\
    //     Updated description\r\n\
    //     --boundary\r\n\
    //     Content-Disposition: form-data; name=\"link\"\r\n\r\n\
    //     https://updated.link\r\n\
    //     --boundary\r\n\
    //     Content-Disposition: form-data; name=\"tags\"\r\n\r\n\
    //     rust,backend,api\r\n\
    //     --boundary\r\n\
    //     Content-Disposition: form-data; name=\"image\"; filename=\"image.png\"\r\n\
    //     Content-Type: image/png\r\n\r\n\
    //     < ./src/front-end/imgs/projects/foodie.png\r\n\
    //     --boundary--"
    //     );

    //     // Convert the string to a byte stream
    //     let body = Cursor::new(form_data);

    //     // Convert the cursor to a byte slice
    //     let body_bytes: &[u8] = body.get_ref().as_bytes();

    //     // Send the PATCH request
    //     let response = client
    //         .patch("/api/v1/projects/6543ea5d875bc6bcda7d9218")
    //         .header(ContentType::new(
    //             "multipart",
    //             "form-data; boundary=boundary",
    //         ))
    //         .body(body_bytes)
    //         .dispatch()
    //         .await;

    //     // Check the response status
    //     assert_eq!(response.status(), rocket::http::Status::Ok);
    // }
}
