// This line is a conditional compilation attribute that ensures the contained module (in this case, `tests`) 
// is only compiled and run when you're executing `cargo test`.
#[cfg(test)]
mod tests {
    // Importing necessary modules and types.
    use actix_web::{web, App, HttpServer, test, http};
    use mysql::{Pool};
    use std::{env, io};
    use crate::models::*;
    use crate::routes::*;
    use crate::config::*;

    // This is a utility function to set up a connection to the test database.
    // This function establishes a connection pool to your MySQL database.
    fn setup_database() -> Pool {
        let pool = Pool::new("mysql://test_db_url").unwrap();
        // The above line connects to a MySQL database with the specified URL.
        // Ensure this points to your TEST database, not your production one.
        
        // Optionally, here you could add code to initialize your database, 
        // like clearing tables or inserting initial test data.

        // Return the connection pool.
        pool
    }

    // This line marks the following function as an asynchronous test function.
    #[test]
    async fn test_update_user_profile_integration() {
        // Calls the previously defined setup_database function to get a connection pool.
        let pool = set_up_environment();
        let shared_data = web::Data::new(pool);
        // This starts an instance of your Actix app inside a test server. 
        // The test server runs your app in the background, allowing you to send test requests to it.
        let srv = HttpServer::new(move || {
            // serve functions at defined endpoints and bind global data pool
            App::new()
                .app_data(shared_data.clone())
                .service(create_user)
                .service(get_users)
                .service(hello)
                .service(login)
                .service(update_user_profile)
        })
        .bind(("127.0.0.1", 8080))
        .run()
        .await
    

        // This constructs and sends a POST request to the `/v1/users/profile` route of your app.
        // The request sends a JSON body corresponding to a UserUpdate struct.
        let request = srv.post("/v1/users/profile")
            .set_json(&UserUpdate {
                username: "testuser".into(),
                bio: Some("Integration Test Bio".into()),
                profile_pic: None,
            })
            // The send() method actually sends the constructed request.
            .send()
            .await
            .expect("Failed to send request.");

        // Asserts (checks) that the response status code is 204 (No Content).
        assert_eq!(request.status(), http::StatusCode::NO_CONTENT);

        // The following section validates the changes made to the database.
        // Gets a database connection from the connection pool.
        let mut conn = pool.get_conn().unwrap();
        // Fetches the user with the given username from the database.
        let updated_user: Option<UserUpdate> = conn.query_first(format!("SELECT * FROM users WHERE username='{}'", "testuser")).unwrap();
        // Asserts that a user was found.
        assert!(updated_user.is_some());
        // Asserts that the bio of the found user matches the one sent in the test request.
        assert_eq!(updated_user.unwrap().bio, Some("Integration Test Bio".into()));
    }
}