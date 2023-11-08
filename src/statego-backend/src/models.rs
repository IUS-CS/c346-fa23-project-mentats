/////////////////////////////////////////////
/// models.rs
/// 
/// data models that map to the mysql queries
/////////////////////////////////////////////

use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Deserialize)]
pub struct UserDetails {
    pub email: String,
    pub username: String,
    pub pass: String,
    pub first_name: String,
    pub last_name: String
}

#[derive(Debug, Serialize, FromRow)]
pub struct UserData {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponseData {
    pub user_data: Vec<UserData>,
}

#[derive(Debug, Deserialize)]
pub struct UserCredentials {
    pub username: String,
    pub pass: String,
}

#[derive(Debug, Deserialize)]
pub struct UserUpdate {
    pub username: String,
    pub bio: Option<String>,
    pub profile_pic: Option<String>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct UserUpdateData {
    pub username: String,
    pub bio: String,
    pub profile_pic: String,
}

#[derive(Debug, Deserialize)]
pub struct Session {
    pub username: String,
    pub game_title: String,
    pub campaign_title: Option<String>,
    pub session_start: NaiveDateTime,
    pub session_end: NaiveDateTime,
    pub players: Vec<String>,
    pub notes: Option<String>,
    pub winner: bool,
    pub winner_name: Option<String>,
    pub session_picture_link: Option<String>
}

#[derive(Debug, Serialize)]
pub struct SessionDataConverted {
    pub session_start: NaiveDateTime,
    pub session_end: NaiveDateTime,
    pub players: Vec<String>,
    pub notes: Option<String>,
    pub winner: bool,
    pub winner_name: Option<String>,
    pub session_picture_link: Option<String>,
    pub number_of_players: i8
}
pub struct SessionDataUnConverted {
    pub session_start: String,
    pub session_end: String,
    pub players: String,
    pub notes: Option<String>,
    pub winner: bool,
    pub winner_name: Option<String>,
    pub session_picture_link: Option<String>,
    pub number_of_players: i8
}

#[derive(Debug, Serialize)]
pub struct SessionResponseVec {
    pub session_list: Vec<SessionDataConverted>
}

#[derive(Debug, Deserialize)]
pub struct SessionsFind {
    pub username: String,
    pub game_title: String,
    pub campaign_title: Option<String>
}





