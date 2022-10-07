#[cfg(test)]
mod unit_tests {
    use actix_web::{body::to_bytes, http::header::AUTHORIZATION, test, web, App};
    use actix_web_httpauth::middleware::HttpAuthentication;
    use diesel::{r2d2, r2d2::ConnectionManager, PgConnection};
    use dotenvy::dotenv;
    use std::fs;
    use std::io::{BufRead, BufReader};

    use crate::auth::{create_token, validator};
    use crate::configuration::Application;
    use crate::db;
    use crate::handler::*;
    use crate::models;
    use crate::utils;
    use shared::auth::UserLogin;
    use shared::models::{NewTimeEntery, NewUser};

    #[actix_web::test]
    async fn test_login() {
        dotenv().ok();
        let settings = Application::default();
        let connection_manager = ConnectionManager::<PgConnection>::new(settings.database_url);
        let pool: models::db::Pool = r2d2::Pool::builder()
            .build(connection_manager)
            .expect("Failed to create pool.");
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .route("/", web::post().to(api::login)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&UserLogin {
                username: "Carlos-test".to_owned(),
                password: "12345678".to_owned(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp);
        assert!(resp.status().is_success());

        println!("Test if response was korrekt.");
        let body_bytes = to_bytes(resp.into_body()).await.unwrap();
        let token_str = create_token(
            "Carlos-test".to_string(),
            Vec::from([
                "ADMIN_ROLE".to_string(),
                "GET_LEDGER_INFO".to_string(),
                "SET_LEDGER_INFO".to_string(),
            ]),
        )
        .await
        .expect("Failed to unwrap Token");

        assert_eq!(
            body_bytes,
            web::Bytes::from(format!(
                r##"{{"username":"Carlos-test","token":"{}"}}"##,
                token_str
            ))
        );
    }

    #[actix_web::test]
    async fn test_create_user() {
        dotenv().ok();
        let settings = Application::default();
        let connection_manager = ConnectionManager::<PgConnection>::new(settings.database_url);
        let pool: models::db::Pool = r2d2::Pool::builder()
            .build(connection_manager)
            .expect("Failed to create pool.");
        let connection: &mut PgConnection = &mut pool.get().unwrap();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(pool.clone()))
                .route("/", web::post().to(api::create_user)),
        )
        .await;
        db::users::delete_user(connection, "create-test");
        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&NewUser {
                username: "create-test".to_owned(),
                password: "12345678".to_owned(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp);
        assert!(resp.status().is_success());

        println!("Test if response was korrekt.");
        let body_bytes = to_bytes(resp.into_body()).await.unwrap();
        // get userid
        let result = db::users::get_user(connection, "create-test").unwrap();
        db::users::delete_user(connection, "create-test");
        assert_eq!(
            body_bytes,
            web::Bytes::from(format!(
                r##"{{"user_id":"{}","username":"create-test","password":"{}"}}"##,
                result.user_id, result.password,
            ))
        );
    }

    #[actix_web::test]
    async fn test_password_hash_and_verify() {
        let pwd = "jkl";
        let pwd_hash = &utils::hash_password(pwd).unwrap();
        assert!(utils::verify(pwd_hash, pwd).unwrap());
    }

    #[actix_web::test]
    async fn test_get_html() {
        let token_str = create_token(
            "Carlos-test".to_string(),
            Vec::from(["GET_LEDGER_INFO".to_string()]),
        )
        .await
        .expect("Failed to unwrap Token");

        let auth = HttpAuthentication::bearer(validator);
        let app = test::init_service(App::new().wrap(auth).route(
            "/get_{target}/depth_{depth}/timeframe_{timeframe}.json",
            web::get().to(api::get_html),
        ))
        .await;
        let req = test::TestRequest::get()
            .uri("/get_music/depth_1/timeframe_all.json")
            .insert_header((AUTHORIZATION, format!("Bearer {}", token_str)))
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("Valid Request {:?}", resp);
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_ledger_time_suggestion() {
        let token_str = create_token(
            "Carlos-test".to_string(),
            Vec::from(["GET_LEDGER_INFO".to_string()]),
        )
        .await
        .expect("Failed to unwrap Token");

        let auth = HttpAuthentication::bearer(validator);
        let app = test::init_service(
            App::new()
                .wrap(auth)
                .route("/", web::get().to(api::get_ledger_time_suggetstions)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/")
            .insert_header((AUTHORIZATION, format!("Bearer {}", token_str)))
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("Valid Request {:?}", resp);
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_set_ledger_time() {
        let token_str = create_token(
            "Carlos-test".to_string(),
            Vec::from(["SET_LEDGER_INFO".to_string()]),
        )
        .await
        .expect("Failed to unwrap Token");

        let auth = HttpAuthentication::bearer(validator);
        let app = test::init_service(
            App::new()
                .wrap(auth)
                .route("/", web::post().to(api::set_ledger_time_entery_start)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .insert_header((AUTHORIZATION, format!("Bearer {}", token_str)))
            .set_json(&NewTimeEntery {
                headline: "Carlos is programming".to_owned(),
                account_origin: "FreeTime".to_owned(),
                account_target: "EducationRust".to_owned(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("Valid Request {:?}", resp);
        assert!(resp.status().is_success());

        //remove added line
        let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open("./files/time_spend.dat")
            .expect("file.txt doesn't exist or so");

        let lines_raw = BufReader::new(file)
            .lines()
            .map(|x| x.unwrap())
            .collect::<Vec<String>>();
        let lines = lines_raw[..lines_raw.len() - 1].join("\n");

        fs::write("./files/time_spend.dat", lines).expect("Can't write");
    }
}
