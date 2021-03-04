use actix_web::{get, web, App, HttpServer, Responder};

mod poke;

static POKE_API_BASE_URL: &str = "https://pokeapi.co/api/v2/pokemon";
static SHAKESPEARE_TRANSLATOR_BASE_URL: &str = "https://api.funtranslations.com/translate/shakespeare.json";

#[get("/pokemon/{name}")]
async fn index(web::Path((name)): web::Path<(String)>) -> impl Responder {
    poke::poke_client::get_pokemon(POKE_API_BASE_URL, &name).await.unwrap();
    format!("Hello {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}