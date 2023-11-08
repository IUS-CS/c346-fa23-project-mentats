/////////////////////////////////////////////
/// queeries.rs
/// 
/// handles querying the mysql database
/////////////////////////////////////////////

use mysql::{params, prelude::*};

use crate::models::*;

pub fn insert_new_ueser(
    conn: &mut mysql::PooledConn,
    my_email: String,
    my_username: String,
    my_password: String,
    my_first_name: String,
    my_last_name: String,
) -> mysql::error::Result<u64> {
    conn.exec_drop(
        "
        INSERT INTO users (email, username, pass, first_name, last_name)
        VALUES (:email, :username,:pass, :first_name, :last_name)
        ",
        params! {
            "email" => my_email,
            "username" => my_username,
            "pass" => my_password,
            "first_name" => my_first_name,
            "last_name" => my_last_name,
        },
    )
    .map(|_| conn.last_insert_id())
}

//TODO select all fields from the user table
pub fn select_all_users(conn: &mut mysql::PooledConn) -> mysql::error::Result<Vec<UserData>> {
    conn.query_map(
        "
        SELECT id, email, username, first_name, last_name
        FROM users
        ",
        |(my_id, my_email, my_username, my_first_name, my_last_name)| UserData {
            id: my_id,
            email: my_email,
            username: my_username,
            first_name: my_first_name,
            last_name: my_last_name,
        },
    )
}

pub fn select_password_by_username(conn: &mut mysql::PooledConn, username: String) -> mysql::error::Result<String> {
    conn.exec_first(
        "
        SELECT pass
        FROM users
        WHERE username = :username
        ",
        params! {
            "username" => username
        },
    )
    .map(|pass| pass.unwrap())
}

pub fn select_user_by_id(
    conn: &mut mysql::PooledConn,
    username: String,
) -> mysql::error::Result<UserData> {
    conn.exec_first(
        "
        SELECT id, email, username, first_name, last_name
        FROM users
        WHERE username = :username
        ",
        params! {
            "username" => username
        },
    )
    .map(|user| user.unwrap())
}

pub fn update_bio_and_profilepic(
    conn: &mut mysql::PooledConn,
    username: String,
    bio: String,
    profile_pic: String 
) -> mysql::error::Result<UserUpdateData> {
    conn.exec_drop(
        r"UPDATE users 
        SET bio = :bio, profile_pic = :profile_pic 
        WHERE username = :username",
        params! {
            "bio" => bio,
            "profile_pic" => profile_pic,
            "username" => username.clone()
        }
    );
    conn.exec_first(
        "
        SELECT username, bio, profile_pic
        FROM users
        WHERE username = :username
        ",
        params! {
            "username" => username
        },
    )
    .map(|user_update| user_update.unwrap())
}

pub fn create_session_in_database(
    conn: &mut mysql::PooledConn,
    user_id: u64,
    game_id: u64,
    campaign_id: Option<u64>,
    session_start_string: String,
    session_end_string: String,
    player_string: String,
    number_of_players: i8,
    notes: Option<String>,
    winner: bool,
    winner_name: Option<String>,
    session_picture_link: Option<String>
) -> mysql::error::Result<u64> {
    conn.exec_drop(
        "
        INSERT INTO session (user_id, game_id, campaign_id, 
        session_start, session_end, players, notes, winner, winner_name, picture, number_of_players)
        VALUES (:user_id, :game_id, :campaign_id, 
            :session_start, :session_end, :player_string, :notes, :winner, :winner_name, :session_picture_link, :number_of_players)
        ",
        params! {
            "user_id" => user_id,
            "game_id" => game_id,
            "campaign_id" => campaign_id,
            "session_start" => session_start_string,
            "session_end" => session_end_string,
            "players" => player_string,
            "notes" => notes,
            "winner" => winner,
            "winner_name" => winner_name,
            "picture" => session_picture_link,
            "number_of_players" => number_of_players
        },
    )
    .map(|_| conn.last_insert_id())
}

pub fn select_gameid_by_gamestring(conn: &mut mysql::PooledConn, game_name: String) -> mysql::error::Result<u64> {
    conn.exec_first(
        "
        SELECT game_id
        FROM games
        WHERE game_name = :game_name
        ",
        params! {
            "game_name" => game_name
        },
    )
    .map(|game_id| game_id.unwrap())
}

pub fn select_campaignid_by_campaignstring(conn: &mut mysql::PooledConn, campaign_name: Option<String>) -> mysql::error::Result<u64> {
    conn.exec_first(
        "
        SELECT campaign_id
        FROM campaigns
        WHERE campaign_name = :campaign_name
        ",
        params! {
            "campaign_name" => campaign_name.unwrap()
        },
    )
    .map(|campaign_id| campaign_id.unwrap())
}

pub fn select_userid_by_userstring(conn: &mut mysql::PooledConn, user_name: String) -> mysql::error::Result<u64> {
    conn.exec_first(
        "
        SELECT user_id
        FROM games
        WHERE user_name = :user_name
        ",
        params! {
            "user_name" => user_name
        },
    )
    .map(|user_id| user_id.unwrap())
}


pub fn get_list_of_sessions_queries(
    conn: &mut mysql::PooledConn,
    user_id: u64,
    game_id: u64,
    campaign_id: Option<u64>
) -> mysql::error::Result<Vec<SessionDataUnConverted>> {
    conn.query_map(
        "
        SELECT session_start, session_end, players, notes, winner, winner_name, picture, number_of_players
        FROM sessions
        WHERE user_id = :user_id , game_id = :game_id , campaign_id = :campaign_id
        ",
        |(session_start, session_end, players, notes, winner, winner_name, picture, number_of_players)| SessionDataUnConverted {
            session_start: session_start,
            session_end: session_end,
            players: players,
            notes: notes,
            winner: winner,
            winner_name: winner_name,
            session_picture_link: picture,
            number_of_players: number_of_players
        },
    )
}

