mod seed_hash;

pub struct Game {
    id: u64,
    seeds: u64,
    hash: String,
}

pub fn parse_aptos_event(event: &serde_json::Value, games: &mut Vec<Game>) {
    // TODO: Here we need to parse event (and think on that how to see difference in event's type)
    // and store it game ID and create seed to keep it

    let events_vec = event.as_array().unwrap();
    for separate_event in events_vec {
        let data = separate_event.get("data").unwrap();

        // TODO: Field may be changed during development
        let id = data.get("id").unwrap().as_u64().unwrap();
        let seeds: u64 = rand::random::<u64>();
        let hash = seed_hash::get_sha256(&seeds);

        games.push(Game { id, seeds, hash });
    }
}
