use actix_web::{get, web, App, HttpServer, Responder};

#[get("/pokemon/{name}")]
async fn index(web::Path((name)): web::Path<(String)>) -> impl Responder {
    format!("Hello {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}