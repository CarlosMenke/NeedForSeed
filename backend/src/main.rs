extern crate diesel;
extern crate dotenvy;
extern crate serde;

use actix_cors::Cors;
use actix_identity::IdentityMiddleware;
use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;

use diesel::{r2d2, r2d2::ConnectionManager, PgConnection};
use models::db::Pool;

use dotenvy::dotenv;

use configuration::Application;
use handler::api;

mod auth;
mod configuration;
mod db;
mod errors;
mod handler;
mod models;
mod tests;
mod utils;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::builder().format_timestamp(None).init();

    let settings = Application::default();
    let private_key = Key::generate();

    let connection_manager = ConnectionManager::<PgConnection>::new(settings.database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(connection_manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        let cors = Cors::permissive()
            //.allow_any_origin()
            //.allowed_origin("http://127.0.0.1:8080")
            //.allowed_methods(vec!["GET", "POST"])
            //.allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            //.allowed_header(header::CONTENT_TYPE)
            //.supports_credentials()
            //.disable_preflight()
            .max_age(3600);
        let auth = HttpAuthentication::bearer(auth::validator);
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new(settings.redis_uri.clone()),
                    private_key.clone(),
                )
                .build(),
            )
            .service(
                web::scope("/api")
                    .route("login.json", web::post().to(api::login))
                    .route("create_user.json", web::post().to(api::create_user))
                    .service(web::scope("/auth").wrap(auth).route(
                        "get_music_depth/{depth}.json",
                        web::get().to(api::get_music),
                    )),
            )
    })
    .bind(format!("{}:{}", settings.server_ip, settings.server_port))
    .expect("Can not bind to IP:PORT")
    .run()
    .await
}
