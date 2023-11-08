/////////////////////////////////////////////
/// persistence.rs
/// 
/// handles error checking on requests
/// handles authentication of users
/////////////////////////////////////////////

use actix_web::http::StatusCode;
use bcrypt::{hash, verify, DEFAULT_COST};
use derive_more::{Display, Error, From};
use chrono::{NaiveDateTime, prelude::*};

use crate::models::*;
use crate::queries::*;

#[derive(Debug, Display, Error, From)]
pub enum PersistenceError {
    EmptyEmail,
    EmptyUsername,
    EmptyPassword,
    BcryptError(bcrypt::BcryptError),
    MysqlError(mysql::Error),
    UnknownUser,
    Unknown,
}

//matches a PersistenceError to a StatusCode
impl actix_web::ResponseError for PersistenceError {
    fn status_code(&self) -> StatusCode {
        match self {
            PersistenceError::EmptyEmail => StatusCode::BAD_REQUEST,
            PersistenceError::EmptyUsername => StatusCode::BAD_REQUEST,
            PersistenceError::UnknownUser => StatusCode::UNAUTHORIZED,
            PersistenceError::EmptyPassword => StatusCode::BAD_REQUEST,
            PersistenceError::BcryptError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            PersistenceError::MysqlError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            PersistenceError::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

pub fn create_user_verify(
    pool: &mysql::Pool,
    email: String,
    username: String,
    password: String,
    first_name: String,
    last_name: String,
) -> Result<(), PersistenceError> {
    if email.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyEmail);
    }

    if username.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyUsername);
    }

    if password.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyPassword);
    }

    let mut conn = pool.get_conn()?;
    let hashed_password = hash(password, DEFAULT_COST)?;

    let last_insert_id = insert_new_ueser(
        &mut conn,
        email,
        username,
        hashed_password,
        first_name,
        last_name,
    )?;

    if last_insert_id > 0 {
        Ok(())
    } else {
        Err(PersistenceError::Unknown)
    }
}

pub fn login_user_verify(
    pool: &mysql::Pool,
    username: String,
    password: String,
) -> Result<UserData, PersistenceError> {
    if username.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyUsername);
    }

    if password.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyPassword);
    }

    let mut conn = pool.get_conn()?;
    let hashed_password = select_password_by_username(&mut conn, username.clone())?;

    if verify(password, &hashed_password)? {
        Ok(select_user_by_id(&mut conn, username)?)
    } else {
        Err(PersistenceError::UnknownUser)
    }
}

pub fn get_users_verify(pool: &mysql::Pool) -> Result<UserResponseData, PersistenceError> {
    let mut conn = pool.get_conn()?;

    Ok(UserResponseData {
        user_data: select_all_users(&mut conn)?,
    })
}


//function that checks if user exists and calls the query to update
pub fn update_user(
    pool: &mysql::Pool,
    username: String,
    bio: String,
    profile_pic: String
) -> Result<UserUpdateData, PersistenceError> {
    let mut conn = pool.get_conn()?;

    if username.replace(' ', "").trim().is_empty() {
        Err(PersistenceError::EmptyUsername)
    } else {
        Ok(update_bio_and_profilepic(&mut conn, username, bio, profile_pic)?)
    }
}

//function that checks if user exists and calls the query to update
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
    session_picture_link: Option<String>
) -> Result<(), PersistenceError> {
    let mut conn = pool.get_conn()?;
    //get number of players
    let number_of_players = players.len() as i8;
    //turn players vector into a string
    let player_string = players.join(", "); // No separator
    //get user_id
    let user_id = select_userid_by_userstring(&mut conn, username).unwrap();
    //get game_id
    let game_id = select_gameid_by_gamestring(&mut conn, game_title).unwrap();
    //if campaign isn't empty, get campaign_id
    let mut campaign_id: Option<u64> = None;
    if !campaign_title.is_none(){
        campaign_id = Some(select_campaignid_by_campaignstring(&mut conn, campaign_title).unwrap());
    }
    //format for naivedatetime strings will "YYYY, MM, DD, HH, MM, SS"
    //I realize this is the dumbest way to do this ever but due to
    //funkiness in the mysql crate not implementing From conversion for NaiveDateTime,
    //this is unavoidable, and must be considered when implementing pulling from database
    //If it makes you feel better, NaiveDateTimes can be subtracted in a complicated way
    //so elapsed time of sessions will NOT have to be implemented from scratch
    let year_str = session_start.date().year().to_string();
    let month_str = session_start.date().month().to_string();
    let day_str = session_start.date().day().to_string();
    let hour_str = session_start.time().hour().to_string();
    let minute_str = session_start.time().minute().to_string();
    let second_str = session_start.time().second().to_string();
    let mut session_vec_start: Vec<String> = Vec::new();
    session_vec_start.push(year_str);
    session_vec_start.push(month_str);
    session_vec_start.push(day_str);
    session_vec_start.push(hour_str);
    session_vec_start.push(minute_str);
    session_vec_start.push(second_str);
    let session_start_string = session_vec_start.join(", ");

    let year_str = session_end.date().year().to_string();
    let month_str = session_end.date().month().to_string();
    let day_str = session_end.date().day().to_string();
    let hour_str = session_end.time().hour().to_string();
    let minute_str = session_end.time().minute().to_string();
    let second_str = session_end.time().second().to_string();
    let mut session_vec_end: Vec<String> = Vec::new();
    session_vec_end.push(year_str);
    session_vec_end.push(month_str);
    session_vec_end.push(day_str);
    session_vec_end.push(hour_str);
    session_vec_end.push(minute_str);
    session_vec_end.push(second_str);
    let session_end_string = session_vec_end.join(", ");

    let last_insert_id = create_session_in_database(&mut conn, user_id, game_id, campaign_id, session_start_string, session_end_string, player_string,
    number_of_players, notes, winner, winner_name, session_picture_link);
    if last_insert_id.unwrap() > 0 {
        Ok(())
    } else {
        Err(PersistenceError::Unknown)
    }

    
}

//function that checks if user exists and calls the query to update
pub fn get_list_of_sessions_persistence(
    pool: &mysql::Pool,
    username: String,
    game_title: String,
    campaign_title: Option<String>,
) -> Result<Vec<SessionDataConverted>, PersistenceError> {
    let mut conn = pool.get_conn()?;
    //get number of players
    //get user_id
    let user_id = select_userid_by_userstring(&mut conn, username).unwrap();
    //get game_id
    let game_id = select_gameid_by_gamestring(&mut conn, game_title).unwrap();
    //if campaign isn't empty, get campaign_id
    let mut campaign_id: Option<u64> = None;
    if !campaign_title.is_none(){
        campaign_id = Some(select_campaignid_by_campaignstring(&mut conn, campaign_title).unwrap());
    }
    
    let unconverted_session_vec = (get_list_of_sessions_queries(&mut conn, user_id, game_id, campaign_id)).unwrap();

    let mut converted_session_vec: Vec<SessionDataConverted> = Vec::new();
    for SessionDataUnConverted in unconverted_session_vec {
        //create SessionDataConverted Object last
        
        //split session_start string into a vector of strings
        let session_start_vec: Vec<&str> = SessionDataUnConverted.session_start.split(',').collect();
        //create NaiveDateTime object out of string
        let year = session_start_vec[0].parse::<i32>();
        let month = session_start_vec[1].parse::<u32>();
        let day = session_start_vec[2].parse::<u32>();
        let hour = session_start_vec[3].parse::<u32>();
        let minute = session_start_vec[4].parse::<u32>();
        let second = session_start_vec[5].parse::<u32>();
        let date = NaiveDate::from_ymd_opt(year.unwrap(), month.unwrap(), day.unwrap()).unwrap();
        let time = NaiveTime::from_hms_opt(hour.unwrap(), minute.unwrap(), second.unwrap()).unwrap();
        let session_start_datetime = NaiveDateTime::new(date, time);

        let session_end_vec: Vec<&str> = SessionDataUnConverted.session_end.split(',').collect();
        //create NaiveDateTime object out of string
        let year = session_end_vec[0].parse::<i32>();
        let month = session_end_vec[1].parse::<u32>();
        let day = session_end_vec[2].parse::<u32>();
        let hour = session_end_vec[3].parse::<u32>();
        let minute = session_end_vec[4].parse::<u32>();
        let second = session_end_vec[5].parse::<u32>();
        let date = NaiveDate::from_ymd_opt(year.unwrap(), month.unwrap(), day.unwrap()).unwrap();
        let time = NaiveTime::from_hms_opt(hour.unwrap(), minute.unwrap(), second.unwrap()).unwrap();
        let session_end_datetime = NaiveDateTime::new(date, time);

        //turn players string into a vector of &str, and then into a vector of Strings for return
        let players_str_vec: Vec<&str> = SessionDataUnConverted.players.split(',').collect();
        let players_string_vec: Vec<String> = players_str_vec.into_iter().map(|s| s.to_string()).collect();

        let converted_session_data = SessionDataConverted{
            session_start: session_start_datetime,
            session_end:    session_end_datetime,
            players: players_string_vec,
            notes: SessionDataUnConverted.notes,
            winner: SessionDataUnConverted.winner,
            winner_name: SessionDataUnConverted.winner_name,
            session_picture_link: SessionDataUnConverted.session_picture_link,
            number_of_players: SessionDataUnConverted.number_of_players
        };
        
        converted_session_vec.push(converted_session_data);
    }

    //see using the struct declared for models when dealing with errors
    Ok(converted_session_vec)
    
}