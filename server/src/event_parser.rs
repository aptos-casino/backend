use crate::GameVecMutex;
use serde_json::json;

mod seed_hash;

#[derive(Clone)]
pub struct Game {
    id: u64,
    seeds: u64,
    hash: String,
}

pub fn parse_aptos_event(
    event: &serde_json::Value,
    games: &mut GameVecMutex,
    event_id: &mut u64,
) -> Vec<Game> {
    // Duplicate of new games that may be added in this call, need this for contract calls instead of looking on full vector with all games
    let mut new_games: Vec<Game> = vec![];

    println!("Debug: {}", event);
    let a1 = event.is_object();
    let a2 = event.is_array();
    let a3 = event.is_number();
    let a4 = event.is_null();
    let a5 = event.is_string();

    let parsed_ev = String::from(event.as_str().unwrap());
    let parsed_ev = json!(parsed_ev);
    let a1 = parsed_ev.is_object();
    let a2 = parsed_ev.is_array();
    let a3 = parsed_ev.is_number();
    let a4 = parsed_ev.is_null();
    let a5 = parsed_ev.is_string();


    // Parse if error occur
    let arr = event.get("message");
    let events_error = event.as_object().unwrap().get("code");
    if events_error.is_some()
    {
        println!("Error during parsing events, code: {}", events_error.unwrap().as_str().unwrap());
    }

    let events_vec = event.as_array().unwrap();
    for separate_event in events_vec {
        let data = separate_event.get("data").unwrap();

        // TODO: Field may be changed during development
        let id = data.get("id").unwrap().as_u64().unwrap();
        let seeds: u64 = rand::random::<u64>();
        let hash = seed_hash::get_sha256(&seeds);
        let current_event_id = data.get("sequence_number").unwrap().as_u64().unwrap();

        if current_event_id < *event_id {
            continue;
        }

        if games.lock().unwrap().iter().any(|val| val.id == id) {
            break;
        }

        *event_id = current_event_id;
        let new_game = Game { id, seeds, hash };

        // Main vec of games
        games.lock().unwrap().push(new_game.clone());

        // Current, new games for calls
        new_games.push(new_game.clone());
    }

    new_games
}
