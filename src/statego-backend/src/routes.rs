/////////////////////////////////////////////
/// routes.rs
/// 
/// maps functions to http requests
/// handles converiting to and from json
/// handles sending responses to client
/////////////////////////////////////////////

use actix_web::{get, post, put, web, HttpResponse, Responder};

// import models and functions from other files
use crate::{
    models::*,
    persistence::*,
};

// an example endpoint that just returns a string
#[get("/")]
pub(crate) async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, cruel world")
}

// endpoint for creating a new user
#[post("/v1/users")]
pub(crate) async fn create_user(
    web::Json(user_data): web::Json<UserDetails>,
    data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
    // extract data from json
    let email = user_data.email;
    let username = user_data.username;
    let password = user_data.pass;
    let first_name = user_data.first_name;
    let last_name = user_data.last_name;

    // attempt to create user
    web::block(move || create_user_verify(&data, email, username, password, first_name, last_name))
        .await??;

    // return 204 status code on success
    Ok(HttpResponse::NoContent())
}

// endpoint for getting all users
#[get("/v1/users")]
pub(crate) async fn get_users(data: web::Data<mysql::Pool>) -> actix_web::Result<impl Responder> {
    let users = web::block(move || get_users_verify(&data)).await??;
    Ok(web::Json(users))
}

//endpoint for loging in a user
#[put("/v1/login")]
pub(crate) async fn login(
    data: web::Data<mysql::Pool>,
    web::Json(user_data): web::Json<UserCredentials>,
) -> actix_web::Result<impl Responder> {
    let username = user_data.username;
    let password = user_data.pass;

    let user = web::block(move || login_user_verify(&data, username, password)).await??;

    Ok(web::Json(user))
}

// endpoint for creating a new user
#[post("/v1/users/profile")]
pub(crate) async fn update_user_profile(
    web::Json(user_data): web::Json<UserUpdate>,
    data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
    // extract data from json
    let username = user_data.username;
    let bio = user_data.bio.unwrap_or(String::from(" "));
    let profile_pic = user_data.profile_pic.unwrap_or(String::from(" "));
    // attempt to update
    web::block(move || update_user(&data, username, bio, profile_pic))
        .await??;

    // return 204 status code on success
    Ok(HttpResponse::NoContent())
}


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
    let session_picture_link = session_data.session_picture_link;
    // attempt to create session
    web::block(move || create_session_persistence(&data, username, game_title, campaign_title, session_start, session_end, players,
        notes, winner, winner_name, session_picture_link))
        .await??;

    // return 204 status code on success
    Ok(HttpResponse::NoContent())
}


// endpoint for getting a new session
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
    let session_list = web::block(move || get_list_of_sessions_persistence(&data, username, game_title, campaign_title)).await??;

    // a list of sessions that match the game title and campaign title for a user
    Ok(web::Json(session_list))
}

// endpoint for creating a new game
#[post("/v1/users/game")]
pub(crate) async fn create_game(
    web::Json(new_game): web::Json<NewGame>,
    data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
    // extract data from json
    
    let username = new_game.username;
    let game_title = new_game.game_title;
    let description = new_game.description;

    // attempt to create session
    web::block(move || create_new_game_persistence(&data, username, game_title, description))
        .await??;

    // return 204 status code on success
    Ok(HttpResponse::NoContent())
}

// endpoint for creating a new campaign
#[post("/v1/users/campaign")]
pub(crate) async fn create_campaign(
    web::Json(new_campaign): web::Json<NewCampaign>,
    data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
    // extract data from json
    
    let username = new_campaign.username;
    let game_title = new_campaign.game_title;
    let campaign_title = new_campaign.campaign_title;
    let description = new_campaign.description;
    let notes = new_campaign.notes;

    // attempt to create session
    web::block(move || create_new_campaign_persistence(&data, username, game_title, campaign_title, description, notes))
        .await??;

    // return 204 status code on success
    Ok(HttpResponse::NoContent())
}

// endpoint for getting a list of games for a user
//#[get("/v1/users/games")]
//pub(crate) async fn get_list_of_games(
//    web::Json(games_find): web::Json<GameFind>,
//    data: web::Data<mysql::Pool>,
//) -> actix_web::Result<impl Responder> {
    // extract data from json
//    let username = games_find.username;
    // attempt to create session
//    let games_list = web::block(move || get_list_of_games_persistence(&data, username)).await??;
//
    // a list of sessions that match the game title and campaign title for a user
//   Ok(web::Json(games_list))
//}
