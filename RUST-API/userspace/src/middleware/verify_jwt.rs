use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: Uuid,
    exp: usize,
}

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = Status;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        if let Some(cookie) = request.cookies().get_private("auth_token") {
            let token = cookie.value();
            match decode_token(token) {
                Ok(user_id) => Outcome::Success(AuthenticatedUser { user_id }),
                Err(_) => Outcome::Error((Status::Unauthorized, Status::Unauthorized)),
            }
        } else {
            Outcome::Error((Status::Unauthorized, Status::Unauthorized))
        }
    }
}

fn decode_token(token: &str) -> Result<Uuid, ()> {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());

    let token_data = decode::<Claims>(token, &decoding_key, &Validation::default())
        .map_err(|_| ())?;

    Ok(token_data.claims.sub)
}
