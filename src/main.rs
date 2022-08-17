#[actix_web::get("/")]
async fn index() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| actix_web::App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
