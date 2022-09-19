#[cfg(test)]
mod unit_tests {
    use actix_web::{body::to_bytes, test, web, App};
    use diesel::{r2d2, r2d2::ConnectionManager, PgConnection};
    use dotenvy::dotenv;

    use crate::auth::create_token;
    use crate::configuration::Application;
    use crate::db;
    use crate::handler::*;
    use crate::models;
    use shared::auth::UserPermissions;
    use shared::models::NewUser;

    #[actix_web::test]
    async fn test_login() {
        let app = test::init_service(App::new().route("/", web::post().to(api::login))).await;
        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&UserPermissions {
                username: "my-name".to_owned(),
                permissions: Vec::from(["ADMIN_ROLE".to_string()]),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp);
        assert!(resp.status().is_success());

        println!("Test if response was korrekt.");
        let body_bytes = to_bytes(resp.into_body()).await.unwrap();
        let token_str = create_token("my-name".to_string(), Vec::from(["ADMIN_ROLE".to_string()]))
            .await
            .expect("Failed to unwrap Token");

        assert_eq!(
            body_bytes,
            web::Bytes::from(format!(
                r##"{{"username":"my-name","permissions":["ADMIN_ROLE"],"token":"{}"}}"##,
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
        db::users::delete_user(connection, "my-name");
        let req = test::TestRequest::post()
            .uri("/")
            .set_json(&NewUser {
                username: "my-name".to_owned(),
                password: "12345678".to_owned(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp);
        assert!(resp.status().is_success());

        println!("Test if response was korrekt.");
        let body_bytes = to_bytes(resp.into_body()).await.unwrap();
        // get userid
        let result = db::users::get_user(connection, "my-name").unwrap();
        db::users::delete_user(connection, "my-name");
        assert_eq!(
            body_bytes,
            web::Bytes::from(format!(
                r##"{{"user_id":"{}","username":"my-name","password":"{}"}}"##,
                result.user_id, result.password,
            ))
        );
    }
}
