use crate::models::session::*;
use mysql::{params, prelude::*};


pub fn create_session_in_database(
    conn: &mut mysql::PooledConn,
    user_id: u64,
    game_id: u64,
    campaign_id: Option<u64>,
    session_start_string: String,
    session_end_string: String,
    players: String,
    number_of_players: i8,
    notes: Option<String>,
    winner: bool,
    winner_name: Option<String>,
    picture: Option<String>,
) -> mysql::error::Result<u64> {
    conn.exec_drop(
        "
        INSERT INTO session (user_id, game_id, campaign_id, 
        session_start, session_end, players, notes, winner, winner_name, picture, number_of_players)
        VALUES (:user_id, :game_id, :campaign_id, 
            :session_start, :session_end, :players, :notes, :winner, :winner_name, :picture, :number_of_players)
        ",
        params! {
            "user_id" => user_id,
            "game_id" => game_id,
            "campaign_id" => campaign_id,
            "session_start" => session_start_string,
            "session_end" => session_end_string,
            "players" => players,
            "notes" => notes,
            "winner" => winner,
            "winner_name" => winner_name,
            "picture" => picture,
            "number_of_players" => number_of_players
        },
    )
    .map(|_| conn.last_insert_id())
}

pub fn delete_session_in_database(
    conn: &mut mysql::PooledConn,
    session_id: u64
) -> mysql::error::Result<u64> {
        conn.exec_drop(
            r"UPDATE session 
            SET is_deleted = 1
            WHERE session_id = :session_id",
            params! {
                "session_id" => session_id,
            }
        )
        .map(|_| conn.affected_rows())
}

pub fn get_single_session_query(
    conn: &mut mysql::PooledConn,
    session_id: u64,
) -> mysql::error::Result<Vec<SessionDataUnConverted>> {
    conn.exec_map(
        "
        SELECT session_id, user_id, game_id, campaign_id, session_start, session_end, players, notes, winner, winner_name, picture, number_of_players
        FROM session
        WHERE session_id = :session_id AND is_deleted = 0
        ",
        params! {
            "session_id" => session_id
        }
        ,
        |(session_id, user_id, game_id, campaign_id, session_start, session_end, 
            players, notes, winner, winner_name, picture, 
            number_of_players)| SessionDataUnConverted {
            session_id : session_id,
            user_id : user_id,
            game_id : game_id,
            campaign_id : campaign_id,
            session_start: session_start,
            session_end: session_end,
            players: players,
            notes: notes,
            winner: winner,
            winner_name: winner_name,
            picture: picture,
            number_of_players: number_of_players
        }
    )
}

pub fn get_list_of_sessions_queries(
    conn: &mut mysql::PooledConn,
    user_id: u64,
    game_id: u64,
    campaign_id: Option<u64>,
) -> mysql::error::Result<Vec<SessionDataUnConverted>> {
    if campaign_id.is_none() {
        conn.exec_map(
            "
            SELECT session_id, user_id, game_id, campaign_id, session_start, session_end, players, notes, winner, winner_name, picture, number_of_players
            FROM session
            WHERE user_id = :user_id AND game_id = :game_id AND campaign_id IS NULL AND is_deleted = 0
            ",
            params! {
                "user_id" => user_id,
                "game_id" => game_id,
            "campaign_id" => campaign_id
            }
            ,

            |(session_id, user_id, game_id, campaign_id, session_start, session_end, 
                players, notes, winner, winner_name, picture, 
                number_of_players)| SessionDataUnConverted {
                session_id : session_id,
                user_id : user_id,
                game_id : game_id,
                campaign_id : campaign_id,
                session_start: session_start,
                session_end: session_end,
                players: players,
                notes: notes,
                winner: winner,
                winner_name: winner_name,
                picture: picture,
                number_of_players: number_of_players
            }
        )
    } else {
        conn.exec_map(
            "
            SELECT session_id, user_id, game_id, campaign_id, session_start, session_end, players, notes, winner, winner_name, picture, number_of_players
            FROM session
            WHERE user_id = :user_id AND game_id = :game_id AND campaign_id = :campaign_id AND is_deleted = 0
            ",
            params! {
                "user_id" => user_id,
                "game_id" => game_id,
            "campaign_id" => campaign_id
            }
            ,

            |(session_id, user_id, game_id, campaign_id, session_start, session_end, 
                players, notes, winner, winner_name, picture, 
                number_of_players)| SessionDataUnConverted {
                session_id : session_id,
                user_id : user_id,
                game_id : game_id,
                campaign_id : campaign_id,
                session_start: session_start,
                session_end: session_end,
                players: players,
                notes: notes,
                winner: winner,
                winner_name: winner_name,
                picture: picture,
                number_of_players: number_of_players
            }
        )
    }
}

pub fn get_list_of_sessions_from_campaign_queries(
    conn: &mut mysql::PooledConn,
    campaign_id: u64
) -> mysql::error::Result<Vec<SessionDataUnConverted>> {
        conn.exec_map(
            "
            SELECT session_id, user_id, game_id, campaign_id, session_start, session_end, players, notes, winner, winner_name, picture, number_of_players
            FROM session
            WHERE campaign_id = :campaign_id AND is_deleted = 0
            ",
            params! {
            "campaign_id" => campaign_id
            }
            ,

            |(session_id, user_id, game_id, campaign_id, session_start, session_end, 
                players, notes, winner, winner_name, picture, 
                number_of_players)| SessionDataUnConverted {
                session_id : session_id,
                user_id : user_id,
                game_id : game_id,
                campaign_id : campaign_id,
                session_start: session_start,
                session_end: session_end,
                players: players,
                notes: notes,
                winner: winner,
                winner_name: winner_name,
                picture: picture,
                number_of_players: number_of_players
            }
        )
}

pub fn get_list_of_sessions_from_game_queries(
    conn: &mut mysql::PooledConn,
    game_id: u64
) -> mysql::error::Result<Vec<SessionDataUnConverted>> {
        conn.exec_map(
            "
            SELECT session_id, user_id, game_id, campaign_id, session_start, session_end, players, notes, winner, winner_name, picture, number_of_players
            FROM session
            WHERE game_id = :game_id AND campaign_id IS NULL AND is_deleted = 0
            ",
            params! {
            "game_id" => game_id
            }
            ,

            |(session_id, user_id, game_id, campaign_id, session_start, session_end, 
                players, notes, winner, winner_name, picture, 
                number_of_players)| SessionDataUnConverted {
                session_id : session_id,
                user_id : user_id,
                game_id : game_id,
                campaign_id : campaign_id,
                session_start: session_start,
                session_end: session_end,
                players: players,
                notes: notes,
                winner: winner,
                winner_name: winner_name,
                picture: picture,
                number_of_players: number_of_players
            }
        )
}
