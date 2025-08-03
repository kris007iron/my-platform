use rocket::{
    fairing::{Fairing, Info, Kind},
    http::{Header, Method, Status},
    Request, Response,
};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "CORS Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let origin = request.headers().get_one("Origin").unwrap_or_default();
        let allowed_origins = [
            "http://127.0.0.1:8000",
            "localhost:8000",
            "localhost:8000/login",
            "https://kris007iron-o9ms.shuttle.app",
        ];

        if allowed_origins.contains(&origin) {
            response.set_header(Header::new("Access-Control-Allow-Origin", origin));
            response.set_header(Header::new(
                "Access-Control-Allow-Methods",
                "GET, POST, OPTIONS",
            ));
            response.set_header(Header::new(
                "Access-Control-Allow-Headers",
                "Content-Type, Authorization",
            ));
            response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        }

        if request.method() == Method::Options {
            response.set_status(Status::Ok);
            response.set_header(Header::new("Content-Length", "0"));
        }
    }
}
