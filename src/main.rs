use std::process::Command;
use std::time::Duration;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use enigo::*;
use serde::Deserialize;
use tokio::time::sleep;

use mime_guess::from_path;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "public/"]
struct Asset;

#[derive(Deserialize, Debug)]
struct DisplayInfo {
    #[serde(rename = "SPDisplaysDataType")]
    gpu_data: Vec<GpuData>,
}

#[derive(Deserialize, Debug)]
struct GpuData {
    #[serde(rename = "spdisplays_ndrvs")]
    displays: Vec<Display>,
}

#[derive(Deserialize, Debug)]
struct Display {
    // We only need this struct to count displays, so we'll use serde(skip) for unused fields
}

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

fn get_volume() -> i8 {
    let script = "osascript -e 'output volume of (get volume settings)'";
    let output = Command::new("sh")
        .arg("-c")
        .arg(script)
        .output()
        .expect("failed to get volume");

    let vol = String::from_utf8_lossy(&output.stdout)
        .trim_end()
        .parse()
        .expect("failed to parse volume into integer");

    return vol;
}

fn set_volume(vol: i8) {
    let script = format!("osascript -e 'set Volume output volume {}'", vol);
    Command::new("sh")
        .arg("-c")
        .arg(script)
        .spawn()
        .expect("failed to spawn process")
        .wait()
        .unwrap();
}

fn count_total_displays() -> usize {
    let output = match Command::new("system_profiler")
        .arg("SPDisplaysDataType")
        .arg("-json")
        .output()
    {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Failed to run system_profiler: {}", e);
            return 0;
        }
    };

    let json_str = match String::from_utf8(output.stdout) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to parse system_profiler output: {}", e);
            return 0;
        }
    };

    match serde_json::from_str::<DisplayInfo>(&json_str) {
        Ok(display_info) => {
            let total_displays = display_info.gpu_data.iter()
                .map(|gpu| gpu.displays.len())
                .sum();
            total_displays
        }
        Err(e) => {
            eprintln!("Failed to parse display JSON: {}", e);
            // Fallback: count lines containing display names
            json_str.matches("\"_name\"").count()
        }
    }
}

fn sleep_system() {
    println!("Putting system to sleep...");
    // Use a more direct approach that should definitely work
    let _ = Command::new("osascript")
        .arg("-e")
        .arg("tell application \"System Events\" to sleep")
        .spawn()
        .map_err(|e| {
            eprintln!("Failed to execute osascript sleep: {}, trying pmset...", e);
            // Fallback to pmset
            let _ = Command::new("pmset")
                .arg("sleepnow")
                .spawn()
                .map_err(|e2| eprintln!("Failed to execute pmset sleepnow: {}", e2));
        });
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
    let current_vol = get_volume();
    set_volume(current_vol - 7);
    "Ok"
}

async fn volume_up() -> impl Responder {
    let current_vol = get_volume();
    set_volume(current_vol + 7);
    "Ok"
}

async fn sleep_display() -> impl Responder {
    sleep_system();
    "Ok"
}

async fn monitor_displays() {
    let mut previous_display_count = count_total_displays();
    println!("Initial total displays detected: {}", previous_display_count);
    
    loop {
        sleep(Duration::from_secs(1)).await; // Check every 1 second for quick response
        
        let current_display_count = count_total_displays();
        
        // If display count decreased, trigger sleep
        if current_display_count < previous_display_count {
            println!("Display count decreased: {} -> {}. Triggering system sleep...", 
                     previous_display_count, current_display_count);
            sleep_system();
            
            // Wait a bit for sleep command to take effect, then immediately resume monitoring
            sleep(Duration::from_secs(3)).await;
            
            // Reset the count and continue monitoring - the system will naturally pause
            // monitoring while asleep and resume when it wakes up
            previous_display_count = count_total_displays();
            println!("System awake again. Resuming monitoring with {} displays", previous_display_count);
            continue;
        }
        
        // If the count changed, log it (but only if it's not a decrease to avoid spam)
        if current_display_count != previous_display_count {
            println!("Display count changed: {} -> {}", 
                     previous_display_count, current_display_count);
        }
        
        previous_display_count = current_display_count;
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let ip = local_ip::get().unwrap().to_string();
    let url = format!("http://{}:3000/", ip);
    qr2term::print_qr(&url).unwrap();
    println!("Server is running at: {}", url);

    // Start display monitoring in background
    tokio::spawn(monitor_displays());

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
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
