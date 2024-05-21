use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::env;
use warp::{filters::BoxedFilter, http::StatusCode, Filter, Rejection, Reply};
use chrono::{prelude::*, Duration};
use once_cell::sync::Lazy;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

static SECRET_KEY: Lazy<String> = Lazy::new(|| {
    env::var("SECRET_KEY").unwrap_or_else(|_| "secret_key".to_string())
});

fn create_token(user_id: &str) -> Result<String, &'static str> {
    let expiration = 60;
    let exp = Utc::now()
        .checked_add_signed(Duration::minutes(expiration))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: exp as usize,
    };

    info!("Creating token for user: {}", user_id);

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY.as_bytes()),
    )
    .map_err(|_| {
        warn!("Error creating the token");
        "Error creating the token"
    })
}

fn verify_token(token: &str) -> Result<Claims, &'static str> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY.as_bytes()),
        &Validation::default(),
    )
    .map(|data| {
        info!("Token is valid");
        data.claims
    })
    .map_err(|_| {
        warn!("Invalid token");
        "Invalid token"
    })
}

fn with_auth() -> BoxedFilter<()> {
    warp::header::<String>("authorization")
        .and_then(|value: String| async move {
            let token = value.trim_start_matches("Bearer ");
            verify_token(token).map_err(|_| {
                warn!("Unauthorized access attempt");
                Rejection::from(warp::reject())
            })
        })
        .untuple_one()
        .boxed()
}

fn protected_route(user: Claims) -> impl Reply {
    info!("Accessing protected route");
    warp::reply::json(&user)
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let protected = warp::path("protected")
        .and(with_auth())
        .map(|user: Claims| protected_route(user));

    let routes = protected.with(warp::cors().allow_any_origin());

    info!("Starting server on 127.0.0.1:3030");

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
