use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

// import models and functions from other files
use crate::models::game::*;
use crate::persistence::game::*;

// endpoint for creating a new game listing
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

// endpoint for getting a single game via id
#[get("/v1/users/game/{game_id}")]
pub(crate) async fn get_single_game(
    data: web::Data<mysql::Pool>,
    path: web::Path<String>
) -> actix_web::Result<impl Responder> {
    let game_id_string: String = path.into_inner();
    let game_id = game_id_string.parse::<u64>().unwrap_or(0);
    let single_game = web::block(move || {
        get_single_game_persistence(&data, game_id)
    })
    .await??;
    Ok(web::Json(single_game))
}

//endpoint for getting a list of games for a user
#[get("/v1/users/games")]
pub(crate) async fn get_list_of_games(
    web::Json(games_find): web::Json<GameFind>,
    data: web::Data<mysql::Pool>,
) -> actix_web::Result<impl Responder> {
    let username = games_find.username;
    let games_list = web::block(move || get_list_of_games_persistence(&data, username)).await??;
    Ok(web::Json(games_list))
}

