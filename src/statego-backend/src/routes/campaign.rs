use actix_web::{delete, get, post, web, HttpResponse, Responder};

// import models and functions from other files
use crate::models::campaign::*;
use crate::persistence::campaign::*;

// endpoint for creating a new campaign listing
#[post("/v1/users/campaign")]
pub(crate) async fn create_campaign(
    web::Json(new_campaign): web::Json<NewCampaign>,
    data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
    let username = new_campaign.username;
    let game_title = new_campaign.game_title;
    let campaign_title = new_campaign.campaign_title;
    let description = new_campaign.description;
    let notes = new_campaign.notes;
    web::block(move || {
        create_new_campaign_persistence(
            &data,
            username,
            game_title,
            campaign_title,
            description,
            notes,
        )
    })
    .await??;

    // return 204 status code on success
    Ok(HttpResponse::NoContent())
}

// endpoint for getting a campaign
#[get("/v1/users/campaign/{campaign_id}")]
pub(crate) async fn get_single_campaign(
    data: web::Data<mysql::Pool>,
    path: web::Path<String>
) -> actix_web::Result<impl Responder> {
    let campaign_id_string: String = path.into_inner();
    let campaign_id = campaign_id_string.parse::<u64>().unwrap_or(0);
    let single_campaign = web::block(move || {
        get_single_campaign_persistence(&data, campaign_id)
    })
    .await??;
    Ok(web::Json(single_campaign))
}

//endpoint for getting a list of campaigns for a user
#[get("/v1/users/campaigns")]
pub(crate) async fn get_list_of_campaigns(
    web::Json(campaign_find): web::Json<CampaignFind>,
    data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
    let username = campaign_find.username;
    let game_title = campaign_find.game_title;
    let campaigns_list =
        web::block(move || get_list_of_campaigns_persistence(&data, username, game_title))
            .await??;

    Ok(web::Json(campaigns_list))
}

// endpoint for deleting a campaign and all associated sessions
#[delete("/v1/users/campaign/{campaign_id}")]
pub(crate) async fn delete_campaign(
    data: web::Data<mysql::Pool>,
    path: web::Path<String>
) -> actix_web::Result<impl Responder> {
    let campaign_id_string: String = path.into_inner();
    let campaign_id = campaign_id_string.parse::<u64>().unwrap_or(0);
    web::block(move || {
        delete_campaign_persistence(
            &data,
            campaign_id
        )
    })
    .await??;

    // return 204 status code on success
    Ok(HttpResponse::NoContent())
}
