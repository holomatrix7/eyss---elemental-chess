use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use hdk::prelude::*;
use holo_hash::{AgentPubKeyB64, EntryHashB64};

#[hdk_entry(id = "chess_game_result")]
#[derive(Clone)]
pub struct ChessGameResult {
    #[serde(with = "ts_milliseconds")]
    timestamp: DateTime<Utc>,
    white_player: AgentPubKeyB64,
    black_player: AgentPubKeyB64,
    winner: ChessResult,
    num_of_moves: usize,
}

/// Represent a color.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ChessResult {
    White,
    Black,
    Draw,
}

pub fn publish_result(result: ChessGameResult) -> ExternResult<()> {
    create_entry(result.clone())?;

    let result_hash = hash_entry(result.clone())?;

    for player in vec![result.white_player, result.black_player] {
        create_link(AgentPubKey::from(player).into(), result_hash.clone(), ())?;      
    }

    Ok(())
}

pub fn get_my_game_results() -> ExternResult<Vec<(EntryHashB64, ChessGameResult)>> {
    let agent_pub_key = agent_info()?.agent_initial_pubkey;

    let links = get_links(agent_pub_key.into(), None)?;

    let results = links
        .into_inner()
        .iter()
        .map(|link| {
            let result = get_game_result(link.target.clone())?;
            Ok((EntryHashB64::from(link.target.clone()), result))
        })
        .collect::<ExternResult<Vec<(EntryHashB64, ChessGameResult)>>>()?;

    Ok(results)
}

fn get_game_result(game_result_hash: EntryHash) -> ExternResult<ChessGameResult> {
    let element = get(game_result_hash, GetOptions::default())?
        .ok_or(WasmError::Guest("Could not get game result".into()))?;

    let game_result: ChessGameResult = element
        .entry()
        .to_app_option()?
        .ok_or(WasmError::Guest("Could not get game result".into()))?;

    Ok(game_result)
}
