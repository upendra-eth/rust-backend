use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "Ram Ram Ji "
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Ram Ram {} ji ", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(hello))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}