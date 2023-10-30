/////////////////////////////////////////////
/// models.rs
/// 
/// data models that map to the mysql queries
/////////////////////////////////////////////

use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, NaiveDate};

#[derive(Debug, Deserialize)]
pub struct UserDetails {
    pub email: String,
    pub username: String,
    pub pass: String,
    pub first_name: String,
    pub last_name: String,

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
    pub user_name: String,
    pub bio: Option<String>,
    pub profile_pic: Option<String>,
}


