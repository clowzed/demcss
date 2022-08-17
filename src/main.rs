use colored::*;

macro_rules! error {
    ($a:expr) => {{
        println!("{} {}", String::from("ERROR   :").red().bold(), $a);
        std::process::exit(1);
    }};
}

macro_rules! warning {
    ($a:expr) => {{
        println!("{} {}", String::from("WARNING :").yellow().bold(), $a);
    }};
}

macro_rules! info {
    ($a:expr) => {{
        println!("{} {}", String::from("INFO    :").green().bold(), $a);
    }};
}

struct AppData {
    render_engine: tera::Tera,
}

#[actix_web::get("/")]
async fn index() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("hello")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = tera::Tera::new("/templates").expect("Failed to initialize tera rendering engine");

    let app_data = actix_web::web::Data::new(std::sync::Mutex::new(AppData {
        render_engine: tera.clone(),
    }));

    actix_web::HttpServer::new(|| actix_web::App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
