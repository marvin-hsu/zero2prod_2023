use actix_web::{HttpServer, App, web, HttpRequest, Responder, HttpResponse};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(||{
        App::new()
        .route("/health_check", web::get().to(health_check))
    })
    .bind(("127.0.0.1",8080))?
    .run()
    .await
}

async fn health_check(_req: HttpRequest) -> impl Responder{
    HttpResponse::Ok()
}
