use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

// import models and functions from other files
use crate::models::user::*;
use crate::persistence::user::*;

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

// endpoint for getting a single user from request path
#[get("/v1/user/{user_name}")]
pub(crate) async fn get_single_user(
    data: web::Data<mysql::Pool>,
    path: web::Path<String>,
) -> actix_web::Result<impl Responder> {
    let username: String = path.into_inner();
    let user = web::block(move || get_single_user_persistence(&data, username)).await??;
    Ok(web::Json(user))
}

//endpoint for logging in a user
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

// endpoint for updating a user profile
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
    web::block(move || update_user(&data, username, bio, profile_pic)).await??;

    // return 204 status code on success
    Ok(HttpResponse::NoContent())
}
