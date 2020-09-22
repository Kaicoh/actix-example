use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use models::Hello;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(Hello {
        msg: "Hello world!".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
