use std::path::Path;

use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, Result, HttpResponse, Error, http::header::ContentType};
use serde::Serialize;

#[derive(Serialize)]
struct ResObj {
    status: i8,
    message: String,
}

async fn index() -> Result<NamedFile> {
    return Ok(NamedFile::open("./build/index.html")?);
}

async fn react_pages(path: web::Path<String>) -> Result<NamedFile> {
    let page = path.into_inner();
    println!("{}", page.as_str());

    let file_path = "./build/".to_string() + (page.as_str());

    if Path::new(file_path.as_str()).is_file() {
        return Ok(NamedFile::open(file_path)?);
    } else {
        return Ok(NamedFile::open("./build/index.html")?);
    }
}

async fn api_index() -> Result<HttpResponse, Error> {
    let obj = ResObj {
        status: 0,
        message: "api index path !".to_string()
    };
    Ok(HttpResponse::Ok().content_type(ContentType::json()).json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(
        || {
            App::new()
                .service(actix_files::Files::new("/static", "./build/static").show_files_listing())
                .route("/", web::get().to(index))
                .route("/{page}", web::get().to(react_pages))
                .service(
                    web::scope("/api")
                        .route("/", web::get().to(api_index))
                )
        }
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}