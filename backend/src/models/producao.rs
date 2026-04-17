use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Producao {
    pub id: i64,
    pub funcionaria_id: i64,
    pub corte_id: i64,
    pub linha_corte_id: i64,
    pub operacao: String,
    pub quantidade: i64,
    pub data_hora: String,
}

#[derive(Debug, Serialize)]
pub struct ProducaoDetalhada {
    pub id: i64,
    pub funcionaria_id: i64,
    pub funcionaria_nome: String,
    pub corte_id: i64,
    pub linha_corte_id: i64,
    pub cor: String,
    pub operacao: String,
    pub quantidade: i64,
    pub data_hora: String,
}

#[derive(Debug, Deserialize)]
pub struct LancarProducao {
    pub linha_corte_id: i64,
    pub operacao: String,
    pub quantidade: i64,
}
