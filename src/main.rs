use axum::{routing::get, Json, Router};
use std::{collections::HashMap, fs, net::SocketAddr};
use tokio::sync::OnceCell;
use regex::Regex;
use dotenv::dotenv;
use std::env;

static FILE_MAP: OnceCell<HashMap<String, String>> = OnceCell::const_new();
const GDRIVE_BASE_URL: &str = "https://drive.google.com/uc?id=";
// const GDRIVE_BASE_URL: &str = "https://drive.usercontent.google.com/download?id=";

async fn parse_log_file(log_file: &str) -> HashMap<String, String> {
    let mut mapping = HashMap::new();
    let log_data = fs::read_to_string(log_file).expect("Failed to read log file");
    let pattern = Regex::new(r"Uploading file '.*?/([^/]+)' with id: ([\w-]+)").unwrap();
    println!("Parsing log file...");
    for line in log_data.lines() {
        if let Some(caps) = pattern.captures(line) {
            let filename = caps[1].to_string();
            let file_id = caps[2].to_string();
            println!("Indexing {}: {}", filename, file_id);
            mapping.insert(filename, file_id);
        }
    }
    println!("Total indexed IDs: {}", mapping.len());
    mapping
}

async fn get_files() -> Json<HashMap<String, String>> {
    let mapping = FILE_MAP.get().expect("FILE_MAP is not initialized");
    let urls: HashMap<String, String> = mapping
        .iter()
        .map(|(filename, file_id)| (filename.clone(), format!("{}{}", GDRIVE_BASE_URL, file_id)))
        .collect();
    Json(urls)
}

async fn get_file(filename: axum::extract::Path<String>) -> Json<Option<String>> {
    let mapping = FILE_MAP.get().expect("FILE_MAP is not initialized");
    Json(mapping.get(&filename.0).map(|file_id| format!("{}{}", GDRIVE_BASE_URL, file_id)))
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let port: u16 = env::var("PORT").unwrap_or_else(|_| "3000".to_string()).parse().expect("Invalid PORT value");
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // Initialize FILE_MAP
    let mapping = parse_log_file("upload_log.txt").await;
    FILE_MAP.set(mapping).expect("Failed to set FILE_MAP");

    println!("Server running on http://{}", addr);
    let app = Router::new()
        .route("/files", get(get_files))
        .route("/files/:filename", get(get_file));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
