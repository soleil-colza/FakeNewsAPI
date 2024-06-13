use actix_web::{get, App, HttpServer, Responder};
use serde::Serialize;
use serde::Deserialize;
use serde_json::from_reader;
use std::fs::File;
use std::io::BufReader;
use std::env;

#[derive(Serialize, Deserialize)]
struct FakeNews {
    id: i32,
    title: String,
    date_published: String,
    authors: Vec<String>,
    content: String,
    tags: Vec<String>,
}

fn get_fake_news() -> Vec<FakeNews> {
    let file = File::open("fake_news.json").expect("Failed to open JSON file");
    let reader = BufReader::new(file);
    from_reader(reader).expect("Failed to load JSON")
}

#[get("/fake-news")]
async fn fake_news() -> impl Responder {
    let news = get_fake_news();
    actix_web::web::Json(news)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let port = port.parse::<u16>().expect("PORT must be a number");

    HttpServer::new(|| {
        App::new()
            .service(fake_news)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
