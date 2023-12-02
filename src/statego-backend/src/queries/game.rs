use crate::models::game::*;
use mysql::{params, prelude::*};


pub fn select_gameid_by_gamestring(
    conn: &mut mysql::PooledConn,
    game_name: String,
) -> mysql::error::Result<u64> {
    conn.exec_first(
        "
        SELECT game_id
        FROM games
        WHERE game_name = :game_name AND is_deleted = 0
        ",
        params! {
            "game_name" => game_name
        },
    )
    .map(|game_id| game_id.unwrap())
}

pub fn select_gamestring_by_gameid(
    conn: &mut mysql::PooledConn,
    game_id: u64,
) -> mysql::error::Result<String> { 
    conn.exec_first(
        "
        SELECT game_name
        FROM games
        WHERE game_id = :game_id AND is_deleted = 0
        ",
        params! {
            "game_id" => game_id
        },
    )
    .map(|game_name| game_name.unwrap())
}

pub fn create_game_in_database(
    conn: &mut mysql::PooledConn,
    user_id: u64,
    game_name: String,
    descr: Option<String>,
) -> mysql::error::Result<u64> {
    conn.exec_drop(
        "
        INSERT INTO games (user_id, game_name, descr)
        VALUES (:user_id, :game_name, :descr)
        ",
        params! {
            "user_id" => user_id,
            "game_name" => game_name,
            "descr" => descr,

        },
    )
    .map(|_| conn.last_insert_id())
}

pub fn get_single_game_query(
    conn: &mut mysql::PooledConn,
    game_id: u64,
) -> mysql::error::Result<Vec<GameInfo>> {
    conn.exec_map(
        "
        SELECT game_id, game_name, descr FROM games WHERE game_id = :game_id AND is_deleted = 0
        ",
        params! {
            "game_id" => game_id
        },
        |(game_id, game_name, descr)| GameInfo {
            game_id: game_id,
            game_title: game_name,
            description: descr,
        },
    )
}

pub fn get_list_of_games_queries(
    conn: &mut mysql::PooledConn,
    user_id: u64,
) -> mysql::error::Result<Vec<GameInfo>> {
    conn.exec_map(
        "
        SELECT game_id, game_name, descr FROM games WHERE user_id = :user_id AND is_deleted = 0
        ",
        params! {
            "user_id" => user_id
        },
        |(game_id, game_name, descr)| GameInfo {
            game_id: game_id,
            game_title: game_name,
            description: descr,
        },
    )
}