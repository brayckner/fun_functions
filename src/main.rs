use actix_web::{web, App, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| App::new().route("/", web::get().to(HttpResponse::Ok))).workers(10)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}