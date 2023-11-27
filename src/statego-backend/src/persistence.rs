/////////////////////////////////////////////
/// persistence.rs
///
/// handles error checking on requests
/// handles authentication of users
/////////////////////////////////////////////
use actix_web::http::StatusCode;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{prelude::*, NaiveDateTime};
use derive_more::{Display, Error, From};

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

pub fn get_single_user_persistence(
    pool: &mysql::Pool,
    username: String,
) -> Result<SingleUserConvertedResponseData, PersistenceError> {
    let mut conn = pool.get_conn()?;

    if username.replace(' ', "").trim().is_empty() {
        return Err(PersistenceError::EmptyUsername);
    }
    //get user_id
    let user_id = select_userid_by_userstring(&mut conn, username).unwrap();
    let single_user = select_single_user(&mut conn, user_id).unwrap();
    //let create_time_vec: Vec<&str> = single_user.create_time.split(',').collect();
    //let year = create_time_vec[0].parse::<i32>();
    //let month = create_time_vec[1].parse::<u32>();
    //let day = create_time_vec[2].parse::<u32>();
    //let hour = create_time_vec[3].parse::<u32>();
    //let minute = create_time_vec[4].parse::<u32>();
    //let second = create_time_vec[5].parse::<u32>();
    //let date = NaiveDate::from_ymd_opt(year.unwrap(), month.unwrap(), day.unwrap()).unwrap();
    //let time = NaiveTime::from_hms_opt(hour.unwrap(), minute.unwrap(), second.unwrap()).unwrap();
    //let create_time_datetime = NaiveDateTime::new(date, time);

    let single_user_converted = SingleUserConvertedResponseData {
        //create_time: create_time_datetime,
        email: single_user.email,
        username: single_user.username,
        first_name: single_user.first_name,
        last_name: single_user.last_name,
        pronouns: single_user.pronouns,
        bio: single_user.bio,
        profile_pic: single_user.profile_pic,
    };

    Ok(single_user_converted)
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
pub fn update_user(
    pool: &mysql::Pool,
    username: String,
    bio: String,
    profile_pic: String,
) -> Result<UserUpdateData, PersistenceError> {
    let mut conn = pool.get_conn()?;

    if username.replace(' ', "").trim().is_empty() {
        Err(PersistenceError::EmptyUsername)
    } else {
        Ok(update_bio_and_profilepic(
            &mut conn,
            username,
            bio,
            profile_pic,
        )?)
    }
}

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
    //pass in deletion
    let is_deleted: bool = true;
    
    let affected_rows = delete_session_in_database(
        &mut conn,
        session_id,
        is_deleted
    );

    if affected_rows.unwrap() > 0 {
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
    for SessionDataUnConverted in unconverted_session_vec {
        //create SessionDataConverted Object last

        //split session_start string into a vector of strings
        let session_start_datetime = string_to_NaiveDateTime(SessionDataUnConverted.session_start);
        let session_end_datetime = string_to_NaiveDateTime(SessionDataUnConverted.session_end);

        //turn players string into a vector of &str, and then into a vector of Strings for return
        let players_str_vec: Vec<&str> = SessionDataUnConverted.players.split(',').collect();
        let players_string_vec: Vec<String> =
            players_str_vec.into_iter().map(|s| s.to_string()).collect();

        let converted_session_data = SessionDataConverted {
            session_id: SessionDataUnConverted.session_id,
            user_name: username2.clone(),
            game_title: game_title2.clone(),
            campaign_title: campaign_title2.clone(),
            session_start: session_start_datetime,
            session_end: session_end_datetime,
            players: players_string_vec,
            notes: SessionDataUnConverted.notes,
            winner: SessionDataUnConverted.winner,
            winner_name: SessionDataUnConverted.winner_name,
            picture: SessionDataUnConverted.picture,
            number_of_players: SessionDataUnConverted.number_of_players
        };

        converted_session_vec.push(converted_session_data);
    }

    //see using the struct declared for models when dealing with errors
    Ok(converted_session_vec)
}

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

pub fn naive_date_time_to_string(
    date: NaiveDateTime,
) -> String {
    let year_str = date.date().year().to_string();
    let month_str = date.date().month().to_string();
    let day_str = date.date().day().to_string();
    let hour_str = date.time().hour().to_string();
    let minute_str = date.time().minute().to_string();
    let second_str = date.time().second().to_string();
    let mut date_vec: Vec<String> = Vec::new();
    date_vec.push(year_str);
    date_vec.push(month_str);
    date_vec.push(day_str);
    date_vec.push(hour_str);
    date_vec.push(minute_str);
    date_vec.push(second_str);
    let date_string = date_vec.join(",");
    return date_string;
}

pub fn string_to_NaiveDateTime(
     date: String
) -> NaiveDateTime {
    let string_vec: Vec<&str> = date.split(',').collect();
        //create NaiveDateTime object out of string
        let year = string_vec[0].parse::<i32>();
        let month = string_vec[1].parse::<u32>();
        let day = string_vec[2].parse::<u32>();
        let hour = string_vec[3].parse::<u32>();
        let minute = string_vec[4].parse::<u32>();
        let second = string_vec[5].parse::<u32>();
        let date = NaiveDate::from_ymd_opt(year.unwrap(), month.unwrap(), day.unwrap()).unwrap();
        let time =
            NaiveTime::from_hms_opt(hour.unwrap(), minute.unwrap(), second.unwrap()).unwrap();
        let session_start_datetime = NaiveDateTime::new(date, time);
        return session_start_datetime;
}