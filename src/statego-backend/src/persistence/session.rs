use chrono::{prelude::*, NaiveDateTime};

use crate::models::session::*;
use crate::queries::{campaign::*, game::*, session::*, user::*};
use crate::persistence::error::*;
use crate::persistence::tools::*;



pub fn create_session_persistence(
    pool: &mysql::Pool,
    username: String,
    game_title: String,
    campaign_title: Option<String>,
    session_start: NaiveDateTime,
    session_end: NaiveDateTime,
    players: Vec<String>,
    notes: Option<String>,
    winner: bool,
    winner_name: Option<String>,
    picture: Option<String>,
) -> Result<(), PersistenceError> {
    let mut conn = pool.get_conn()?;
    //get number of players
    let number_of_players = players.len() as i8;
    //turn players vector into a string
    let player_string = players.join(","); // Seperated by commas
                                           //get user_id
    let user_id = select_userid_by_userstring(&mut conn, username).unwrap();
    //get game_id
    let game_id = select_gameid_by_gamestring(&mut conn, game_title).unwrap();
    //if campaign isn't empty, get campaign_id
    let mut campaign_id: Option<u64> = None;
    if !campaign_title.is_none() {
        campaign_id = Some(select_campaignid_by_campaignstring(&mut conn, campaign_title).unwrap());
    }
    let session_start_string = naive_date_time_to_string(session_start);
    let session_end_string = naive_date_time_to_string(session_end);

    let last_insert_id = create_session_in_database(
        &mut conn,
        user_id,
        game_id,
        campaign_id,
        session_start_string,
        session_end_string,
        player_string,
        number_of_players,
        notes,
        winner,
        winner_name,
        picture,
    );

    if last_insert_id.unwrap() > 0 {
        Ok(())
    } else {
        Err(PersistenceError::Unknown)
    }
}

pub fn delete_session_persistence(
    pool: &mysql::Pool,
    session_id: u64
) -> Result<(), PersistenceError> {
    let mut conn = pool.get_conn()?;
    
    let affected_rows = delete_session_in_database(
        &mut conn,
        session_id
    );

    if affected_rows.unwrap() > 0 {
        Ok(())
    } else {
        Err(PersistenceError::Unknown)
    }
}

pub fn get_single_session_persistence(
    pool: &mysql::Pool,
    session_id: u64,
) -> Result<SessionDataConverted, PersistenceError> {
    let mut conn = pool.get_conn()?;
    let single_session_unconverted_vec = get_single_session_query(&mut conn, session_id).unwrap();
    let single_session_unconverted = single_session_unconverted_vec.first().unwrap();
    //get strings for user_id, game_id, and campaign_id (if some)
    let user_name = select_userstring_by_userid(&mut conn, single_session_unconverted.user_id).unwrap();
    let game_title = select_gamestring_by_gameid(&mut conn, single_session_unconverted.game_id).unwrap();
    let mut campaign_title: Option<String> = None;
    if single_session_unconverted.campaign_id.is_some(){
        campaign_title = Some(select_campaignstring_by_campaignid(&mut conn, single_session_unconverted.campaign_id.unwrap()).unwrap());
    }
    let converted_session_start = string_to_NaiveDateTime(single_session_unconverted.session_start.clone());
    let converted_session_end = string_to_NaiveDateTime(single_session_unconverted.session_end.clone());
    let players_str_vec: Vec<&str> = single_session_unconverted.players.split(',').collect();
        let players_string_vec: Vec<String> =
            players_str_vec.into_iter().map(|s| s.to_string()).collect();


    let single_session_converted = SessionDataConverted {
        //create_time: create_time_datetime,
        session_id: single_session_unconverted.session_id,
        user_name: user_name,
        game_title: game_title,
        campaign_title: campaign_title,
        session_start: converted_session_start,
        session_end: converted_session_end,
        players: players_string_vec,
        notes: single_session_unconverted.notes.clone(),
        winner: single_session_unconverted.winner,
        winner_name: single_session_unconverted.winner_name.clone(),
        picture: single_session_unconverted.picture.clone(),
        number_of_players: single_session_unconverted.number_of_players
        
    };

    Ok(single_session_converted)
}


//function that checks if user exists and calls the query to update
pub fn get_list_of_sessions_persistence(
    pool: &mysql::Pool,
    username: String,
    game_title: String,
    campaign_title: Option<String>,
) -> Result<Vec<SessionDataConverted>, PersistenceError> {

    let username2 = username.clone();
    let game_title2 = game_title.clone();
    let campaign_title2 = campaign_title.clone();

    let mut conn = pool.get_conn()?;
    //get user_id
    let user_id = select_userid_by_userstring(&mut conn, username).unwrap();
    //get game_id
    let game_id = select_gameid_by_gamestring(&mut conn, game_title).unwrap();
    //if campaign isn't empty, get campaign_id
    let mut campaign_id: Option<u64> = None;
    if !campaign_title.is_none() {
        campaign_id = Some(select_campaignid_by_campaignstring(&mut conn, campaign_title).unwrap());
    }

    
    let unconverted_session_vec =
        (get_list_of_sessions_queries(&mut conn, user_id, game_id, campaign_id)).unwrap();

    let mut converted_session_vec: Vec<SessionDataConverted> = Vec::new();
    for session_data_unconverted in unconverted_session_vec {
        //create SessionDataConverted Object last

        //split session_start string into a vector of strings
        let session_start_datetime = string_to_NaiveDateTime(session_data_unconverted.session_start);
        let session_end_datetime = string_to_NaiveDateTime(session_data_unconverted.session_end);

        //turn players string into a vector of &str, and then into a vector of Strings for return
        let players_str_vec: Vec<&str> = session_data_unconverted.players.split(',').collect();
        let players_string_vec: Vec<String> =
            players_str_vec.into_iter().map(|s| s.to_string()).collect();

        let converted_session_data = SessionDataConverted {
            session_id: session_data_unconverted.session_id,
            user_name: username2.clone(),
            game_title: game_title2.clone(),
            campaign_title: campaign_title2.clone(),
            session_start: session_start_datetime,
            session_end: session_end_datetime,
            players: players_string_vec,
            notes: session_data_unconverted.notes,
            winner: session_data_unconverted.winner,
            winner_name: session_data_unconverted.winner_name,
            picture: session_data_unconverted.picture,
            number_of_players: session_data_unconverted.number_of_players
        };

        converted_session_vec.push(converted_session_data);
    }

    //see using the struct declared for models when dealing with errors
    Ok(converted_session_vec)
}


pub fn get_list_of_sessions_for_game_persistence(
    pool: &mysql::Pool,
    game_id: u64
) -> Result<Vec<SessionDataConverted>, PersistenceError> {

    let mut conn = pool.get_conn()?;
    let game_name = select_gamestring_by_gameid(&mut conn, game_id).unwrap();

    
    
    let unconverted_session_vec =
        (get_list_of_sessions_from_game_queries(&mut conn, game_id)).unwrap();

    let mut converted_session_vec: Vec<SessionDataConverted> = Vec::new();
    for session_data_unconverted in unconverted_session_vec {
        //create SessionDataConverted Object last

        //split session_start string into a vector of strings
        let session_start_datetime = string_to_NaiveDateTime(session_data_unconverted.session_start);
        let session_end_datetime = string_to_NaiveDateTime(session_data_unconverted.session_end);
        let user_name2 = select_userstring_by_userid(&mut conn, session_data_unconverted.user_id).unwrap().clone();


        //turn players string into a vector of &str, and then into a vector of Strings for return
        let players_str_vec: Vec<&str> = session_data_unconverted.players.split(',').collect();
        let players_string_vec: Vec<String> =
            players_str_vec.into_iter().map(|s| s.to_string()).collect();

        let converted_session_data = SessionDataConverted {
            session_id: session_data_unconverted.session_id,
            user_name: user_name2,
            game_title: game_name.clone(),
            campaign_title: None,
            session_start: session_start_datetime,
            session_end: session_end_datetime,
            players: players_string_vec,
            notes: session_data_unconverted.notes,
            winner: session_data_unconverted.winner,
            winner_name: session_data_unconverted.winner_name,
            picture: session_data_unconverted.picture,
            number_of_players: session_data_unconverted.number_of_players
        };

        converted_session_vec.push(converted_session_data);
    }

    //see using the struct declared for models when dealing with errors
    Ok(converted_session_vec)
}




pub fn get_list_of_sessions_for_campaign_persistence(
    pool: &mysql::Pool,
    campaign_id: u64
) -> Result<Vec<SessionDataConverted>, PersistenceError> {

    let mut conn = pool.get_conn()?;
    
    let unconverted_session_vec =
        (get_list_of_sessions_from_campaign_queries(&mut conn, campaign_id)).unwrap();

    let mut converted_session_vec: Vec<SessionDataConverted> = Vec::new();
    for session_data_unconverted in unconverted_session_vec {
        //create SessionDataConverted Object last

        //split session_start string into a vector of strings
        let session_start_datetime = string_to_NaiveDateTime(session_data_unconverted.session_start);
        let session_end_datetime = string_to_NaiveDateTime(session_data_unconverted.session_end);

        let user_name2 = select_userstring_by_userid(&mut conn, session_data_unconverted.user_id).unwrap().clone();
        let game_name = select_gamestring_by_gameid(&mut conn, session_data_unconverted.game_id).unwrap().clone();
        let campaign_name = select_campaignstring_by_campaignid(&mut conn, session_data_unconverted.campaign_id.unwrap()).unwrap();


        //turn players string into a vector of &str, and then into a vector of Strings for return
        let players_str_vec: Vec<&str> = session_data_unconverted.players.split(',').collect();
        let players_string_vec: Vec<String> =
            players_str_vec.into_iter().map(|s| s.to_string()).collect();

        let converted_session_data = SessionDataConverted {
            session_id: session_data_unconverted.session_id,
            user_name: user_name2,
            game_title: game_name,
            campaign_title: Some(campaign_name),
            session_start: session_start_datetime,
            session_end: session_end_datetime,
            players: players_string_vec,
            notes: session_data_unconverted.notes,
            winner: session_data_unconverted.winner,
            winner_name: session_data_unconverted.winner_name,
            picture: session_data_unconverted.picture,
            number_of_players: session_data_unconverted.number_of_players
        };

        converted_session_vec.push(converted_session_data);
    }

    //see using the struct declared for models when dealing with errors
    Ok(converted_session_vec)
}

