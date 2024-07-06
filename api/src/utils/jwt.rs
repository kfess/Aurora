use anyhow::Result;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn encode_jwt(secret: &str, user_id: &str) -> Result<String> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(7))
        .ok_or_else(|| anyhow::anyhow!("Failed to calculate expiration"))?
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };

    let header = {
        let mut h = Header::default();
        h.alg = Algorithm::HS256;
        h
    };

    let key = EncodingKey::from_secret(secret.as_ref());

    let jwt = encode(&header, &claims, &key)?;

    Ok(jwt)
}

pub fn decode_jwt(secret: &str, jwt: &str) -> Result<String> {
    match decode::<Claims>(
        &jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(token_data) => Ok(token_data.claims.sub),
        Err(e) => Err(anyhow::anyhow!(e)),
    }
}
