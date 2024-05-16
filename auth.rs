[dependencies]
log = "0.4"
env_logger = "0.9"
serde = "1.0"
serde_derive = "1.0"
warp = "0.3"
jsonwebtoken = "7"
chrono = "0.4"
tokio = { version = "1", features = ["full"] }
```
```rust
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::env;
use warp::{filters::BoxedFilter, http::StatusCode, Filter, Rejection, Reply};
use chrono::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // Subject (user_id in this case)
    exp: usize,  // Expiry
}

fn create_token(user_id: &str) -> Result<String, &'static str> {
    let secret = env::var("SECRET_KEY").unwrap_or_else(|_| "secret_key".to_string());
    let expiration = 60;
    let exp = Utc::now()
        .checked_add_signed(chrono::Duration::minutes(expiration))
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
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| {
        warn!("Error creating the token");
        "Error creating the token"
    })
}

fn verify_token(token: &str) -> Result<Claims, &'static str> {
    let secret = env::var("SECRET_KEY").unwrap_or_else(|_| "secret_key".to_string());
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
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
            let token = value.trim_start_matches("Bearer ").to_string();
            verify_token(&token).map_err(|_| {
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