use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use std::env;
use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize)]
struct FakeNews {
    id: i32,
    title: String,
    date_published: String,
    authors: Vec<String>,
    content: String,
    tags: Vec<String>,
}

fn get_fake_news() -> Result<Vec<FakeNews>, String> {
    let file =
        File::open("news_content.json").map_err(|e| format!("Failed to open JSON file: {}", e))?;
    let reader = BufReader::new(file);
    let news: Vec<FakeNews> =
        from_reader(reader).map_err(|e| format!("Failed to load JSON: {}", e))?;
    Ok(news)
}

#[get("/fake-news")]
async fn fake_news() -> impl Responder {
    match get_fake_news() {
        Ok(news) => HttpResponse::Ok().json(news),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port = port.parse::<u16>().expect("PORT must be a number");

    HttpServer::new(|| App::new().service(fake_news))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
