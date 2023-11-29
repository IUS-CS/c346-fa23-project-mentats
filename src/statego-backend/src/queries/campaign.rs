use crate::models::campaign::*;
use mysql::{params, prelude::*};


pub fn select_campaignid_by_campaignstring(
    conn: &mut mysql::PooledConn,
    campaign_name: Option<String>,
) -> mysql::error::Result<u64> {
    conn.exec_first(
        "
        SELECT campaign_id
        FROM campaign
        WHERE campaign_name = :campaign_name AND is_deleted = 0
        ",
        params! {
            "campaign_name" => campaign_name
        },
    )
    .map(|campaign_id| campaign_id.unwrap())
}

pub fn select_campaignstring_by_campaignid(
    conn: &mut mysql::PooledConn,
    campaign_id: u64,
) -> mysql::error::Result<String> {
    conn.exec_first(
        "
        SELECT campaign_name
        FROM campaign
        WHERE campaign_id = :campaign_id AND is_deleted = 0
        ",
        params! {
            "campaign_id" => campaign_id
        },
    )
    .map(|campaign_id| campaign_id.unwrap())
}

pub fn create_campaign_in_database(
    conn: &mut mysql::PooledConn,
    user_id: u64,
    game_id: u64,
    campaign_name: String,
    descr: Option<String>,
    notes: Option<String>,
) -> mysql::error::Result<u64> {
    conn.exec_drop(
        "
        INSERT INTO campaign (user_id, game_id, campaign_name, descr, notes)
        VALUES (:user_id, :game_id, :campaign_name, :descr, :notes)
        ",
        params! {
            "user_id" => user_id,
            "game_id" => game_id,
            "campaign_name" => campaign_name,
            "descr" => descr,
            "notes" => notes

        },
    )
    .map(|_| conn.last_insert_id())
}



pub fn get_single_campaign_query(
    conn: &mut mysql::PooledConn,
    campaign_id: u64,
) -> mysql::error::Result<Vec<CampaignInfo>> {
    conn.exec_map(
        "
        SELECT campaign_id, campaign_name, descr, notes FROM campaign WHERE campaign_id = :campaign_id AND is_deleted = 0
        ",
        params! {
            "campaign_id" => campaign_id
        }
        ,

        |( campaign_id, campaign_name, descr, notes)| CampaignInfo {
            campaign_id: campaign_id,
            campaign_title: campaign_name,
            description: descr,
            notes: notes
        }
    )
}

pub fn get_list_of_campaigns_queries(
    conn: &mut mysql::PooledConn,
    user_id: u64,
    game_id: u64,
) -> mysql::error::Result<Vec<CampaignInfo>> {
    conn.exec_map(
        "
        SELECT campaign_id, campaign_name, descr, notes FROM campaign WHERE user_id = :user_id AND game_id = :game_id AND is_deleted = 0
        ",
        params! {
            "user_id" => user_id,
            "game_id" => game_id
        }
        ,

        |( campaign_id, campaign_name, descr, notes)| CampaignInfo {
            campaign_id: campaign_id,
            campaign_title: campaign_name,
            description: descr,
            notes: notes
        },
    )
}

pub fn delete_campaign_in_database(
    conn: &mut mysql::PooledConn,
    campaign_id: u64,
) -> mysql::error::Result<u64> {
        let campaign_update = conn.exec_drop(
            r"UPDATE campaign
            SET is_deleted = 1
            WHERE campaign_id = :campaign_id",
            params! {
                "campaign_id" => campaign_id
            }
        )
        .map(|_| conn.affected_rows());
        conn.exec_drop(
            r"UPDATE session
            SET is_deleted = 1
            WHERE campaign_id = :campaign_id",
            params! {
                "campaign_id" => campaign_id
            }
        )
        .map(|_| conn.affected_rows() + campaign_update.unwrap())
}