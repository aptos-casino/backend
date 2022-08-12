use crate::GameVecMutex;

mod seed_hash;

pub struct Game {
    id: u64,
    seeds: u64,
    hash: String,
}

pub fn parse_aptos_event(event: &serde_json::Value, games: &mut GameVecMutex, event_id: &mut u64) {
    // TODO: Here we need to parse event (and think on that how to see difference in event's type)
    // and store it game ID and create seed to keep it

    let events_vec = event.as_array().unwrap();
    for separate_event in events_vec {
        let data = separate_event.get("data").unwrap();

        // TODO: Field may be changed during development
        let id = data.get("id").unwrap().as_u64().unwrap();
        let seeds: u64 = rand::random::<u64>();
        let hash = seed_hash::get_sha256(&seeds);
        let current_event_id = data.get("sequence_number").unwrap().as_u64().unwrap();

        if current_event_id < *event_id
        {
            continue;
        }

        if *event_id > current_event_id
        {
            std::panic!("Panic here!")
        }

        if games.lock().unwrap().iter().any(|val| val.id == id)
        {
            return;
        }

        *event_id = current_event_id;
        games.lock().unwrap().push(Game { id, seeds, hash });
    }
}
