use std::{future::Future, pin::Pin};

use actix_web::{http::header, FromRequest, HttpMessage};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AuthToken(String);

impl FromRequest for AuthToken {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        match get_auth_token_from_header(req) {
            Ok("super-secret-password") => {
                Box::pin(async move { Ok(AuthToken("super-secret-password".into())) })
            }
            _ => Box::pin(async move { Err(actix_web::error::ErrorUnauthorized("Invalid")) }),
        }
    }
}

fn get_auth_token_from_header(req: &impl HttpMessage) -> Result<&str, &str> {
    req.headers()
        .get(header::AUTHORIZATION)
        .and_then(|t| t.to_str().ok())
        .and_then(|t| t.strip_prefix("Bearer "))
        .ok_or("missing error")
}
