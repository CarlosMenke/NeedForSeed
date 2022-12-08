use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web::Error;
use actix_web_grants::permissions::AttachPermissions;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use chrono::{Duration, Utc};
use jsonwebtoken::{self, DecodingKey, EncodingKey, Header, Validation};

use log::debug;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    // We just get permissions from JWT
    let result = decode_jwt(credentials.token());
    match result {
        Ok(claims) => {
            req.attach(claims.permissions);
            debug!("Token is valid");
            Ok(req)
        }
        // required by `actix-web-httpauth` validator signature
        Err(e) => {
            debug!("Token is invalid");
            Err((e, req))
        }
    }
}

pub async fn create_token(username: String, permissions: Vec<String>) -> Result<String, Error> {
    let claims = Claims::new(username, permissions);
    let jwt = create_jwt(claims)?;
    Ok(jwt)
}

//TODO implement enum or so with all role models, to restirct them
const JWT_EXPIRATION_HOURS: i64 = 24 * 7;
const SECRET: &str = "SECRET";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub username: String,
    pub permissions: Vec<String>,
    exp: i64,
}

impl Claims {
    fn new(username: String, permissions: Vec<String>) -> Self {
        Self {
            username,
            permissions,
            exp: (Utc::now() + Duration::hours(JWT_EXPIRATION_HOURS)).timestamp(),
        }
    }
}

/// Create a json web token (JWT)
fn create_jwt(claims: Claims) -> Result<String, Error> {
    debug!("Create jwt Token. {:?}", claims);
    let encoding_key = EncodingKey::from_secret(SECRET.as_bytes());
    jsonwebtoken::encode(&Header::default(), &claims, &encoding_key)
        .map_err(|e| ErrorUnauthorized(e.to_string()))
}

//TODO change to RSA for securety
//TODO think about to make it public
/// Decode a json web token (JWT)
pub fn decode_jwt(token: &str) -> Result<Claims, Error> {
    let decoding_key = DecodingKey::from_secret(SECRET.as_bytes());

    jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| ErrorUnauthorized(e.to_string()))
}
