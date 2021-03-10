use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use actix_files::NamedFile;
use std::path::PathBuf;
use std::io::Error;
use std::fs::File;
use std::io::Read;


const PATH_TO_FILE: &str = "/home/..."; // TODO add your filepath

async fn send(_req: HttpRequest) -> Result<NamedFile, Error> {
   
    let mut pb = PathBuf::new();
    pb.push(PATH_TO_FILE);
    Ok(NamedFile::open(pb.as_path())?)
}

async fn show(_req: HttpRequest) -> impl Responder {
   
    let mut pb = PathBuf::new();
    pb.push(PATH_TO_FILE);

    let mut file = File::open(pb).unwrap();
    let mut res = String::new();
    file.read_to_string(&mut res).unwrap();
   
    res

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   
    HttpServer::new(|| {
        App::new()
            .route("/data.csv", web::get().to(send))
            .route("/plain.csv", web::get().to(show))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
