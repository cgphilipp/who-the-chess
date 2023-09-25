use axum::{
    body::{self, Empty, Full},
    extract::{Path, Query, State},
    http::{header, HeaderValue, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use include_dir::{include_dir, Dir};
use minijinja::{context, Environment};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    net::{IpAddr, Ipv6Addr, SocketAddr},
    sync::Arc,
};

#[derive(Clone)]
struct AppState<'a> {
    env: Environment<'a>,
    player_infos: Arc<Vec<PlayerInfo>>,
}

#[derive(Debug, Serialize, Clone)]
struct Line {
    category: String,
    answer: String,
    animated: bool,
}

#[derive(Clone, Serialize)]
struct PlayerDisplay {
    name: String,
    lines: Vec<Line>,
    image: String,
}

#[derive(Serialize)]
struct GameResultDisplay {
    success: bool,
    time: String,
    player: PlayerDisplay,
}

#[derive(Deserialize)]
struct GameRequest {
    game_id: u32,
    hint_id: u32,
}

#[derive(Deserialize)]
struct AnswerRequest {
    game_id: u32,
    name: String,
}

#[derive(Deserialize)]
struct PlayerInfo {
    name: String,
    birth_date: String,
    birth_place: String,
    year_of_gm: i32,
    chess_com_name: HashSet<String>,
    // lichess_name: HashSet<String>, // dont have data often enough sadly :(
    peak_rating: u32,
    // sport_country: String, // very often the same as citizenship, lets leave it out
    citizenship_country: String,
    images: HashSet<String>,
}

fn get_answer(player_infos: &Vec<PlayerInfo>, game_id: u32) -> String {
    let player_id = game_id as usize % player_infos.len();
    let info = player_infos.get(player_id).unwrap();

    info.name.clone()
}

const MAX_HINT: u32 = 7;

fn get_player_display(
    player_infos: &Vec<PlayerInfo>,
    game_id: u32,
    hint_nr: u32,
) -> Option<PlayerDisplay> {
    let player_id = game_id as usize % player_infos.len();
    let info = player_infos.get(player_id).unwrap();

    let lines = vec![
        Line {
            category: "Peak rating".to_string(),
            answer: info.peak_rating.to_string(),
            animated: hint_nr == 1,
        },
        Line {
            category: "Birth date".to_string(),
            answer: info.birth_date.clone(),
            animated: hint_nr == 2,
        },
        Line {
            category: "Year of GM title".to_string(),
            answer: info.year_of_gm.to_string(),
            animated: hint_nr == 3,
        },
        Line {
            category: "Citizenship".to_string(),
            answer: info.citizenship_country.clone(),
            animated: hint_nr == 4,
        },
        Line {
            category: "Birth place".to_string(),
            answer: info.birth_place.to_string(),
            animated: hint_nr == 5,
        },
        Line {
            category: "Chess.com username".to_string(),
            answer: info
                .chess_com_name
                .iter()
                .nth(0)
                .unwrap_or(&"Unknown".to_string())
                .clone(),
            animated: hint_nr == 6,
        },
    ];

    if hint_nr as u32 > MAX_HINT {
        return None;
    }

    let image = info.images.iter().nth(0).unwrap_or(&"".to_string()).clone();

    let last_hint = std::cmp::min(hint_nr as usize, lines.len());
    let display_lines = &lines[0..last_hint];

    Some(PlayerDisplay {
        name: info.name.clone(),
        lines: display_lines.to_vec(),
        image: image,
    })
}

async fn introduction(State(state): State<AppState<'_>>) -> Html<String> {
    let template = state.env.get_template("introduction").unwrap();
    let rendered = template.render(context!());

    match rendered {
        Ok(result) => Html(result),
        Err(..) => Html("".into()),
    }
}

async fn start_game(
    State(state): State<AppState<'_>>,
    request: Query<GameRequest>,
) -> Html<String> {
    let player_display =
        get_player_display(&state.player_infos, request.game_id, request.hint_id).unwrap();

    let template = state.env.get_template("game").unwrap();
    let rendered = template.render(context!(lines => player_display.lines, show_image => false));

    match rendered {
        Ok(result) => Html(result),
        Err(..) => Html("".into()),
    }
}

fn resolve_game(state: AppState<'_>, request: Query<GameRequest>) -> Response {
    let player_display = get_player_display(&state.player_infos, request.game_id, MAX_HINT)
        .unwrap_or(PlayerDisplay {
            name: "".to_string(),
            lines: vec![],
            image: "".to_string(),
        });

    let result = GameResultDisplay {
        success: false,
        time: "".to_string(),
        player: player_display,
    };

    let template = state.env.get_template("result").unwrap();
    let rendered = template.render(context!(result => result));

    match rendered {
        Ok(result) => {
            let mut resp = Html(result).into_response();
            *resp.status_mut() = StatusCode::CREATED;
            return resp;
        }
        Err(..) => return Html("").into_response(),
    }
}

async fn get_category(State(state): State<AppState<'_>>, request: Query<GameRequest>) -> Response {
    let player_display = get_player_display(&state.player_infos, request.game_id, request.hint_id);

    if player_display.is_none() {
        return resolve_game(state, request);
    }

    let player_display = player_display.unwrap();

    let show_image = request.hint_id as u32 == MAX_HINT;

    let template = state.env.get_template("playarea").unwrap();
    let rendered = template.render(
        context!(lines => player_display.lines, show_image => show_image, img_src => player_display.image),
    );

    match rendered {
        Ok(result) => Html(result).into_response(),
        Err(..) => Html("").into_response(),
    }
}

async fn skip(State(state): State<AppState<'_>>, request: Query<GameRequest>) -> Response {
    resolve_game(state, request)
}

async fn get_prediction(
    State(state): State<AppState<'_>>,
    request: Query<AnswerRequest>,
) -> Html<String> {
    let template = state.env.get_template("prediction").unwrap();

    if request.name.len() < 3 {
        let html = template.render(context!(show_prediction => false));
        return Html(html.unwrap_or("".to_string()));
    }

    let requested_name = request.name.to_lowercase();

    for player in state.player_infos.iter() {
        // first check against full name
        if player
            .name
            .to_lowercase()
            .starts_with(requested_name.as_str())
        {
            let html = template
                .render(context!(show_prediction => true, prediction => player.name))
                .unwrap_or("".to_string());
            return Html(html);
        }

        // then check against parts of name only
        let parts = player.name.split(" ");
        for part in parts {
            if part.to_lowercase().starts_with(requested_name.as_str()) {
                let html = template
                    .render(context!(show_prediction => true, prediction => player.name))
                    .unwrap_or("".to_string());
                return Html(html);
            }
        }
    }

    let html = template.render(context!(show_prediction => false));
    return Html(html.unwrap_or("".to_string()));
}

async fn submit_answer(
    State(state): State<AppState<'_>>,
    request: Query<AnswerRequest>,
) -> Response {
    if request.name.to_lowercase()
        == get_answer(&state.player_infos, request.game_id).to_lowercase()
    {
        // TODO reimplement duration counting with database access
        // let duration = SystemTime::now()
        //     .duration_since(game_state.start_time)
        //     .unwrap_or(Duration::new(0, 0));
        // let micros_str = duration.as_micros().to_string();
        // let duration_str = if micros_str.len() >= 4 {
        //     micros_str[0..2].to_string() + "." + micros_str[2..4].as_ref()
        // } else {
        //     duration.as_secs().to_string()
        // };

        let player_display = get_player_display(&state.player_infos, request.game_id, MAX_HINT)
            .unwrap_or(PlayerDisplay {
                name: "".to_string(),
                lines: vec![],
                image: "".to_string(),
            });

        let result = GameResultDisplay {
            success: true,
            time: "".to_string(),
            player: player_display,
        };

        let template = state.env.get_template("result").unwrap();
        let rendered = template.render(context!(result => result));

        return Html(rendered.unwrap()).into_response();
    }

    // dummy code for wrong answer
    StatusCode::IM_A_TEAPOT.into_response()
}

static STATIC_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets");
async fn assets(Path(path): Path<String>) -> Response {
    let path = path.trim_start_matches('/');
    let mime_type = mime_guess::from_path(path).first_or_text_plain();

    match STATIC_DIR.get_file(path) {
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap(),
        Some(file) => Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(mime_type.as_ref()).unwrap(),
            )
            .body(body::boxed(Full::from(file.contents())))
            .unwrap(),
    }
}

macro_rules! add_template {
    ($env:expr,$name:expr,$path:expr) => {
        $env.add_template(
            $name,
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), $path)),
        )
        .expect("Could not load a template!");
    };
}

#[tokio::main]
async fn main() {
    let entries: Vec<PlayerInfo> = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/resources/player-data.json"
    )))
    .expect("JSON was not well-formatted");

    let mut env = Environment::new();
    add_template!(env, "base", "/html/base.html");
    add_template!(env, "logic", "/html/logic.js");

    add_template!(env, "game", "/html/game/game.html");
    add_template!(env, "playarea", "/html/game/playarea.html");
    add_template!(env, "prediction", "/html/game/prediction.html");

    add_template!(env, "introduction", "/html/introduction/introduction.html");
    add_template!(env, "result", "/html/result/result.html");

    let state = AppState {
        env,
        player_infos: Arc::new(entries),
    };

    let app = Router::new()
        .route("/", get(introduction))
        .route("/start_game", get(start_game))
        .route("/category", get(get_category))
        .route("/answer", get(submit_answer))
        .route("/skip", get(skip))
        .route("/prediction", get(get_prediction))
        .route("/assets/*path", get(assets))
        .with_state(state);

    let addr = &SocketAddr::new(IpAddr::from(Ipv6Addr::UNSPECIFIED), 8080);
    println!("Listening on {}...", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
