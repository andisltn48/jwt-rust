use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct JWTClaims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt() -> Result<String, Box<dyn std::error::Error>> {
    let my_claims = JWTClaims {
        sub: "user123".to_string(),
        exp: 10000000000,  // Example expiration time (Unix timestamp)
    };

    let encoding_key = EncodingKey::from_secret("your-secret-key".as_ref());

    let token = encode(&Header::default(), &my_claims, &encoding_key)?;

    Ok(token)
}

// pub fn decode_jwt(token: &str) -> Result<JWTClaims, Box<dyn std::error::Error>> {
//     let decoding_key = DecodingKey::from_secret("your-secret-key".as_ref());

//     let validation = Validation {
//         leeway: 0,
//         validate_exp: true,
//         ..Validation::default()
//     };

//     let token_data = decode::<JWTClaims>(&token, &decoding_key, &validation)?;

//     Ok(token_data.claims)
// }

pub fn decode_jwt(token: &str) -> Result<JWTClaims, Box<dyn std::error::Error>> {
    // Secret key used to decode the token
    let decoding_key = DecodingKey::from_secret("your-secret-key".as_ref());

    // Validation settings (e.g., check expiration)
    let validation = Validation::new(Algorithm::HS256);

    // Decode the token and validate the claims
    let token_data: TokenData<JWTClaims> = decode::<JWTClaims>(&token, &decoding_key, &validation)?;

    // Return the decoded claims
    Ok(token_data.claims)
}