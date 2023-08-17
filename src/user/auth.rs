use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rocket::{
    http::{Status},
    request::{FromRequest, Outcome},
    serde::{json::Json, Deserialize, Serialize},
    Request
};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    exp: usize,
    iat: usize,
    id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Message {
    msg: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct EncodeResponse {
    token: String,
}
#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[post("/login", data="<login_info>")]
pub fn login(login_info: Json<LoginInfo>) -> Json<EncodeResponse> {
    let login_info = login_info.into_inner();
    // Normally here you'd check the credentials and return the user id from the DB
    // We'll just do this for now just to have something
    // Calculates the sum of the characters
    let id = login_info.username.chars().map(|x| x as i32).sum();

    let secret = env::var("SECRET").expect("Please set a secret key");
    let exp = (Utc::now() + Duration::hours(24)).timestamp() as usize;
    let iat = Utc::now().timestamp() as usize;
    let claims = Claims { exp, iat, id };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_str().as_ref()),
    )
    .unwrap();

    
    Json(EncodeResponse{token} )
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct AuthenticationToken {
    pub id: usize,
}

#[derive(Debug)]
pub enum TokenError {
    Missing,
    Invalid,
}

#[async_trait]
impl<'a> FromRequest<'a> for AuthenticationToken {
    type Error = TokenError;
    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let token = request.headers().get_one("Authorization").unwrap_or("");
        if token.is_empty() {
            return Outcome::Failure((Status::Unauthorized, TokenError::Missing));
        }
        let secret = env::var("SECRET").expect("Please set a secret key");
        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret.as_str().as_ref()),
            &Validation::new(Algorithm::HS256),
        );
        match decoded {
            Ok(token) => Outcome::Success(AuthenticationToken {
                id: token.claims.id as usize,
            }),
            Err(_) => Outcome::Failure((Status::Unauthorized, TokenError::Invalid)),
        }
    }
}


