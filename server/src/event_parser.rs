use crate::{utils, GameVecMutex};
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Game {
    pub id: u64,
    pub seeds: u64,
    pub hash: String,
}

pub fn parse_aptos_event(
    event: &mut serde_json::Value,
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

    let events_vec = event.as_array_mut().unwrap();
    events_vec.sort_by(|a, b| {
        let a_int = u64::from_str(a.get("sequence_number").unwrap().as_str().unwrap()).unwrap();

        let b_int = u64::from_str(b.get("sequence_number").unwrap().as_str().unwrap()).unwrap();

        return a_int.cmp(&b_int);
    });

    for separate_event in events_vec {
        let data = separate_event.get("data").unwrap();

        // TODO: Field may be changed during development
        let id = u64::from_str(data.get("game_id").unwrap().as_str().unwrap()).unwrap();
        let seeds: u64 = rand::random::<u64>(); // Our generation if needed
                                                // let seeds = u64::from_str(data.get("seed").unwrap().as_str().unwrap()).unwrap();
        let hash = utils::get_sha256(seeds);
        let current_event_id = u64::from_str(
            separate_event
                .get("sequence_number")
                .unwrap()
                .as_str()
                .unwrap(),
        )
        .unwrap();

        if games.lock().unwrap().iter().any(|val| val.id == id) || *event_id > current_event_id {
            continue;
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

mod test {
    use crate::event_parser::parse_aptos_event;
    use crate::GameVecMutex;
    use std::sync::{Arc, Mutex};

    #[test]
    fn parse_aptos_event_test() {
        let mut latest_event_id: u64 = 0;
        let json_str = r#"[{"data":{"game_id":"88","seed":"0x58"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"17","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"10350151"},{"data":{"game_id":"88","seed":"0x58"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"18","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"10353079"},{"data":{"game_id":"88","seed":"0x58"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"19","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"10357545"},{"data":{"game_id":"33","seed":"0x21"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"20","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"14997851"},{"data":{"game_id":"33","seed":"0x21"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"21","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15000083"},{"data":{"game_id":"33","seed":"0x21"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"22","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15004955"},{"data":{"game_id":"88","seed":"0x58"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"23","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15033977"},{"data":{"game_id":"88","seed":"0x58"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"24","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15036866"},{"data":{"game_id":"88","seed":"0x58"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"25","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15039980"},{"data":{"game_id":"88","seed":"0x58"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"26","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15055896"},{"data":{"game_id":"88","seed":"0x58"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"27","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15062616"},{"data":{"game_id":"88","seed":"0x58"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"28","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15064452"},{"data":{"game_id":"33","seed":"0x21"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"29","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15108917"},{"data":{"game_id":"53","seed":"0x35"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"30","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15151648"},{"data":{"game_id":"53","seed":"0x35"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"31","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15236823"},{"data":{"game_id":"42","seed":"0x793fb1484cfa8a7686a13bc22197cb142d2a74e8f73fc186dc534a80643bede4"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"32","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15237047"},{"data":{"game_id":"88","seed":"0x3d0e995d947232acbce2a69c9c49462bed0f3edd8636da438106fa0cb4fa659a"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"33","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15237223"},{"data":{"game_id":"53","seed":"0x35"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"34","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15243031"},{"data":{"game_id":"88","seed":"0xcf255b2adbe5358e02e595c1477e7a6f3700db98f50facd24365604d8fe4b35c"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"35","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15243236"},{"data":{"game_id":"53","seed":"0x35"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"36","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15245265"},{"data":{"game_id":"88","seed":"0x22368a4fc50fc87305624883e87f6a5907e34f9b406651bea3f9a8b1e361f659"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"37","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15245464"},{"data":{"game_id":"53","seed":"0x35"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"38","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15246776"},{"data":{"game_id":"88","seed":"0x4aa1c77fe8b7b3e979ffb2625501a95d747cd36b6d67477d2208204f76ad01ab"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"39","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15247009"},{"data":{"game_id":"88","seed":"0x58"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"40","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15477433"},{"data":{"game_id":"88","seed":"0x58"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"41","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15479439"}]"#;
        let mut json_obj: serde_json::Value = serde_json::from_str(json_str).unwrap();
        let mut games: GameVecMutex = Arc::new(Mutex::new(vec![]));

        let res = parse_aptos_event(&mut json_obj, &mut games, &mut latest_event_id);

        assert_eq!(res.len(), 4);
        assert_eq!(games.lock().unwrap().len(), 4);
        assert_eq!(latest_event_id, 32);

        let mut json_str = r#"[{"data":{"game_id":"88","seed":"0x58"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"17","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"10350151"},{"data":{"game_id":"88","seed":"0x58"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"18","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"10353079"},{"data":{"game_id":"88","seed":"0x58"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"19","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"10357545"},{"data":{"game_id":"33","seed":"0x21"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"20","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"14997851"},{"data":{"game_id":"33","seed":"0x21"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"21","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15000083"},{"data":{"game_id":"33","seed":"0x21"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"22","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15004955"},{"data":{"game_id":"88","seed":"0x58"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"23","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15033977"},{"data":{"game_id":"88","seed":"0x58"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"24","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15036866"},{"data":{"game_id":"88","seed":"0x58"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"25","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15039980"},{"data":{"game_id":"88","seed":"0x58"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"26","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15055896"},{"data":{"game_id":"88","seed":"0x58"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"27","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15062616"},{"data":{"game_id":"88","seed":"0x58"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"28","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15064452"},{"data":{"game_id":"33","seed":"0x21"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"29","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15108917"},{"data":{"game_id":"53","seed":"0x35"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"30","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15151648"},{"data":{"game_id":"53","seed":"0x35"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"31","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15236823"},{"data":{"game_id":"42","seed":"0x793fb1484cfa8a7686a13bc22197cb142d2a74e8f73fc186dc534a80643bede4"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"32","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15237047"},{"data":{"game_id":"88","seed":"0x3d0e995d947232acbce2a69c9c49462bed0f3edd8636da438106fa0cb4fa659a"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"33","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15237223"},{"data":{"game_id":"53","seed":"0x35"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"34","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15243031"},{"data":{"game_id":"88","seed":"0xcf255b2adbe5358e02e595c1477e7a6f3700db98f50facd24365604d8fe4b35c"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"35","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15243236"},{"data":{"game_id":"53","seed":"0x35"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"36","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15245265"},{"data":{"game_id":"88","seed":"0x22368a4fc50fc87305624883e87f6a5907e34f9b406651bea3f9a8b1e361f659"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"37","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15245464"},{"data":{"game_id":"53","seed":"0x35"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"38","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15246776"},{"data":{"game_id":"88","seed":"0x4aa1c77fe8b7b3e979ffb2625501a95d747cd36b6d67477d2208204f76ad01ab"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"39","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15247009"},{"data":{"game_id":"88","seed":"0x58"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"40","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15477433"},{"data":{"game_id":"88","seed":"0x58"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"41","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15479439"}, {"data":{"game_id":"1992","seed":"0x58"},"key":"0x0600000000000000ce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007","sequence_number":"42","type":"0xce8bd5e351eb3639e2a41b323d707b0ecc486479e4963fbbe4ff7f8ed0469007::Casino::InitedBackendSeedEvent","version":"15479439"}]"#;
        let mut json_obj: serde_json::Value = serde_json::from_str(json_str).unwrap();

        let res = parse_aptos_event(&mut json_obj, &mut games, &mut latest_event_id);
        assert_eq!(res.len(), 1);
        assert_eq!(games.lock().unwrap().len(), 5);
        assert_eq!(latest_event_id, 42);
    }
}
