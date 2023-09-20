use axum::{
    routing::{get, post},
    Router, response::Html, extract::Query, extract::{State, FromRef}
};
use std::{net::SocketAddr, ops::RangeTo, rc::Rc, sync::{RwLock, Mutex, Arc}, collections::HashMap};
use serde::{Serialize, Deserialize};
use minijinja::{Environment, render, context};
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState<'a> {
    env: Environment<'a>,
    game_states: Arc<HashMap<u32, Mutex<GameState>>>,
}

#[derive(Clone)]
struct GameState {
    game_id: u32,
    current_hint: u32,
}

#[derive(Debug, Serialize, Clone)]
struct Line {
    category: String,
    answer: String,
}

#[derive(Deserialize)]
struct CategoryRequest {
    id: u32,
}

fn get_lines(u32: u64, hint_nr: u32) -> Vec<Line> {
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

    let last_hint = std::cmp::min(hint_nr as usize, lines.len());
    let display_lines = &lines[0..last_hint];

    display_lines.to_vec()
}

async fn introduction(State(state): State<Arc<RwLock<AppState<'_>>>>) -> Html<String> {
    let app_state = state.read().unwrap();

    let template = app_state.env.get_template("game").unwrap();
    let rendered = template.render(
        context!(start_screen => true)
    );

    match rendered {
        Ok(result) => Html(result),
        Err(..) => Html("".into())
    }
}

async fn start_game(State(state): State<Arc<RwLock<AppState<'_>>>>) -> Html<String> {
    let app_state = state.read().unwrap();

    let template = app_state.env.get_template("game").unwrap();
    let rendered = template.render(
        context!(lines => get_lines(0, 1), start_screen => false)
    );

    match rendered {
        Ok(result) => Html(result),
        Err(..) => Html("".into())
    }
}

async fn get_category(State(state): State<Arc<RwLock<AppState<'_>>>>, request: Query<CategoryRequest>) -> Html<String> {
    // println!("request get_category with id {}", request.id);

    let app_state = state.read().unwrap();

    let template = app_state.env.get_template("playarea").unwrap();
    let rendered = template.render(
        context!(lines => get_lines(0, request.id))
    );

    match rendered {
        Ok(result) => Html(result),
        Err(..) => Html("".into())
    }
}

#[tokio::main]
async fn main() {
    let mut state = AppState {
        env: Environment::new(),
        game_states: Arc::new(HashMap::new()),
    };

    state.env.add_template("game", include_str!("../../html/game.html")).expect("Could not load a template!");
    state.env.add_template("playarea", include_str!("../../html/playarea.html")).expect("Could not load a template!");

    let shared_state = Arc::new(RwLock::new(state));

    let app = Router::new()
    .route("/", get(introduction))
    .route("/category", get(get_category))
    .route("/start_game", get(start_game))
    .nest_service("/assets", ServeDir::new("assets"))
    .with_state(shared_state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}...", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
