//TODO restructure tests. Move function testing to used file, rest to tests folder beside src
#[cfg(test)]
mod unit_tests {
    use actix_web::{body::to_bytes, http::header::AUTHORIZATION, test, web, App};
    use actix_web_httpauth::middleware::HttpAuthentication;
    use diesel::{r2d2, r2d2::ConnectionManager, PgConnection};
    use dotenvy::dotenv;
    use std::fs;
    use std::io::Write;

    use crate::auth::{create_token, validator};
    use crate::configuration::Application;
    use crate::db;
    use crate::handler::*;
    use crate::models;
    use crate::utils;
    use shared::auth::UserLogin;
    use shared::models::*;

    pub const FILE_DIR: &str = "./files";
    pub const PATH_TIME_SPEND: &str = "time_spend.dat";
    // all finance files. First one is the default
    pub const PATH_FINANCE_FILES: [&'static str; 4] =
        ["nachhilfe.dat", "invest.dat", "rent.dat", "gesamt.dat"];

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
    async fn test_get_html() {
        let token_str = create_token(
            "Carlos-test".to_string(),
            Vec::from(["GET_LEDGER_INFO".to_string()]),
        )
        .await
        .expect("Failed to unwrap Token");

        let auth = HttpAuthentication::bearer(validator);
        let app = test::init_service(App::new().wrap(auth).route(
            "/get_{target}/depth_{depth}/timeframe_{timeframe}/timepoint_{timepoint}.json",
            web::get().to(api::get_html),
        ))
        .await;
        let req = test::TestRequest::get()
            .uri("/get_music/depth_1/timeframe_all/timepoint_0.json")
            .insert_header((AUTHORIZATION, format!("Bearer {}", token_str)))
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("Valid Request {:?}", resp);
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_get_time_suggestion() {
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
                .route("/", web::get().to(api::get_time_suggetstions)),
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
    async fn test_set_time_start() {
        let user = "Carlos-test".to_string();
        let token_str = create_token(user.clone(), Vec::from(["SET_LEDGER_INFO".to_string()]))
            .await
            .expect("Failed to unwrap Token");
        let start_entery = shared::models::StartTimeEntery {
            headline: "Carlos is programming".to_owned(),
            account_origin: "FreeTime".to_owned(),
            account_target: "EducationRust".to_owned(),
            duration: None,
            date: None,
            offset: None,
        };

        let auth = HttpAuthentication::bearer(validator);
        let app = test::init_service(
            App::new()
                .wrap(auth)
                .route("/", web::post().to(api::set_time_entery_start)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .insert_header((AUTHORIZATION, format!("Bearer {}", token_str)))
            .set_json(&start_entery)
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("Valid Request {:?}", resp);
        assert!(resp.status().is_success());
        let remove_line = utils::ledger_start_time_entery(&user, start_entery).unwrap();

        //remove added line
        let ledger =
            fs::read_to_string(format!("{}/{}/{}", FILE_DIR, &user, PATH_TIME_SPEND)).unwrap();
        fs::File::create(format!("{}/{}/{}", FILE_DIR, &user, PATH_TIME_SPEND))
            .unwrap()
            .write(
                ledger
                    .replace(&format!("{}\n", &remove_line), "")
                    .as_bytes(),
            )
            .unwrap();
    }

    #[actix_web::test]
    async fn test_get_time_running() {
        let user = "Carlos-test".to_string();
        let token_str = create_token(user.clone(), Vec::from(["GET_LEDGER_INFO".to_string()]))
            .await
            .expect("Failed to unwrap Token");

        let auth = HttpAuthentication::bearer(validator);
        let app = test::init_service(
            App::new()
                .wrap(auth)
                .route("/", web::get().to(api::get_time_entery_running)),
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

    ///tests also basic ledger functions
    #[actix_web::test]
    async fn test_set_time_stop() {
        let user = "Carlos-test".to_string();
        let token_str = create_token(user.clone(), Vec::from(["SET_LEDGER_INFO".to_string()]))
            .await
            .expect("Failed to unwrap Token");
        let start_entery = shared::models::StartTimeEntery {
            headline: "Carlos is programming".to_owned(),
            account_origin: "FreeTime".to_owned(),
            account_target: "EducationRust".to_owned(),
            duration: None,
            date: None,
            offset: None,
        };
        let remove_line = utils::ledger_start_time_entery(&user, start_entery).unwrap();
        //TODO find error
        let new_entery = utils::ledger_get_running_time_entery(&user)
            .unwrap()
            .get(&remove_line)
            .unwrap()
            .clone();

        let auth = HttpAuthentication::bearer(validator);
        let app = test::init_service(
            App::new()
                .wrap(auth)
                .route("/", web::post().to(api::set_time_entery_stop)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .insert_header((AUTHORIZATION, format!("Bearer {}", token_str)))
            .set_json(&StopLedgerTimeEntery {
                remove_line: remove_line.clone(),
                new_entery: new_entery.clone(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("Valid Request {:?}", resp);
        assert!(resp.status().is_success());

        //remove added line
        let ledger =
            fs::read_to_string(format!("{}/{}/{}", FILE_DIR, &user, PATH_TIME_SPEND)).unwrap();
        fs::File::create(format!("{}/{}/{}", FILE_DIR, &user, PATH_TIME_SPEND))
            .unwrap()
            .write(
                ledger
                    .replace(
                        &utils::ledger_create_time_entery(&user, new_entery).unwrap(),
                        "",
                    )
                    .as_bytes(),
            )
            .unwrap();
    }

    #[actix_web::test]
    async fn test_set_finance_create() {
        let user = "Carlos-test".to_string();
        let token_str = create_token(user.clone(), Vec::from(["SET_LEDGER_INFO".to_string()]))
            .await
            .expect("Failed to unwrap Token");
        let new_entery = shared::models::NewFinanceEntery {
            headline: "Carlos is programming".to_owned(),
            account_origin: "FreeTime".to_owned(),
            account_target: "Girokonto:N2".to_owned(),
            ammount: 10 as f32,
            date: None,
            target_file: "Finance".to_string(),
        };

        let auth = HttpAuthentication::bearer(validator);
        let app = test::init_service(
            App::new()
                .wrap(auth)
                .route("/", web::post().to(api::set_finance_entery_create)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/")
            .insert_header((AUTHORIZATION, format!("Bearer {}", token_str)))
            .set_json(&new_entery)
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("Valid Request {:?}", resp);
        assert!(resp.status().is_success());
        let remove_line = utils::ledger_create_finance_entery(&user, new_entery).unwrap();

        //remove added line
        let ledger =
            fs::read_to_string(format!("{}/{}/{}", FILE_DIR, &user, PATH_FINANCE_FILES[0]))
                .unwrap();
        fs::File::create(format!("{}/{}/{}", FILE_DIR, &user, PATH_FINANCE_FILES[0]))
            .unwrap()
            .write(ledger.replace(&format!("{}", &remove_line), "").as_bytes())
            .unwrap();
    }
    //TODO add finance test
    //TODO add history test.
}
