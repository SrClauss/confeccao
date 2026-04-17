use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::config::jwt_secret;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: i64,
    pub nome: String,
    pub cargo: String,
    pub exp: usize,
}

pub fn criar_token(id: i64, nome: &str, cargo: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiracao = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("Falha ao calcular expiração")
        .timestamp() as usize;

    let claims = Claims {
        sub: id,
        nome: nome.to_string(),
        cargo: cargo.to_string(),
        exp: expiracao,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret().as_bytes()),
    )
}

pub fn validar_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let dados = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret().as_bytes()),
        &Validation::default(),
    )?;
    Ok(dados.claims)
}
