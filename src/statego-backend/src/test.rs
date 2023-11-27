#[cfg(test)]
mod tests {
    use crate::models::*;
    use crate::routes;
    use actix_web::{test, web, App};
    use serde_json::json;
    use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

    #[actix_rt::test]
    async fn a_test_hello_endpoint() {
        let mut app = test::init_service(App::new().service(routes::hello)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, web::Bytes::from_static(b"Hello, cruel world"));
    }

    #[actix_rt::test]
    async fn b_user_creation_reading_updating_complete() {
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

        let pool = crate::config::set_up_test_environment();
        let shared_data = web::Data::new(pool);

        let mut app = test::init_service(
            App::new()
                .app_data(shared_data.clone())
                .service(routes::create_user)
                .service(routes::update_user_profile)
                .service(routes::get_single_user),
        )
        .await;
        let mut req = test::TestRequest::post()
            .uri("/v1/users")
            .set_json(create_user_data)
            .to_request();
        let mut resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());

        let update_user_data = json!({
                "username": "ASingleLaysPotatoChip",
                "bio": "I live in a bowl of other potato chips, but I'm special",
                "profile_pic": "https://www.eatthis.com/wp-content/uploads/sites/4/2018/09/bowl-potato-chips.jpg?quality=82&strip=1&w=800"
        });
        //make request for updating profile and send it to test
        req = test::TestRequest::post()
            .uri("/v1/users/profile")
            .set_json(update_user_data)
            .to_request();
        resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        //make request for pulling profile data and extracting it into a struct
        let finalreq = test::TestRequest::get()
            .uri("/v1/user/AsingleLaysPotatoChip")
            .to_request();
        resp = test::call_service(&mut app, finalreq).await;
        let json_response: SingleUserConvertedResponseData = test::read_body_json(resp).await;

        assert_eq!(json_response, user_data);
    
    
        let new_game = NewGame {
                username: String::from("ASingleLaysPotatoChip"),
                game_title: String::from("Hello Kitty Island Adventure"),
                description: Some(String::from("Animal Crossing but good"))
        };


        let new_game_data = json!({
                "username": "ASingleLaysPotatoChip",
                "game_title": "Hello Kitty Island Adventure",
                "description": "Animal Crossing but good",
        });

        let pool = crate::config::set_up_test_environment();
        let shared_data = web::Data::new(pool);

        let mut app = test::init_service(
            App::new()
                .app_data(shared_data.clone())
                .service(routes::create_game)
                .service(routes::create_campaign)
                .service(routes::create_session)
        )
        .await;

        //make request for posting game and sending it to test
        let mut req = test::TestRequest::post()
            .uri("/v1/users/game")
            .set_json(new_game_data)
            .to_request();
        let mut resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());

        let new_campaign = NewCampaign {
            username: String::from("ASingleLaysPotatoChip"),
            game_title: String::from("Hello Kitty Island Adventure"),
            campaign_title: String::from("Descent into Avernus"),
            description: Some(String::from("A quest to rescue Elturel from the depths")),
            notes: Some(String::from("Questioning the reason why the developers implemented death animations in Hello Kitty"))
        };

        let new_campaign_data = json!({
                "username": "ASingleLaysPotatoChip",
                "game_title": "Hello Kitty Island Adventure",
                "campaign_title": "Descent into Avernus",
                "description": "A quest to rescue Elturel from the depths",
                "notes": "Questioning the reason why the developers implemented death animations in Hello Kitty"
        });

        //make request for posting campaign and send it to test
        req = test::TestRequest::post()
            .uri("/v1/users/campaign")
            .set_json(new_campaign_data)
            .to_request();
        resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let start_date = NaiveDate::from_ymd_opt(2040, 1, 1);
        let start_time = NaiveTime::from_hms_opt(12, 30, 45);
        let start_datetime = NaiveDateTime::new(start_date.unwrap(),start_time.unwrap());

        let end_date = NaiveDate::from_ymd_opt(2040, 1, 1);
        let end_time = NaiveTime::from_hms_opt(16, 30, 45);
        let end_datetime = NaiveDateTime::new(end_date.unwrap(),end_time.unwrap());
        
        let player_strings_vec = vec![String::from("Papapia"), String::from("Mamamia"), String::from("ASingleLaysPotatoChip")];

        let new_session = Session {
            username: String::from("ASingleLaysPotatoChip"),
            game_title: String::from("Hello Kitty Island Adventure"),
            campaign_title: Some(String::from("Descent into Avernus")),
            session_start: start_datetime,
            session_end: end_datetime,
            players: player_strings_vec,
            notes: Some(String::from("We enjoyed saving the merchant from wolves")),
            winner: false,
            winner_name: None,
            picture: None
        };

        let new_session_data = json!({
                "username": "ASingleLaysPotatoChip",
                "game_title": "Hello Kitty Island Adventure",
                "campaign_title": "Descent into Avernus",
                "session_start": "2040-01-01 12:30:45",
                "session_end": "2040-01-01 16:30:45",
                "players": ["Papapia", "Mamamia", "ASingleLaysPotatoChip"],
                "notes": "We enjoyed saving the merchant from wolves",
                "winner": false
        });

        //make request for posting session and send it to test
        req = test::TestRequest::post()
            .uri("/v1/users/session")
            .set_json(new_session_data)
            .to_request();
        resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        //make request for pulling profile data and extracting it into a struct
        //let finalreq = test::TestRequest::get()
        //    .uri("/v1/user/AsingleLaysPotatoChip")
        //    .to_request();
        //resp = test::call_service(&mut app, finalreq).await;
        //let json_response: SingleUserConvertedResponseData = test::read_body_json(resp).await;

        //assert_eq!(json_response, user_data);
    }
}
