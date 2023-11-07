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
pub struct SessionData {
    pub username: String,
    pub game_title: String,
    pub campaign_title: String,
    pub session_start: NaiveDateTime,
    pub session_end: NaiveDateTime,
    pub players: Vec<String>,
    pub notes: String,
    pub winner: bool,
    pub winner_name: String,
    pub session_picture_link: String,
}




