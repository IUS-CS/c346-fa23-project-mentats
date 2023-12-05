
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct NewGame {
    pub username: String,
    pub game_title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GameFind {
    pub username: String,
}

#[derive(Debug, Serialize, Clone, Deserialize)]
pub struct GameInfo {
    pub game_id: u64,
    pub game_title: String,
    pub description: Option<String>,
    //pub campaign_list: Vec<CampaignInfo>
}