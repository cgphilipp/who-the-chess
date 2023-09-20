use axum::{
    extract::Query,
    extract::State,
    response::Html,
    routing::{get, post},
    Router,
};
use minijinja::{context, Environment};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex, RwLock},
};
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState<'a> {
    env: Environment<'a>,
    game_states: Arc<RwLock<HashMap<u32, GameState>>>,
}

#[derive(Clone)]
struct GameState {
    current_hint: u32,
}

#[derive(Debug, Serialize, Clone)]
struct Line {
    category: String,
    answer: String,
}

#[derive(Deserialize)]
struct GameRequest {
    game_id: u32,
}

fn get_lines(game_id: u32, hint_nr: u32) -> Vec<Line> {
    let lines = vec![
        Line {
            category: "Peak rating".to_string(),
            answer: "2882".to_string(),
        },
        Line {
            category: "Birth date".to_string(),
            answer: "30 November 1990".to_string(),
        },
        Line {
            category: "dasdsa".to_string(),
            answer: "1".to_string(),
        },
        Line {
            category: "asdasda".to_string(),
            answer: "2".to_string(),
        },
        Line {
            category: "asdasdas".to_string(),
            answer: "3".to_string(),
        },
    ];

    let last_hint = std::cmp::min(hint_nr as usize, lines.len());
    let display_lines = &lines[0..last_hint];

    display_lines.to_vec()
}

async fn introduction(State(state): State<Arc<RwLock<AppState<'_>>>>) -> Html<String> {
    let app_state = state.read().unwrap();

    let template = app_state.env.get_template("game").unwrap();
    let rendered = template.render(context!(start_screen => true));

    match rendered {
        Ok(result) => Html(result),
        Err(..) => Html("".into()),
    }
}

async fn start_game(
    State(state): State<Arc<RwLock<AppState<'_>>>>,
    request: Query<GameRequest>,
) -> Html<String> {
    println!("Start game [game_id {}]", request.game_id);

    let app_state = state.write().unwrap();
    app_state
        .game_states
        .write()
        .unwrap()
        .insert(request.game_id, GameState { current_hint: 2 });

    let template = app_state.env.get_template("game").unwrap();
    let rendered =
        template.render(context!(lines => get_lines(request.game_id, 1), start_screen => false));

    match rendered {
        Ok(result) => Html(result),
        Err(..) => Html("".into()),
    }
}

async fn get_category(
    State(state): State<Arc<RwLock<AppState<'_>>>>,
    request: Query<GameRequest>,
) -> Html<String> {
    println!("Request [game_id {}]", request.game_id);

    let app_state = state.read().unwrap();
    let mut writer = app_state.game_states.write().unwrap();

    let mut num_lines = 0;

    match writer.get_mut(&request.game_id) {
        Some(game_state) => {
            num_lines = game_state.current_hint;
            game_state.current_hint += 1;
        }
        None => return Html("".into()),
    }

    let template = app_state.env.get_template("playarea").unwrap();
    let rendered = template.render(context!(lines => get_lines(request.game_id, num_lines)));

    match rendered {
        Ok(result) => Html(result),
        Err(..) => Html("".into()),
    }
}

#[tokio::main]
async fn main() {
    let mut state = AppState {
        env: Environment::new(),
        game_states: Arc::new(RwLock::new(HashMap::new())),
    };

    state
        .env
        .add_template("game", include_str!("../../html/game.html"))
        .expect("Could not load a template!");
    state
        .env
        .add_template("playarea", include_str!("../../html/playarea.html"))
        .expect("Could not load a template!");

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
