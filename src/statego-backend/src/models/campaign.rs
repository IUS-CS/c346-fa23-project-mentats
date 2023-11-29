
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct NewCampaign {
    pub username: String,
    pub game_title: String,
    pub campaign_title: String,
    pub description: Option<String>,
    pub notes: Option<String>,
}


#[derive(Debug, Deserialize)]
pub struct CampaignFind {
    pub username: String,
    pub game_title: String,
}



#[derive(Debug, Serialize, Clone)]
pub struct CampaignInfo {
    pub campaign_id: u64,
    pub campaign_title: String,
    pub description: Option<String>,
    pub notes: Option<String>,
}
