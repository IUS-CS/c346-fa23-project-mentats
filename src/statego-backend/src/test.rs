#[cfg(test)]
mod tests {
    use crate::models::{campaign::*, game::*, session::*, user::*};
    use crate::routes::{campaign::*, game::*, session::*, user::*};
    use actix_web::{test, web, App};
    use serde_json::json;
    use chrono::{NaiveDate, NaiveTime, NaiveDateTime};

    #[actix_rt::test]
    async fn a_test_hello_endpoint() {
        let mut app = test::init_service(App::new().service(hello)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
        assert_eq!(body, web::Bytes::from_static(b"Hello, cruel world"));
    }

    #[actix_rt::test]
    async fn first_user_creation_reading_updating_complete() {
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
                .service(create_user)
                .service(update_user_profile)
                .service(get_single_user),
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
    }    
    #[actix_rt::test]
    async fn second_game_campaign_session_creation() {

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
                .service(create_game)
                .service(create_campaign)
                .service(create_session)
        )
        .await;

        //make request for posting game and sending it to test
        let mut req = test::TestRequest::post()
            .uri("/v1/users/game")
            .set_json(new_game_data)
            .to_request();
        let mut resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());

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

    
    }
    #[actix_rt::test]
    async fn third_game_campaign_session_get() {

        let new_game = NewGame {
            username: String::from("ASingleLaysPotatoChip"),
            game_title: String::from("Hello Kitty Island Adventure"),
            description: Some(String::from("Animal Crossing but good"))
    };

        let game_find_into = json!({
            "username": "ASingleLaysPotatoChip"
        });

        let pool = crate::config::set_up_test_environment();
        let shared_data = web::Data::new(pool);

        let mut app = test::init_service(
            App::new()
                .app_data(shared_data.clone())
                .service(get_list_of_games)
                .service(get_list_of_campaigns)
                .service(get_list_of_sessions)
        )
        .await;

        //make request for pulling game list and checking equivalence
        let mut req = test::TestRequest::get()
            .uri("/v1/users/games")
            .set_json(game_find_into)
            .to_request();
        let mut resp = test::call_service(&mut app, req).await;
        let json_response: Vec<GameInfo> = test::read_body_json(resp).await;
        let game_info: GameInfo = json_response.first().unwrap().clone();
        assert_eq!(game_info.game_title, new_game.game_title);
        assert_eq!(game_info.description, new_game.description);

        let new_campaign = NewCampaign {
            username: String::from("ASingleLaysPotatoChip"),
            game_title: String::from("Hello Kitty Island Adventure"),
            campaign_title: String::from("Descent into Avernus"),
            description: Some(String::from("A quest to rescue Elturel from the depths")),
            notes: Some(String::from("Questioning the reason why the developers implemented death animations in Hello Kitty"))
        };

        let campaign_find = json!({
            "username": "ASingleLaysPotatoChip",
            "game_title": "Hello Kitty Island Adventure"
        });

        //make request for pulling campaign list and checking equivalence
        req = test::TestRequest::get()
            .uri("/v1/users/campaigns")
            .set_json(campaign_find)
            .to_request();
        resp = test::call_service(&mut app, req).await;
        let json_response: Vec<CampaignInfo> = test::read_body_json(resp).await;
        let campaign_info: CampaignInfo = json_response.first().unwrap().clone();
        assert_eq!(campaign_info.campaign_title, new_campaign.campaign_title);
        assert_eq!(campaign_info.description, new_campaign.description);
        assert_eq!(campaign_info.notes, new_campaign.notes);

        let start_date = NaiveDate::from_ymd_opt(2040, 1, 1);
        let start_time = NaiveTime::from_hms_opt(12, 30, 45);
        let start_datetime = NaiveDateTime::new(start_date.unwrap(),start_time.unwrap());

        let end_date = NaiveDate::from_ymd_opt(2040, 1, 1);
        let end_time = NaiveTime::from_hms_opt(16, 30, 45);
        let end_datetime = NaiveDateTime::new(end_date.unwrap(),end_time.unwrap());
        
        let player_strings_vec = vec![String::from("Papapia"), String::from("Mamamia"), String::from("ASingleLaysPotatoChip")];

        let new_session = SessionDataConverted {
            session_id:  64, //placeholder session_id this is only for frontend
            user_name: String::from("ASingleLaysPotatoChip"),
            game_title: String::from("Hello Kitty Island Adventure"),
            campaign_title: Some(String::from("Descent into Avernus")),
            session_start: start_datetime,
            session_end: end_datetime,
            players: player_strings_vec,
            notes: Some(String::from("We enjoyed saving the merchant from wolves")),
            winner: false,
            winner_name: None,
            picture: None,
            number_of_players: 3
        };

        let session_find = json!({
            "username": "ASingleLaysPotatoChip",
            "game_title": "Hello Kitty Island Adventure",
            "campaign_title": "Descent into Avernus"
        });

        //make request for pulling session list and checking equivalence
        req = test::TestRequest::get()
            .uri("/v1/users/session")
            .set_json(session_find)
            .to_request();
        resp = test::call_service(&mut app, req).await;
        let json_response: Vec<SessionDataConverted> = test::read_body_json(resp).await;
        let session_info: SessionDataConverted = json_response.first().unwrap().clone();
        assert_eq!(session_info.user_name, new_session.user_name);
        assert_eq!(session_info.game_title, new_session.game_title);
        assert_eq!(session_info.campaign_title, new_session.campaign_title);
        assert_eq!(session_info.session_start, new_session.session_start);
        assert_eq!(session_info.session_end, new_session.session_end);
        assert_eq!(session_info.players, new_session.players);
        assert_eq!(session_info.notes, new_session.notes);
        assert_eq!(session_info.winner, new_session.winner);
        assert_eq!(session_info.winner_name, new_session.winner_name);
        assert_eq!(session_info.picture, new_session.picture);
        assert_eq!(session_info.number_of_players, new_session.number_of_players);
    }
    #[actix_rt::test]
    #[should_panic]
    async fn fourth_game_campaign_session_delete() {

        let pool = crate::config::set_up_test_environment();
        let shared_data = web::Data::new(pool);

        let mut app = test::init_service(
            App::new()
                .app_data(shared_data.clone())
                .service(create_session)
                .service(get_list_of_sessions)
                .service(delete_session)
                .service(get_list_of_sessions_from_campaign)
                .service(get_list_of_sessions_from_game)
                .service(get_single_session)
                .service(create_game)
                .service(get_single_game)
                .service(get_list_of_games)
                .service(create_campaign)
                .service(get_single_campaign)
                .service(get_list_of_campaigns)
                .service(delete_campaign)
                .service(delete_game)

        )
        .await;

        //get id for game
        let game_find_into = json!({
            "username": "ASingleLaysPotatoChip"
        });
        let mut req = test::TestRequest::get()
            .uri("/v1/users/games")
            .set_json(game_find_into)
            .to_request();
        let mut resp = test::call_service(&mut app, req).await;
        let json_response: Vec<GameInfo> = test::read_body_json(resp).await;
        let game_info: GameInfo = json_response.first().unwrap().clone();
        let game_id = game_info.game_id;

        //get id for campaigns under the game
        let campaign_find_into = json!({
            "username": "ASingleLaysPotatoChip",
            "game_title": "Hello Kitty Island Adventure"
        });
        req = test::TestRequest::get()
            .uri("/v1/users/campaigns")
            .set_json(campaign_find_into)
            .to_request();
        resp = test::call_service(&mut app, req).await;
        let json_response: Vec<CampaignInfo> = test::read_body_json(resp).await;
        
        let mut campaign_id_vec: Vec<u64> = Vec::new();
        for campaign_info in json_response{
            campaign_id_vec.push(campaign_info.campaign_id);
        }

        //get id for sessions under the game
        let sessions_find_into = json!({
            "username": "ASingleLaysPotatoChip",
            "game_title": "Hello Kitty Island Adventure",
            "campaign_title": "Descent into Avernus"
        });
        req = test::TestRequest::get()
            .uri("/v1/users/session")
            .set_json(sessions_find_into)
            .to_request();
        resp = test::call_service(&mut app, req).await;
        let json_response: Vec<SessionDataConverted> = test::read_body_json(resp).await;
        
        let mut session_id_vec: Vec<u64> = Vec::new();
        for session_converted in json_response{
            session_id_vec.push(session_converted.session_id);
        }

        //getting game id ready for delete request
        let game_id_string: String = game_id.to_string();
        let request_str = "/v1/users/game/";
        let request_string_with_id = format!("{}{}", request_str, game_id_string);
        //make request for deleting game
        req = test::TestRequest::delete()
            .uri(&request_string_with_id)
            .to_request();
        resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        //check that game is deleted by using get_single_game route with game id
        //this will also require
        // let result = std::panic::catch_unwind(|| <expected_to_panic_operation_here>);
        //assert!(result.is_err())
        //make request for getting deleted game
        
         //getting a panic with a single error
         //make request for getting single game
         req = test::TestRequest::get()
             .uri(&request_string_with_id)
             .to_request();
         resp = test::call_service(&mut app, req).await;

        //check that campaign is deleted by using get_single_campaign with campaign ids listed under game
        //this will require pulling a vector of campaign ids
        //this will also require
        // let result = std::panic::catch_unwind(|| <expected_to_panic_operation_here>);
        //assert!(result.is_err());

        for campaign_id in campaign_id_vec{
            let campaign_id_string: String = campaign_id.to_string();
            let request_str = "/v1/users/campaign/";
            let request_string_with_id = format!("{}{}", request_str, campaign_id_string);
            req = test::TestRequest::get()
             .uri(&request_string_with_id)
             .to_request();
            resp = test::call_service(&mut app, req).await;
        }
        


        //check that sessions are deleted by using get_single_session with session ids listed under game
        //this will require pulling a vector of session ids
        //this will also require
        // let result = std::panic::catch_unwind(|| <expected_to_panic_operation_here>);
        //assert!(result.is_err());
    
        for session_id in session_id_vec{
            let session_id_string: String = session_id.to_string();
            let request_str = "/v1/users/session/";
            let request_string_with_id = format!("{}{}", request_str, session_id_string);
            req = test::TestRequest::get()
             .uri(&request_string_with_id)
             .to_request();
            resp = test::call_service(&mut app, req).await;
        }
        


    }
}

