use axum::{
    extract::{State, Extension},
    routing::{get, post},
    Router, Json,
};
use serde_json::{json, Value};
use sqlx::SqlitePool;
use crate::{
    auth::jwt::Claims,
    errors::AppError,
    models::producao::LancarProducao,
};

async fn listar_producao(
    Extension(claims): Extension<Claims>,
    State(pool): State<SqlitePool>,
) -> Result<Json<Value>, AppError> {
    if claims.cargo != "admin" {
        return Err(AppError::Proibido);
    }

    let producoes = sqlx::query!(
        r#"SELECT p.id, p.funcionaria_id, u.nome as funcionaria_nome,
           p.corte_id, p.linha_corte_id, lc.cor, p.operacao, p.quantidade, p.data_hora
           FROM producao p
           JOIN usuarios u ON u.id = p.funcionaria_id
           JOIN linhas_corte lc ON lc.id = p.linha_corte_id
           ORDER BY p.data_hora DESC"#
    )
    .fetch_all(&pool)
    .await?;

    let lista: Vec<_> = producoes.iter().map(|p| json!({
        "id": p.id,
        "funcionaria_id": p.funcionaria_id,
        "funcionaria_nome": p.funcionaria_nome,
        "corte_id": p.corte_id,
        "linha_corte_id": p.linha_corte_id,
        "cor": p.cor,
        "operacao": p.operacao,
        "quantidade": p.quantidade,
        "data_hora": p.data_hora
    })).collect();

    Ok(Json(json!(lista)))
}

async fn lancar_producao(
    Extension(claims): Extension<Claims>,
    State(pool): State<SqlitePool>,
    Json(body): Json<LancarProducao>,
) -> Result<Json<Value>, AppError> {
    if body.quantidade <= 0 {
        return Err(AppError::BadRequest("Quantidade deve ser maior que zero".to_string()));
    }

    let linha = sqlx::query!(
        "SELECT corte_id FROM linhas_corte WHERE id = ?",
        body.linha_corte_id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NaoEncontrado)?;

    let result = sqlx::query!(
        "INSERT INTO producao (funcionaria_id, corte_id, linha_corte_id, operacao, quantidade)
         VALUES (?, ?, ?, ?, ?) RETURNING id, data_hora",
        claims.sub,
        linha.corte_id,
        body.linha_corte_id,
        body.operacao,
        body.quantidade
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(json!({
        "id": result.id,
        "funcionaria_id": claims.sub,
        "corte_id": linha.corte_id,
        "linha_corte_id": body.linha_corte_id,
        "operacao": body.operacao,
        "quantidade": body.quantidade,
        "data_hora": result.data_hora
    })))
}

async fn minhas_producoes(
    Extension(claims): Extension<Claims>,
    State(pool): State<SqlitePool>,
) -> Result<Json<Value>, AppError> {
    let hoje = chrono::Utc::now().format("%Y-%m-%d").to_string();

    let producoes = sqlx::query!(
        r#"SELECT p.id, p.corte_id, c.numero as corte_numero, p.linha_corte_id,
           lc.cor, p.operacao, p.quantidade, p.data_hora
           FROM producao p
           JOIN linhas_corte lc ON lc.id = p.linha_corte_id
           JOIN cortes c ON c.id = p.corte_id
           WHERE p.funcionaria_id = ? AND date(p.data_hora) = ?
           ORDER BY p.data_hora DESC"#,
        claims.sub,
        hoje
    )
    .fetch_all(&pool)
    .await?;

    let lista: Vec<_> = producoes.iter().map(|p| json!({
        "id": p.id,
        "corte_id": p.corte_id,
        "corte_numero": p.corte_numero,
        "linha_corte_id": p.linha_corte_id,
        "cor": p.cor,
        "operacao": p.operacao,
        "quantidade": p.quantidade,
        "data_hora": p.data_hora
    })).collect();

    Ok(Json(json!(lista)))
}

pub fn rotas(pool: SqlitePool) -> Router {
    Router::new()
        .route("/producao", get(listar_producao).post(lancar_producao))
        .route("/producao/minhas", get(minhas_producoes))
        .with_state(pool)
}
