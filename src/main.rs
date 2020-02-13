extern crate actix_rt;
extern crate actix_web;
extern crate enigo;
extern crate local_ip;
extern crate qr2term;

use actix_files::NamedFile;
use actix_web::{web, App, HttpRequest, HttpServer, Responder, Result};

use enigo::*;

fn press(key: enigo::Key) {
    let mut en = Enigo::new();
    en.key_down(key);
}

async fn index(_req: HttpRequest) -> Result<NamedFile> {
    Ok(NamedFile::open("index.html")?)
}

async fn press_space() -> impl Responder {
    press(Key::Space);
    "Ok"
}

async fn press_left() -> impl Responder {
    press(Key::LeftArrow);
    "Ok"
}

async fn press_right() -> impl Responder {
    press(Key::RightArrow);
    "Ok"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let ip = local_ip::get().unwrap().to_string();
    let url = format!("http://{}:3000/", ip);
    qr2term::print_qr(&url).unwrap();

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/api/space", web::post().to(press_space))
            .route("/api/left", web::post().to(press_left))
            .route("/api/right", web::post().to(press_right))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
