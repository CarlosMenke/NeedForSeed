use std::env;

#[derive(Debug)]
pub struct Application {
    pub redis_uri: String,
    pub domain: String,
    pub database_url: String,
    pub server_ip: String,
    pub server_port: String,
}

impl Default for Application {
    fn default() -> Application {
        Application {
            redis_uri: env::var("REDIS_URL").expect("REDIS_URL must be set in .env."),
            domain: env::var("DOMAIN").expect("DMOAIN must be set in .env."),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env"),
            server_ip: env::var("SERVER_IP").expect("SERVER_IP must be set in .evn."),
            server_port: env::var("SERVER_PORT").expect("SERVER_PORT must be set in .evn."),
        }
    }
}
