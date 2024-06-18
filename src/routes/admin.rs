use crate::{
    utils::jwt::{create_token, verify_token},
    MyState,
};
use rocket::{http::Status, serde::json::Json};
use rocket::{post, response::status::Custom, State};
use rocket::{
    request::{FromRequest, Outcome, Request},
    serde::json::{json, Value},
};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct AuthenticatedUser {
    username: String,
}

#[derive(Deserialize, Debug)]
struct UserData {
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
                        username: data.claims.username,
                    }),
                    Err(_) => Outcome::Error((Status::Unauthorized, ())),
                }
            }
            None => Outcome::Error((Status::InternalServerError, ())),
        }
    }
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
