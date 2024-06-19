use crate::{utils::jwt::create_token, MyState};
use rocket::serde::json::{json, Value};
use rocket::{http::Status, serde::json::Json};
use rocket::{post, response::status::Custom, State};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct UserData {
    username: String,
    hashed_password: String,
}

#[post("/api/v1/login", data = "<user>")]
pub async fn login(
    user: Json<UserData>,
    state: &State<MyState>,
) -> Result<Json<String>, Custom<Json<Value>>> {
    if user.username != state.username {
        return Err(Custom(
            Status::Unauthorized,
            Json(json!({"error": "Incorrect username"})),
        ));
    }

    if user.hashed_password.as_bytes() != state.hashed_password {
        return Err(Custom(
            Status::Unauthorized,
            Json(json!({"error": "Incorrect password"})),
        ));
    }
    match create_token(&user.username, &state.jwt_token) {
        Ok(token) => Ok(Json(token)),
        Err(_) => Err(Custom(
            Status::InternalServerError,
            Json(json!({"error": "Internal Server Error" })),
        )),
    }
}
