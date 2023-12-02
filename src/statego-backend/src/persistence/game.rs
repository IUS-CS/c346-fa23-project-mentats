
use crate::models::game::*;
use crate::queries::{campaign::*, game::*, session::*, user::*};
use crate::persistence::error::*;


pub fn create_new_game_persistence(
    pool: &mysql::Pool,
    username: String,
    game_title: String,
    description: Option<String>,
) -> Result<(), PersistenceError> {
    let mut conn = pool.get_conn()?;

    //get user_id
    let user_id = select_userid_by_userstring(&mut conn, username).unwrap();

    let last_insert_id = create_game_in_database(&mut conn, user_id, game_title, description);
    if last_insert_id.unwrap() > 0 {
        Ok(())
    } else {
        Err(PersistenceError::Unknown)
    }
}



pub fn get_single_game_persistence(
    pool: &mysql::Pool,
    game_id: u64,
) -> Result<GameInfo, PersistenceError> {
    let mut conn = pool.get_conn()?;
    let single_game_vec = get_single_game_query(&mut conn, game_id).unwrap();
    let single_game = single_game_vec.first().unwrap().clone();
    Ok(single_game)
}

pub fn get_list_of_games_persistence(
    pool: &mysql::Pool,
    username: String,
) -> Result<Vec<GameInfo>, PersistenceError> {
    let mut conn = pool.get_conn()?;
    //get user_id
    let user_id = select_userid_by_userstring(&mut conn, username).unwrap();
    let games_vec = (get_list_of_games_queries(&mut conn, user_id)).unwrap();
    //see using the struct declared for models when dealing with errors
    Ok(games_vec)
}