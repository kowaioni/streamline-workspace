use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use warp::{filters::BoxedFilter, http::StatusCode, Filter, Rejection, Reply};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, 
    exp: usize,  
}

fn create_token(user_id: &str) -> Result<String, &'static str> {
    let secret = env::var("SECRET_KEY").unwrap_or_else(|_| "secret_key".to_string());
    let expiration = 60; 
    let exp = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::minutes(expiration))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: exp as usize,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| "Error creating the token")
}

fn verify_token(token: &str) -> Result<Claims, &'static str> {
    let secret = env::var("SECRET_KEY").unwrap_or_else(|_| "secret_key".to_string());
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| "Invalid token")
}

fn with_auth() -> BoxedFilter<()> {
    warp::header::<String>("authorization")
        .and_then(|value: String| async move {
            let token = value.trim_start_matches("Bearer ").to_string();
            verify_token(&token).map_err(|_| Rejection::from(warp::reject()))
        })
        .untuple_one()
        .boxed()
}

fn protected_route(user: Claims) -> impl Reply {
    warp::reply::json(&user)
}

#[tokio::main]
async fn main() {
    let protected = warp::path("protected")
        .and(with_auth())
        .map(|user: Claims| protected_route(user));

    let routes = protected.with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}