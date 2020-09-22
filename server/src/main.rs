extern crate derive_more;

use actix_web::{
    dev::HttpResponseBuilder, error, get, http::header, http::StatusCode,
    App, HttpResponse, HttpServer, Responder,
};
use derive_more::{Display, Error};
use models::Hello;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(Hello {
        msg: "Hello world!".to_string(),
    })
}

#[get("/error")]
async fn hello_err() -> Result<&'static str, MyError> {
    Err(MyError::BadRequest)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(hello_err)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}

#[derive(Debug, Display, Error)]
enum MyError {
    #[display(fmt = "bad request")]
    BadRequest,
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .set_header(header::CONTENT_TYPE, "text/plain; charset=utf-8")
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::BadRequest => StatusCode::BAD_REQUEST,
        }
    }
}
