use std::io::{Error, ErrorKind};
use std::str::FromStr;

use axum::http::{header, Method};
use axum::Json;
use axum::{routing::get, Router};

use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_headers([header::CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    let app = Router::new()
        .route("/questions", get(get_questions))
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!(
        "->> LISTENING on: {:?}\n",
        listener
            .local_addr()
            .expect("Failed to get local address of the listener")
    );

    axum::serve(listener, app).await.unwrap();
}

async fn get_questions() -> Json<Question> {
    let question = Question::new(
        QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of the question".to_string(),
        Some(vec!["faq".to_string()]),
    );

    Json(question)
}

#[derive(Debug, Deserialize, Serialize)]
struct QuestionId(String);

#[derive(Debug, Deserialize, Serialize)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Self {
            id,
            title,
            content,
            tags,
        }
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No ID provided")),
        }
    }
}
