use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Usuario {
    pub id: i64,
    pub nome: String,
    pub pin: String,
    pub cargo: String,
    pub ativo: i64,
    pub criado_em: String,
}

#[derive(Debug, Deserialize)]
pub struct CriarUsuario {
    pub nome: String,
    pub pin: String,
    pub cargo: String,
}

#[derive(Debug, Serialize)]
pub struct UsuarioPublico {
    pub id: i64,
    pub nome: String,
    pub cargo: String,
}
