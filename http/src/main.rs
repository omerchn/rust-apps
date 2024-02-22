use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn root() -> impl Responder {
    format!("root")
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

const PORT: u16 = 9000;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(greet).service(root))
        .bind(("127.0.0.1", PORT))?
        .run()
        .await
}
