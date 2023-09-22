use axum::{
    extract::Query,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Router,
};
use minijinja::{context, Environment};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use tower_http::services::ServeDir;

#[derive(Clone)]
struct AppState<'a> {
    env: Environment<'a>,
    game_states: Arc<RwLock<HashMap<u32, GameState>>>,
    player_infos: Arc<HashMap<String, PlayerInfo>>,
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

#[derive(Clone)]
struct PlayerDisplay {
    lines: Vec<Line>,
    image: String,
}

#[derive(Deserialize)]
struct GameRequest {
    game_id: u32,
}

#[derive(Deserialize)]
struct AnswerRequest {
    game_id: u32,
    name: String,
}

#[derive(Deserialize)]
struct PlayerInfo {
    birth_date: String,
    birth_place: String,
    year_of_gm: i32,
    chess_com_name: HashSet<String>,
    lichess_name: HashSet<String>,
    peak_rating: u32,
    sport_country: String,
    citizenship_country: String,
    images: HashSet<String>,
}

fn get_answer(player_infos: &HashMap<String, PlayerInfo>, game_id: u32) -> String {
    let player_id = game_id as usize % player_infos.len();
    let (name, _) = player_infos.iter().nth(player_id).unwrap();

    name.clone()
}

fn get_player_display(
    player_infos: &HashMap<String, PlayerInfo>,
    game_id: u32,
    hint_nr: u32,
) -> Option<PlayerDisplay> {
    let player_id = game_id as usize % player_infos.len();
    let (_, info) = player_infos.iter().nth(player_id).unwrap();

    let lines = vec![
        Line {
            category: "Peak rating".to_string(),
            answer: info.peak_rating.to_string(),
        },
        Line {
            category: "Birth date".to_string(),
            answer: info.birth_date.clone(),
        },
        Line {
            category: "Year of GM title".to_string(),
            answer: info.year_of_gm.to_string(),
        },
        Line {
            category: "Citizenship".to_string(),
            answer: info.citizenship_country.clone(),
        },
        Line {
            category: "Chess.com username".to_string(),
            answer: info
                .chess_com_name
                .iter()
                .nth(0)
                .unwrap_or(&"Unknown".to_string())
                .clone(),
        },
    ];

    if hint_nr as usize >= lines.len() + 2 {
        return None;
    }

    let mut image = "".to_string();
    if hint_nr as usize >= lines.len() + 1 {
        image = info.images.iter().nth(0).unwrap_or(&"".to_string()).clone();
    }

    let last_hint = std::cmp::min(hint_nr as usize, lines.len());
    let display_lines = &lines[0..last_hint];

    Some(PlayerDisplay {
        lines: display_lines.to_vec(),
        image: image,
    })
}

async fn introduction(State(state): State<AppState<'_>>) -> Html<String> {
    let template = state.env.get_template("game").unwrap();
    let rendered = template.render(context!(start_screen => true));

    match rendered {
        Ok(result) => Html(result),
        Err(..) => Html("".into()),
    }
}

async fn start_game(
    State(state): State<AppState<'_>>,
    request: Query<GameRequest>,
) -> Html<String> {
    println!("Start game [game_id {}]", request.game_id);

    {
        let mut game_states = state.game_states.write().unwrap();
        game_states.insert(request.game_id, GameState { current_hint: 2 });
    }

    let player_display = get_player_display(&state.player_infos, request.game_id, 1).unwrap();

    let template = state.env.get_template("game").unwrap();
    let rendered = template.render(
        context!(lines => player_display.lines, start_screen => false, show_image => false),
    );

    match rendered {
        Ok(result) => Html(result),
        Err(..) => Html("".into()),
    }
}

async fn get_category(State(state): State<AppState<'_>>, request: Query<GameRequest>) -> Response {
    println!("Request [game_id {}]", request.game_id);

    let mut num_lines = 0;

    {
        let mut game_states = state.game_states.write().unwrap();
        match game_states.get_mut(&request.game_id) {
            Some(game_state) => {
                num_lines = game_state.current_hint;
                game_state.current_hint += 1;
            }
            None => {
                return Html("").into_response();
            }
        }
    }

    let player_display = get_player_display(&state.player_infos, request.game_id, num_lines);
    if player_display.is_none() {
        return StatusCode::IM_A_TEAPOT.into_response();
    }
    let player_display = player_display.unwrap();

    let template = state.env.get_template("playarea").unwrap();
    let rendered = template.render(
        context!(lines => player_display.lines, show_image => !player_display.image.is_empty(), img_src => player_display.image),
    );

    match rendered {
        Ok(result) => Html(result).into_response(),
        Err(..) => Html("").into_response(),
    }
}

async fn get_prediction(
    State(state): State<AppState<'_>>,
    request: Query<AnswerRequest>,
) -> Response {
    println!(
        "Get prediction [game_id {}]: {}",
        request.game_id, request.name
    );

    if request.name.len() < 3 {
        return StatusCode::IM_A_TEAPOT.into_response();
    }

    let requested_name = request.name.to_lowercase();

    for (name, _) in state.player_infos.iter() {
        let parts = name.split(" ");
        for part in parts {
            if part.to_lowercase().starts_with(requested_name.as_str()) {
                return Html(name.clone()).into_response();
            }
        }
    }

    StatusCode::IM_A_TEAPOT.into_response()
}

async fn submit_answer(
    State(state): State<AppState<'_>>,
    request: Query<AnswerRequest>,
) -> Response {
    println!("Answer [game_id {}]: {}", request.game_id, request.name);

    if (request.name.to_lowercase()
        == get_answer(&state.player_infos, request.game_id).to_lowercase())
    {
        let template = state.env.get_template("result").unwrap();
        let rendered = template.render(context!(success => true));

        return Html(rendered.unwrap()).into_response();
    }

    // dummy code for wrong answer
    StatusCode::IM_A_TEAPOT.into_response()
}

#[tokio::main]
async fn main() {
    let entries: HashMap<String, PlayerInfo> =
        serde_json::from_str(include_str!("../../resources/player-data.json"))
            .expect("JSON was not well-formatted");

    let mut state = AppState {
        env: Environment::new(),
        game_states: Arc::new(RwLock::new(HashMap::new())),
        player_infos: Arc::new(entries),
    };

    state
        .env
        .add_template("game", include_str!("../../html/game.html"))
        .expect("Could not load a template!");
    state
        .env
        .add_template("playarea", include_str!("../../html/playarea.html"))
        .expect("Could not load a template!");
    state
        .env
        .add_template("result", include_str!("../../html/result.html"))
        .expect("Could not load a template!");

    let app = Router::new()
        .route("/", get(introduction))
        .route("/start_game", get(start_game))
        .route("/category", get(get_category))
        .route("/answer", get(submit_answer))
        .route("/prediction", get(get_prediction))
        .nest_service("/assets", ServeDir::new("assets"))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}...", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
