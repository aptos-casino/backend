use crate::GameVecMutex;
use std::str::FromStr;

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

    // Parse if error occur
    if event.is_object() {
        let events_error = event.as_object().unwrap().get("code");
        if events_error.is_some() {
            let code = events_error.unwrap().as_u64().unwrap();
            let message = event
                .as_object()
                .unwrap()
                .get("message")
                .unwrap()
                .as_str()
                .unwrap();
            println!(
                "Error during parsing events, code: {}, message: {}",
                code, message
            );
            return new_games;
        }
    }

    let events_vec = event.as_array().unwrap();
    for separate_event in events_vec {
        let data = separate_event.get("data").unwrap();

        // TODO: Field may be changed during development
        let id = u64::from_str(data.get("game_id").unwrap().as_str().unwrap()).unwrap();
        let seeds: u64 = rand::random::<u64>(); // Our generation if needed
        // let seeds = u64::from_str(data.get("seed").unwrap().as_str().unwrap()).unwrap();
        let hash = seed_hash::get_sha256(&seeds);
        let current_event_id = u64::from_str(separate_event.get("sequence_number").unwrap().as_str().unwrap()).unwrap();

        if *event_id > current_event_id{
            continue;
        }

        if games.lock().unwrap().iter().any(|val| val.id == id) {
            println!("Cannot add game, same ID already exist");
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
