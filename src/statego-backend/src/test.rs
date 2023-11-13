#[cfg(test)]
mod tests {
    use crate::routes;
    use crate::models::*;
    use actix_web::{test, web, App};
    use serde_json::{json, from_str, Value};


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
    async fn create_user() {
        //let user_data = UserDetails {
        //        email: String::from("SweetSouthernHeatBarbecue@gmail.com"),
        //        username: String::from("ASingleLaysPotatoChip"),
        //        pass:  String::from("ThisIsGonnaBeHashedAnyway"),
        //        first_name: String::from("Frito"),
        //        last_name:  String::from("Lay")
        //};

        let my_data = json!({ 
                "email": "SweetSouthernHeatBarbecue@gmail.com",
                "username": "ASingleLaysPotatoChip",
                "pass": "ThisIsGonnaBeHashedAnyway",
                "first_name": "Frito",
                "last_name": "Lay"     
        });

        let pool = crate::config::set_up_environment();
        let shared_data = web::Data::new(pool);
        
        let mut app = test::init_service(App::new().app_data(shared_data.clone()).service(routes::create_user)).await;
        let req = test::TestRequest::post().uri("/v1/users").set_json(my_data).to_request();
        let resp = test::call_service(&mut app, req).await;

        assert!(resp.status().is_success());
        let body = test::read_body(resp).await;
    }
}