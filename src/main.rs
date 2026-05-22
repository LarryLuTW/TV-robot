use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use enigo::*;

use mime_guess::from_path;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "public/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
    match Asset::get(path) {
        Some(content) => {
            let mime = from_path(path).first_or_octet_stream();
            HttpResponse::Ok()
                .content_type(mime)
                .body(content.data.into_owned())
        }
        None => HttpResponse::NotFound().body("404 Not Found"),
    }
}

fn press(key: enigo::Key) {
    let settings = enigo::Settings::default();
    if let Ok(mut en) = Enigo::new(&settings) {
        // Use the key method with Direction::Click for a press and release
        let _ = en.key(key, enigo::Direction::Click);
    } else {
        eprintln!("Failed to initialize Enigo");
    }
}

#[cfg(target_os = "macos")]
fn sleep_system() {
    use std::process::Command;
    let _ = Command::new("pmset")
        .arg("sleepnow")
        .spawn()
        .map_err(|e| eprintln!("Failed to execute pmset sleepnow: {}", e));
}

#[cfg(not(target_os = "macos"))]
fn sleep_system() {
    // Sleep is only supported on macOS.
    eprintln!("sleep_system is a no-op on this platform");
}

async fn index() -> HttpResponse {
    handle_embedded_file("index.html")
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

async fn volume_down() -> impl Responder {
    press(Key::VolumeDown);
    "Ok"
}

async fn volume_up() -> impl Responder {
    press(Key::VolumeUp);
    "Ok"
}

async fn sleep_display() -> impl Responder {
    sleep_system();
    "Ok"
}

async fn platform() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(format!("{{\"os\":\"{}\"}}", std::env::consts::OS))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let ip = local_ip::get().unwrap().to_string();
    let url = format!("http://{}:3000/", ip);
    qr2term::print_qr(&url).unwrap();
    println!("Server is running at: {}", url);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/api/platform", web::get().to(platform))
            .route("/api/space", web::post().to(press_space))
            .route("/api/left", web::post().to(press_left))
            .route("/api/right", web::post().to(press_right))
            .route("/api/volume_down", web::post().to(volume_down))
            .route("/api/volume_up", web::post().to(volume_up))
            .route("/api/sleep", web::post().to(sleep_display))
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
