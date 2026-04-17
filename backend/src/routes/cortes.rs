use axum::{
    extract::{State, Path, Extension},
    routing::{get, post, delete},
    Router, Json,
};
use serde_json::{json, Value};
use sqlx::SqlitePool;
use crate::{
    auth::jwt::Claims,
    errors::AppError,
    models::{corte::CriarCorte, linha_corte::CriarLinhaCorte},
};

fn calcular_linha(n_folhas: f64, a_mais: f64, rendimento: f64, divisor: f64) -> (i64, i64, i64) {
    let total = (n_folhas * rendimento).floor() as i64;
    let soma_a_g = if divisor > 0.0 { (a_mais / divisor).floor() as i64 } else { 0 };
    let total_geral = total + soma_a_g;
    (total, soma_a_g, total_geral)
}

async fn listar_cortes(
    State(pool): State<SqlitePool>,
) -> Result<Json<Value>, AppError> {
    let cortes = sqlx::query!(
        "SELECT id, numero, data, tamanho, rendimento, divisor, total_pecas, observacao, criado_em
         FROM cortes ORDER BY numero DESC"
    )
    .fetch_all(&pool)
    .await?;

    let lista: Vec<_> = cortes.iter().map(|c| json!({
        "id": c.id,
        "numero": c.numero,
        "data": c.data,
        "tamanho": c.tamanho,
        "rendimento": c.rendimento,
        "divisor": c.divisor,
        "total_pecas": c.total_pecas,
        "observacao": c.observacao,
        "criado_em": c.criado_em
    })).collect();

    Ok(Json(json!(lista)))
}

async fn criar_corte(
    Extension(claims): Extension<Claims>,
    State(pool): State<SqlitePool>,
    Json(body): Json<CriarCorte>,
) -> Result<Json<Value>, AppError> {
    if claims.cargo != "admin" {
        return Err(AppError::Proibido);
    }

    let result = sqlx::query!(
        "INSERT INTO cortes (numero, data, tamanho, rendimento, divisor, observacao)
         VALUES (?, ?, ?, ?, ?, ?) RETURNING id",
        body.numero,
        body.data,
        body.tamanho,
        body.rendimento,
        body.divisor,
        body.observacao
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(json!({
        "id": result.id,
        "numero": body.numero,
        "data": body.data,
        "tamanho": body.tamanho,
        "rendimento": body.rendimento,
        "divisor": body.divisor,
        "total_pecas": 0,
        "observacao": body.observacao
    })))
}

async fn obter_corte(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Value>, AppError> {
    let corte = sqlx::query!(
        "SELECT id, numero, data, tamanho, rendimento, divisor, total_pecas, observacao, criado_em
         FROM cortes WHERE id = ?",
        id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NaoEncontrado)?;

    Ok(Json(json!({
        "id": corte.id,
        "numero": corte.numero,
        "data": corte.data,
        "tamanho": corte.tamanho,
        "rendimento": corte.rendimento,
        "divisor": corte.divisor,
        "total_pecas": corte.total_pecas,
        "observacao": corte.observacao,
        "criado_em": corte.criado_em
    })))
}

async fn listar_linhas(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Value>, AppError> {
    let linhas = sqlx::query!(
        "SELECT id, corte_id, num, n_folhas, a_mais, cor, total, soma_a_g, total_geral
         FROM linhas_corte WHERE corte_id = ? ORDER BY num",
        id
    )
    .fetch_all(&pool)
    .await?;

    let lista: Vec<_> = linhas.iter().map(|l| json!({
        "id": l.id,
        "corte_id": l.corte_id,
        "num": l.num,
        "n_folhas": l.n_folhas,
        "a_mais": l.a_mais,
        "cor": l.cor,
        "total": l.total,
        "soma_a_g": l.soma_a_g,
        "total_geral": l.total_geral
    })).collect();

    Ok(Json(json!(lista)))
}

async fn adicionar_linha(
    Extension(claims): Extension<Claims>,
    State(pool): State<SqlitePool>,
    Path(corte_id): Path<i64>,
    Json(body): Json<CriarLinhaCorte>,
) -> Result<Json<Value>, AppError> {
    if claims.cargo != "admin" {
        return Err(AppError::Proibido);
    }

    let corte = sqlx::query!(
        "SELECT rendimento, divisor FROM cortes WHERE id = ?",
        corte_id
    )
    .fetch_optional(&pool)
    .await?
    .ok_or(AppError::NaoEncontrado)?;

    let (total, soma_a_g, total_geral) = calcular_linha(
        body.n_folhas,
        body.a_mais,
        corte.rendimento,
        corte.divisor,
    );

    let max_num = sqlx::query_scalar!(
        "SELECT COALESCE(MAX(num), 0) FROM linhas_corte WHERE corte_id = ?",
        corte_id
    )
    .fetch_one(&pool)
    .await?;

    let num = max_num + 1;

    let result = sqlx::query!(
        "INSERT INTO linhas_corte (corte_id, num, n_folhas, a_mais, cor, total, soma_a_g, total_geral)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?) RETURNING id",
        corte_id,
        num,
        body.n_folhas,
        body.a_mais,
        body.cor,
        total,
        soma_a_g,
        total_geral
    )
    .fetch_one(&pool)
    .await?;

    sqlx::query!(
        "UPDATE cortes SET total_pecas = (
            SELECT COALESCE(SUM(total_geral), 0) FROM linhas_corte WHERE corte_id = ?
         ) WHERE id = ?",
        corte_id,
        corte_id
    )
    .execute(&pool)
    .await?;

    Ok(Json(json!({
        "id": result.id,
        "corte_id": corte_id,
        "num": num,
        "n_folhas": body.n_folhas,
        "a_mais": body.a_mais,
        "cor": body.cor,
        "total": total,
        "soma_a_g": soma_a_g,
        "total_geral": total_geral
    })))
}

async fn deletar_linha(
    Extension(claims): Extension<Claims>,
    State(pool): State<SqlitePool>,
    Path((corte_id, linha_id)): Path<(i64, i64)>,
) -> Result<Json<Value>, AppError> {
    if claims.cargo != "admin" {
        return Err(AppError::Proibido);
    }

    sqlx::query!(
        "DELETE FROM linhas_corte WHERE id = ? AND corte_id = ?",
        linha_id,
        corte_id
    )
    .execute(&pool)
    .await?;

    sqlx::query!(
        "UPDATE cortes SET total_pecas = (
            SELECT COALESCE(SUM(total_geral), 0) FROM linhas_corte WHERE corte_id = ?
         ) WHERE id = ?",
        corte_id,
        corte_id
    )
    .execute(&pool)
    .await?;

    Ok(Json(json!({ "mensagem": "Linha removida" })))
}

pub fn rotas(pool: SqlitePool) -> Router {
    Router::new()
        .route("/cortes", get(listar_cortes).post(criar_corte))
        .route("/cortes/:id", get(obter_corte))
        .route("/cortes/:id/linhas", get(listar_linhas).post(adicionar_linha))
        .route("/cortes/:id/linhas/:linha_id", delete(deletar_linha))
        .with_state(pool)
}
