use actix_web::{web, App, HttpResponse, HttpServer};
use std::sync::Mutex;

struct AppState {
    counter: Mutex<u32>,
}

#[actix_web::main]
async fn main() {
    let data = web::Data::new(AppState {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/", web::get().to(increase_counter))
    })
    .workers(4)
    .bind("127.0.0.1:8080")
    .expect("Failed to bind to address: 127.0.0.1:8080")
    .run()
    .await
    .expect("Server failed to start");
}

async fn increase_counter(data: web::Data<AppState>) -> HttpResponse {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    HttpResponse::Ok().body(format!("This page has been visited {} times", *counter))
}
