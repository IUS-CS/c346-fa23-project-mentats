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
            .service(routes::create_user)
            .service(routes::get_users)
            .service(routes::hello)
            .service(routes::login)
            .service(routes::update_user_profile)
            .service(routes::create_session)
            .service(routes::get_list_of_sessions)
            .service(routes::create_game)
            .service(routes::create_campaign)
            .service(routes::get_list_of_games)
            .service(routes::get_list_of_campaigns)
            .service(routes::get_single_user)
            .service(routes::delete_session)
            .service(routes::get_single_session)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}
