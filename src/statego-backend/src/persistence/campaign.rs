/////////////////////////////////////////////
/// persistence.rs
///
/// handles error checking on requests
/// handles authentication of users
/////////////////////////////////////////////

use crate::models::campaign::*;
use crate::queries::{campaign::*, game::*, session::*, user::*};
use crate::persistence::error::*;
use crate::persistence::tools::*;


pub fn get_single_campaign_persistence(
    pool: &mysql::Pool,
    campaign_id: u64,
) -> Result<CampaignInfo, PersistenceError> {
    let mut conn = pool.get_conn()?;
    let single_campaign_vec = get_single_campaign_query(&mut conn, campaign_id).unwrap();
    let single_campaign = single_campaign_vec.first().unwrap().clone();
    Ok(single_campaign)
}

pub fn create_new_campaign_persistence(
    pool: &mysql::Pool,
    username: String,
    game_title: String,
    campaign_title: String,
    description: Option<String>,
    notes: Option<String>,
) -> Result<(), PersistenceError> {
    let mut conn = pool.get_conn()?;

    //get user_id
    let user_id = select_userid_by_userstring(&mut conn, username).unwrap();
    //get game_id
    let game_id = select_gameid_by_gamestring(&mut conn, game_title).unwrap();

    let last_insert_id = create_campaign_in_database(
        &mut conn,
        user_id,
        game_id,
        campaign_title,
        description,
        notes,
    );
    if last_insert_id.unwrap() > 0 {
        Ok(())
    } else {
        Err(PersistenceError::Unknown)
    }
}

pub fn get_list_of_campaigns_persistence(
    pool: &mysql::Pool,
    username: String,
    game_title: String,
) -> Result<Vec<CampaignInfo>, PersistenceError> {
    let mut conn = pool.get_conn()?;
    //get user_id
    let user_id = select_userid_by_userstring(&mut conn, username).unwrap();
    let game_id = select_gameid_by_gamestring(&mut conn, game_title).unwrap();
    let campaigns_vec = (get_list_of_campaigns_queries(&mut conn, user_id, game_id)).unwrap();
    //see using the struct declared for models when dealing with errors
    Ok(campaigns_vec)
}

pub fn delete_campaign_persistence(
    pool: &mysql::Pool,
    campaign_id: u64
) -> Result<(), PersistenceError> {
    let mut conn = pool.get_conn()?;
    
    let affected_rows = delete_campaign_in_database(
        &mut conn,
        campaign_id
    );

    if affected_rows.unwrap() > 0 {
        Ok(())
    } else {
        Err(PersistenceError::Unknown)
    }
}
