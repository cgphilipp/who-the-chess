use std::collections::{HashSet, HashMap};
use serde::Deserialize;
use chrono::{DateTime, Datelike};

#[derive(Debug, Deserialize)]
struct PlayerDumpEntry {
    playerLabel: String,
    wdLabel: String,
    ps_Label: String,
    wdpqLabel: Option<String>,
    pq_Label: Option<String>,
}

#[derive(Debug, Default)]
struct PlayerInfo
{
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

impl PlayerInfo
{
    fn new() -> Self {
        Default::default()
    }
}

fn main() {
    let filename = "resources/2700-wikidata-dump.json";
    let json = std::fs::read_to_string(filename).unwrap();
    let entries: Vec<PlayerDumpEntry> = serde_json::from_str(json.as_str()).expect("JSON was not well-formatted");

    let mut players : HashMap<String, PlayerInfo> = HashMap::new();

    for entry in entries.iter()
    {
        if !players.contains_key(&entry.playerLabel)
        {
            players.insert(entry.playerLabel.clone(), PlayerInfo::new());
        }
        
        if entry.wdLabel == "Lichess username"
        {
            if let Some(player) = players.get_mut(&entry.playerLabel)
            {
                player.lichess_name.insert(entry.ps_Label.clone());
            }
        }

        if entry.wdLabel == "Chess.com member ID"
        {
            if let Some(player) = players.get_mut(&entry.playerLabel)
            {
                player.chess_com_name.insert(entry.ps_Label.clone());
            }
        }

        if entry.wdLabel == "title of chess person" && entry.ps_Label == "Grandmaster"
        {
            if let Some(player) = players.get_mut(&entry.playerLabel)
            {
                let date_string = entry.pq_Label.as_ref().unwrap().as_str();
                let date = DateTime::parse_from_rfc3339(date_string);
                player.year_of_gm = date.unwrap().year();
            }
        }

        if entry.wdLabel == "Elo rating"
        {
            if let Some(player) = players.get_mut(&entry.playerLabel)
            {
                let cmp =  entry.ps_Label.parse().unwrap();
                player.peak_rating = std::cmp::max(player.peak_rating, cmp);
            }
        }

        if entry.wdLabel == "place of birth"
        {
            let has_country =  entry.wdpqLabel.is_some();

            if let Some(player) = players.get_mut(&entry.playerLabel)
            {
                player.birth_place = entry.ps_Label.clone();
                if (has_country)
                {
                    player.birth_place +=  ", ";
                    player.birth_place += entry.pq_Label.as_ref().unwrap();
                }
            }
        }

        if entry.wdLabel == "date of birth"
        {
            if let Some(player) = players.get_mut(&entry.playerLabel)
            {
                let date_string = &entry.ps_Label;
                let date = DateTime::parse_from_rfc3339(date_string).unwrap();
                player.birth_date = date.day().to_string() + "." + date.month().to_string().as_str() + "." + date.year().to_string().as_str();
            }
        }

        if entry.wdLabel == "country for sport"
        {
            if let Some(player) = players.get_mut(&entry.playerLabel)
            {
                player.sport_country = entry.ps_Label.clone();
            }
        }

        if entry.wdLabel == "country of citizenship"
        {
            if let Some(player) = players.get_mut(&entry.playerLabel)
            {
                player.citizenship_country = entry.ps_Label.clone();
            }
        }

        if entry.wdLabel == "image"
        {
            if let Some(player) = players.get_mut(&entry.playerLabel)
            {
                player.images.insert(entry.ps_Label.clone());
            }
        }
    }

    for (name, player_info) in players.iter() {
        println!("{} => {:?}", name, player_info);
    }
}
