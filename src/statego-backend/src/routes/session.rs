use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

// import models and functions from other files
use crate::models::session::*;
use crate::persistence::session::*;

// endpoint for creating a new session
#[post("/v1/users/session")]
pub(crate) async fn create_session(
    web::Json(session_data): web::Json<Session>,
    data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
    // extract data from json

    let username = session_data.username;
    let game_title = session_data.game_title;
    let campaign_title = session_data.campaign_title;
    let session_start = session_data.session_start;
    let session_end = session_data.session_end;
    let players = session_data.players;
    let notes = session_data.notes;
    let winner = session_data.winner;
    let winner_name = session_data.winner_name;
    let picture = session_data.picture;
    // attempt to create session
    web::block(move || {
        create_session_persistence(
            &data,
            username,
            game_title,
            campaign_title,
            session_start,
            session_end,
            players,
            notes,
            winner,
            winner_name,
            picture,
        )
    })
    .await??;

    // return 204 status code on success
    Ok(HttpResponse::NoContent())
}

// endpoint for deleting a session
#[delete("/v1/users/session/{session_id}")]
pub(crate) async fn delete_session(
    data: web::Data<mysql::Pool>,
    path: web::Path<String>
) -> actix_web::Result<impl Responder> {
    // extract data from json

    let session_id_string: String = path.into_inner();
    let session_id = session_id_string.parse::<u64>().unwrap_or(0);
    
    // attempt to delete session
    web::block(move || {
        delete_session_persistence(
            &data,
            session_id
        )
    })
    .await??;

    // return 204 status code on success
    Ok(HttpResponse::NoContent())
}

// endpoint for getting a list of sessions from username,game title, and optionally campaign title
#[get("/v1/users/session")]
pub(crate) async fn get_list_of_sessions(
    web::Json(sessions_find): web::Json<SessionsFind>,
    data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
    // extract data from json
    let username = sessions_find.username;
    let game_title = sessions_find.game_title;
    let campaign_title = sessions_find.campaign_title;
    // attempt to create session
    let session_list = web::block(move || {
        get_list_of_sessions_persistence(&data, username, game_title, campaign_title)
    })
    .await??;

    // a list of sessions that match the game title and campaign title for a user
    Ok(web::Json(session_list))
}

// endpoint for getting a list of sessions from username,game title, and optionally campaign title
#[get("/v1/campaign/session/{campaign_id}")]
pub(crate) async fn get_list_of_sessions_from_campaign(
    data: web::Data<mysql::Pool>,
    path: web::Path<String>
) -> actix_web::Result<impl Responder> {
    let campaign_id_string: String = path.into_inner();
    let campaign_id = campaign_id_string.parse::<u64>().unwrap_or(0);
    let session_list = web::block(move || {
        get_list_of_sessions_for_campaign_persistence(&data, campaign_id)
    })
    .await??;

    // a list of sessions that match the game title and campaign title for a user
    Ok(web::Json(session_list))
}

#[get("/v1/game/session/{game_id}")]
pub(crate) async fn get_list_of_sessions_from_game(
    data: web::Data<mysql::Pool>,
    path: web::Path<String>
) -> actix_web::Result<impl Responder> {
    let game_id_string: String = path.into_inner();
    let game_id = game_id_string.parse::<u64>().unwrap_or(0);
    let session_list = web::block(move || {
        get_list_of_sessions_for_game_persistence(&data, game_id)
    })
    .await??;

    // a list of sessions that match the game title and campaign title for a user
    Ok(web::Json(session_list))
}


// endpoint for getting a single session
#[get("/v1/users/session/{session_id}")]
pub(crate) async fn get_single_session(
    data: web::Data<mysql::Pool>,
    path: web::Path<String>
) -> actix_web::Result<impl Responder> {
    let session_id_string: String = path.into_inner();
    let session_id = session_id_string.parse::<u64>().unwrap_or(0);
    // attempt to create session
    let single_session = web::block(move || {
        get_single_session_persistence(&data, session_id)
    })
    .await??;

    // a list of sessions that match the game title and campaign title for a user
    Ok(web::Json(single_session))
}
