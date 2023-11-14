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


#[derive(Debug, Serialize, FromRow)]
pub struct SingleUserUnconvertedResponseData {
    //pub create_time: String,
    pub email: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub pronouns: Option<String>,
    pub bio: Option<String>,
    pub profile_pic: Option<String>
}

#[derive(Debug, Serialize, PartialEq, Deserialize)]
pub struct SingleUserConvertedResponseData {
    //pub create_time: NaiveDateTime,
    pub email: String,
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub pronouns: Option<String>,
    pub bio: Option<String>,
    pub profile_pic: Option<String>
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
    #[serde(with = "my_date_format")]
    pub session_start: NaiveDateTime,
    #[serde(with = "my_date_format")]
    pub session_end: NaiveDateTime,
    pub players: Vec<String>,
    pub notes: Option<String>,
    pub winner: bool,
    pub winner_name: Option<String>,
    pub picture: Option<String>

}

#[derive(Debug, Serialize)]
pub struct SessionDataConverted {
    #[serde(with = "my_date_format")]
    pub session_start: NaiveDateTime,
    #[serde(with = "my_date_format")]
    pub session_end: NaiveDateTime,
    pub players: Vec<String>,
    pub notes: Option<String>,
    pub winner: bool,
    pub winner_name: Option<String>,
    pub picture: Option<String>,
    pub number_of_players: i8
}


#[derive(Debug, Serialize, FromRow)]
pub struct SessionDataUnConverted {
    pub session_start: String,
    pub session_end: String,
    pub players: String,
    pub notes: Option<String>,
    pub winner: bool,
    pub winner_name: Option<String>,
    pub picture: Option<String>,
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

#[derive(Debug, Deserialize)]
pub struct NewGame {
    pub username: String,
    pub game_title: String,
    pub description: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct NewCampaign {
    pub username: String,
    pub game_title: String,
    pub campaign_title: String,
    pub description: Option<String>,
    pub notes: Option<String>
}

#[derive(Debug, Deserialize)]
pub struct GameFind {
    pub username: String,
}

#[derive(Debug, Deserialize)]
pub struct CampaignFind {
    pub username: String,
    pub game_title: String
}

#[derive(Debug, Serialize)]
pub struct GameInfo {
    pub game_title: String,
    pub description: Option<String>,
    //pub campaign_list: Vec<CampaignInfo>
}

#[derive(Debug, Serialize)]
pub struct CampaignInfo {
    pub campaign_title: String,
    pub description: Option<String>,
    pub notes: Option<String>
}


mod my_date_format{
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Serializer, Deserializer};


    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
    pub fn serialize<S>(
        date: &NaiveDateTime,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(dt)
    }
}



