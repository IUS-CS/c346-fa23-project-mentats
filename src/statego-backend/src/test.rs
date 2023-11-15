#[cfg(test)]
mod tests {
    use crate::routes;
    use crate::models::*;
    use actix_web::{test, web, App};
    use serde_json::json;


    #[actix_rt::test]
    async fn test_hello_endpoint() {
        let mut app = test::init_service(App::new().service(routes::hello)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, web::Bytes::from_static(b"Hello, cruel world"));
    }


    #[actix_rt::test]
    async fn user_creation_reading_updating_complete() {
        let user_data = SingleUserConvertedResponseData {
                email: String::from("SweetSouthernHeatBarbecue@gmail.com"),
                username: String::from("ASingleLaysPotatoChip"),
                first_name: Some(String::from("Frito")),
                last_name:  Some(String::from("Lay")),
                pronouns: None,
                bio: Some(String::from("I live in a bowl of other potato chips, but I'm special")),
                profile_pic: Some(String::from("https://www.eatthis.com/wp-content/uploads/sites/4/2018/09/bowl-potato-chips.jpg?quality=82&strip=1&w=800"))
        };

        let create_user_data = json!({ 
                "email": "SweetSouthernHeatBarbecue@gmail.com",
                "username": "ASingleLaysPotatoChip",
                "pass": "ThisIsGonnaBeHashedAnyway",
                "first_name": "Frito",
                "last_name": "Lay"     
        });

        let pool = crate::config::set_up_environment();
        let shared_data = web::Data::new(pool);
        
        let mut app = test::init_service(
            App::new()
            .app_data(shared_data.clone())
            .service(routes::create_user)
            .service(routes::update_user_profile)
            .service(routes::get_single_user)
            )
            .await;
        let mut req = test::TestRequest::post().uri("/v1/users").set_json(create_user_data).to_request();
        let mut resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());

        let update_user_data = json!({ 
            "username": "ASingleLaysPotatoChip",
            "bio": "I live in a bowl of other potato chips, but I'm special",
            "profile_pic": "https://www.eatthis.com/wp-content/uploads/sites/4/2018/09/bowl-potato-chips.jpg?quality=82&strip=1&w=800"     
    });
        //make request for updating profile and send it to test
        req = test::TestRequest::post().uri("/v1/users/profile").set_json(update_user_data).to_request();
        resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        //make request for pulling profile data and extracting it into a struct
        let finalreq = test::TestRequest::get().uri("/v1/user/AsingleLaysPotatoChip").to_request();
        resp = test::call_service(&mut app, finalreq).await;
        let json_response: SingleUserConvertedResponseData = test::read_body_json(resp).await;

        assert_eq!(json_response,user_data);

    }
}

