/////////////////////////////////////////////
/// main.rs
///
/// handles loading environment variables
/// handles setting up database connection
/// handles staring the server
/////////////////////////////////////////////
use actix_web::{web, App, HttpServer};
use std::io;

// modules from other files in project
mod config;
mod models;
mod persistence;
mod queries;
mod routes;
mod test;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // create shared data for app to be passed to functions for database connections
    let pool = crate::config::set_up_environment();
    let shared_data = web::Data::new(pool);

    log::info!("starting HTTP server at http://localhost:8080");
    HttpServer::new(move || {
        // serve functions at defined endpoints and bind global data pool
        App::new()
            .app_data(shared_data.clone())
            .service(routes::user::create_user)
            .service(routes::user::get_users)
            .service(routes::user::hello)
            .service(routes::user::login)
            .service(routes::user::update_user_profile)
            .service(routes::user::get_single_user)
            .service(routes::session::create_session)
            .service(routes::session::get_list_of_sessions)
            .service(routes::session::delete_session)
            .service(routes::session::get_list_of_sessions_from_campaign)
            .service(routes::session::get_list_of_sessions_from_game)
            .service(routes::session::get_single_session)
            .service(routes::game::create_game)
            .service(routes::game::get_single_game)
            .service(routes::game::get_list_of_games)
            .service(routes::campaign::create_campaign)
            .service(routes::campaign::get_single_campaign)
            .service(routes::campaign::get_list_of_campaigns)
            .service(routes::campaign::delete_campaign)
            .service(routes::game::delete_game)
            
            
            
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
