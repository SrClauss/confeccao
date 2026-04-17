use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Corte {
    pub id: i64,
    pub numero: i64,
    pub data: String,
    pub tamanho: String,
    pub rendimento: f64,
    pub divisor: f64,
    pub total_pecas: i64,
    pub observacao: Option<String>,
    pub criado_em: String,
}

#[derive(Debug, Deserialize)]
pub struct CriarCorte {
    pub numero: i64,
    pub data: String,
    pub tamanho: String,
    pub rendimento: f64,
    pub divisor: f64,
    pub observacao: Option<String>,
}
