use colored::*;

// A macro that is used to print an error message and exit the program.
macro_rules! error {
    ($a:expr) => {{
        println!("{} {}", String::from("ERROR   :").red().bold(), $a);
        std::process::exit(1);
    }};
}

// A macro that is used to print a warning message.
macro_rules! warning {
    ($a:expr) => {{
        println!("{} {}", String::from("WARNING :").yellow().bold(), $a);
    }};
}

// A macro that is used to print an information message.
macro_rules! info {
    ($a:expr) => {{
        println!("{} {}", String::from("INFO    :").green().bold(), $a);
    }};
}

// A macro that is used to render a template.
macro_rules! render {
    ($a:expr, $b:expr, $c:expr) => {{
        info!(format!("Trying to render {}", $a));
        let rendered_template: Option<String> = match $c.render_engine.render($a, &$b) {
            Ok(rendered_template) => {
                info!(format!("Rendered template: {} suceeded", $a));
                Some(rendered_template)
            }
            Err(err) => {
                warning!(format!(
                    "Rendering template {} failed! Reason: {}",
                    $a,
                    err.to_string()
                ));
                None
            }
        };
        rendered_template
    }};
}

// `AppData` is a struct that contains a single field, `render_engine`, which is a `tera::Tera` type.
// The `tera` crate is a crate that is used to render templates.
struct AppData {
    render_engine: tera::Tera,
}

#[actix_web::get("/")]
async fn index(conf: actix_web::web::Data<AppData>) -> impl actix_web::Responder {
    info!("Got request for index...");

    let context = tera::Context::new();

    match render!("index.html", context, conf) {
        Some(html) => {
            info!("Request was successfully processed");
            return actix_web::HttpResponse::Ok().body(html);
        }
        None => {
            return actix_web::HttpResponse::InternalServerError().finish();
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!("Trying to load environment variables from .env file");

    // It loads environment variables from a .env file.
    match dotenv::dotenv() {
        Ok(path) => info!(format!(
            "Environment variables were loaded from {}",
            path.into_os_string().into_string().unwrap()
        )),
        Err(e) => error!(format!(
            "Failed to load environment variables from .env. Reason: {}",
            e
        )),
    };

    // A variable that holds the port number that the server will listen to.
    let port: u16 = match std::env::var("CSSING_PORT") {
        Ok(port) => {
            info!(format!("Port = {}", port));

            match port.parse() {
                Ok(p)  => p,
                Err(_) => error!("Failed to parse port environment variable. Reason: Incorrect format (try using number)")
            }
        }
        Err(_) => error!("Failed to read environment variable 'CSSING_PORT'"),
    };

    // `tmplfolder` is a variable that holds the path to the folder that contains the templates.
    let tmplfolder = match std::env::var("CSSING_TEMPLATES_FOLDER") {
        Ok(dir) => {
            info!(format!("Templates folder = {}", dir));
            dir
        }
        Err(_) => error!("Failed to read environment variable 'CSSING_TEMPLATES_FOLDER'"),
    };

    info!("Initializing `tera` rendering engine...");

    // Initializing the `tera` rendering engine.
    let tera = tera::Tera::new(&tmplfolder).expect("Failed to initialize tera rendering engine");

    // `app_data` is a variable that holds the `AppData` struct.
    let app_data = actix_web::web::Data::new(AppData {
        render_engine: tera.clone(),
    });

    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .service(index)
            .app_data(app_data.clone())
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
