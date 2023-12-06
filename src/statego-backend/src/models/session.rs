/////////////////////////////////////////////
use chrono::NaiveDateTime;
/// models.rs
///
/// data models that map to the mysql queries
/////////////////////////////////////////////
use mysql::prelude::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
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
    pub picture: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionDataConverted {
    pub session_id : u64,
    pub user_name : String,
    pub game_title : String,
    pub campaign_title : Option<String>,
    #[serde(with = "my_date_format")]
    pub session_start: NaiveDateTime,
    #[serde(with = "my_date_format")]
    pub session_end: NaiveDateTime,
    pub players: Vec<String>,
    pub notes: Option<String>,
    pub winner: bool,
    pub winner_name: Option<String>,
    pub picture: Option<String>,
    pub number_of_players: i8,
}

#[derive(Debug, Serialize, FromRow, Deserialize)]
pub struct SessionDataUnConverted {
    pub session_id : u64,
    pub user_id : u64,
    pub game_id : u64,
    pub campaign_id : Option<u64>,
    pub session_start: String,
    pub session_end: String,
    pub players: String,
    pub notes: Option<String>,
    pub winner: bool,
    pub winner_name: Option<String>,
    pub picture: Option<String>,
    pub number_of_players: i8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionResponseVec {
    pub session_list: Vec<SessionDataConverted>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SessionsFind {
    pub username: String,
    pub game_title: String,
    pub campaign_title: Option<String>,
}



mod my_date_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(dt)
    }
}

