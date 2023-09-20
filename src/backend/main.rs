use axum::{
    routing::{get, post},
    Router, response::Html, extract::Query
};
use std::{net::SocketAddr, ops::RangeTo};
use serde::{Serialize, Deserialize};
use minijinja::render;
use tower_http::services::ServeDir;

#[derive(Debug, Serialize)]
struct Line {
    category: String,
    answer: String,
}

#[derive(Deserialize)]
struct CategoryRequest {
    id: u32,
}

async fn introduction() -> Html<String> {
    let template = std::fs::read_to_string("html/game.html").unwrap();
    let r = render!(&template, lines => {}, start_screen => true);
    Html(r)
}

async fn start_game() -> Html<String> {
    let template = std::fs::read_to_string("html/game.html").unwrap();
    let r = render!(&template, lines => {}, start_screen => false);
    Html(r)
}

async fn get_category(request: Query<CategoryRequest>) -> Html<String> {
    println!("request get_category with id {}", request.id);

    let lines = vec![
        Line{
            category: "Peak rating".to_string(),
            answer: "2882".to_string()
        },
        Line{
            category: "Birth date".to_string(),
            answer: "30 November 1990".to_string()
        },
        Line{
            category: "dasdsa".to_string(),
            answer: "1".to_string()
        },
        Line{
            category: "asdasda".to_string(),
            answer: "2".to_string()
        },
        Line{
            category: "asdasdas".to_string(),
            answer: "3".to_string()
        }
    ];

    let id = std::cmp::min(request.id as usize, lines.len());

    if id > lines.len()
    {
        return Html("Error occured".to_string())
    }
    let shown_lines = &lines[RangeTo{end: id as usize}];

    let template = std::fs::read_to_string("html/game.html").unwrap();
    let r = render!(&template, lines => shown_lines, start_screen => false);
    Html(r)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
    .route("/", get(introduction))
    .route("/category", get(get_category))
    .route("/start_game", get(start_game))
    .nest_service("/assets", ServeDir::new("assets"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}...", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
