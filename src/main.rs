use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::Local;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello from the base path of Fun Functions")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    let current_time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let response_string = format!("[{}] - Welcome to fun functions: {}", current_time, req_body);
    HttpResponse::Ok().body(response_string)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(echo)
            .service(hello)
            .route("/", web::get().to(HttpResponse::Ok))
    })
    .workers(10)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}