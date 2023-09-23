use chrono::{DateTime, Datelike};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Deserialize)]
struct PlayerDumpEntry {
    player_label: String,
    wd_label: String,
    ps_label: String,
    wdpq_label: Option<String>,
    pq_label: Option<String>,
}

#[derive(Debug, Default, Serialize)]
struct PlayerInfo {
    name: String,
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

impl PlayerInfo {
    fn new() -> Self {
        Default::default()
    }
}

fn main() {
    let filename = "resources/2700-wikidata-dump.json";
    let json = std::fs::read_to_string(filename).unwrap();
    let entries: Vec<PlayerDumpEntry> =
        serde_json::from_str(json.as_str()).expect("JSON was not well-formatted");

    let mut players: HashMap<String, PlayerInfo> = HashMap::new();

    for entry in entries.iter() {
        if !players.contains_key(&entry.player_label) {
            players.insert(entry.player_label.clone(), PlayerInfo::new());

            if let Some(player) = players.get_mut(&entry.player_label) {
                player.name = entry.player_label.clone();
            }
        }

        if entry.wd_label == "Lichess username" {
            if let Some(player) = players.get_mut(&entry.player_label) {
                player.lichess_name.insert(entry.ps_label.clone());
            }
        }

        if entry.wd_label == "Chess.com member ID" {
            if let Some(player) = players.get_mut(&entry.player_label) {
                player.chess_com_name.insert(entry.ps_label.clone());
            }
        }

        if entry.wd_label == "title of chess person" && entry.ps_label == "Grandmaster" {
            if let Some(player) = players.get_mut(&entry.player_label) {
                let date_string = entry.pq_label.as_ref().unwrap().as_str();
                let date = DateTime::parse_from_rfc3339(date_string);
                player.year_of_gm = date.unwrap().year();
            }
        }

        if entry.wd_label == "Elo rating" {
            if let Some(player) = players.get_mut(&entry.player_label) {
                let cmp = entry.ps_label.parse().unwrap();
                player.peak_rating = std::cmp::max(player.peak_rating, cmp);
            }
        }

        if entry.wd_label == "place of birth" {
            let has_country = entry.wdpq_label.is_some();

            if let Some(player) = players.get_mut(&entry.player_label) {
                player.birth_place = entry.ps_label.clone();
                if has_country {
                    player.birth_place += ", ";
                    player.birth_place += entry.pq_label.as_ref().unwrap();
                }
            }
        }

        if entry.wd_label == "date of birth" {
            if let Some(player) = players.get_mut(&entry.player_label) {
                let date_string = &entry.ps_label;
                let date = DateTime::parse_from_rfc3339(date_string).unwrap();
                player.birth_date = date.day().to_string()
                    + "."
                    + date.month().to_string().as_str()
                    + "."
                    + date.year().to_string().as_str();
            }
        }

        if entry.wd_label == "country for sport" {
            if let Some(player) = players.get_mut(&entry.player_label) {
                player.sport_country = entry.ps_label.clone();
            }
        }

        if entry.wd_label == "country of citizenship" {
            if let Some(player) = players.get_mut(&entry.player_label) {
                player.citizenship_country = entry.ps_label.clone();
            }
        }

        if entry.wd_label == "image" {
            if let Some(player) = players.get_mut(&entry.player_label) {
                player.images.insert(entry.ps_label.clone());
            }
        }
    }

    for player_info in players.values() {
        println!("{:?}", player_info);
    }

    let player_array = Vec::from_iter(players.values());
    let json = serde_json::to_string(&player_array);
    match json {
        Ok(string) => {
            println!("Serializing JSON...");
            std::fs::write("resources/player-data.json", string).expect("Could not write File!");
        }
        Err(..) => {
            println!("Error while serializing JSON!");
        }
    }
}
