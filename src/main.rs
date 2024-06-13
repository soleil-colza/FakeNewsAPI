use actix_web::{get, App, HttpServer, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct FakeNews {
    title: String,
    date_published: String,
    authors: Vec<String>,
    content: String,
    tags: Vec<String>,
}

fn get_fake_news() -> Vec<FakeNews> {
    vec![
        FakeNews {
            title: "title1".to_string(),
            date_published: "2024-06-13T12:00:00Z".to_string(),
            authors: vec!["author1".to_string(), "author2".to_string()],
            content: "this is the content1".to_string(),
            tags: vec!["tag1".to_string(), "tag2".to_string()],
        },
        FakeNews {
            title: "title2".to_string(),
            date_published: "2024-06-14T12:00:00Z".to_string(),
            authors: vec!["author".to_string()],
            content: "this is the content2".to_string(),
            tags: vec!["tag3".to_string(), "tag4".to_string()],
        },
    ]
}
