use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct LinhaCorte {
    pub id: i64,
    pub corte_id: i64,
    pub num: i64,
    pub n_folhas: f64,
    pub a_mais: f64,
    pub cor: String,
    pub total: i64,
    pub soma_a_g: i64,
    pub total_geral: i64,
}

#[derive(Debug, Deserialize)]
pub struct CriarLinhaCorte {
    pub n_folhas: f64,
    pub a_mais: f64,
    pub cor: String,
}
