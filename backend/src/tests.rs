#[cfg(test)]
mod unit_tests {
    use actix_web::{body::to_bytes, test, web, App};

    use crate::auth::{create_token, UserPermissions};
    use crate::handler::*;

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

        println!("Test if answer was korrekt.");
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
}
