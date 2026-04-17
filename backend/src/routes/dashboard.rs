use axum::{
    extract::{State, Path, Query},
    routing::get,
    Router, Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::SqlitePool;
use crate::errors::AppError;

async fn resumo_geral(
    State(pool): State<SqlitePool>,
) -> Result<Json<Value>, AppError> {
    let hoje = chrono::Utc::now().format("%Y-%m-%d").to_string();

    let total_cortado = sqlx::query_scalar!(
        "SELECT COALESCE(SUM(total_pecas), 0) FROM cortes"
    )
    .fetch_one(&pool)
    .await? as i64;

    let total_produzido = sqlx::query_scalar!(
        "SELECT COALESCE(SUM(quantidade), 0) FROM producao"
    )
    .fetch_one(&pool)
    .await? as i64;

    let funcionarias_ativas = sqlx::query_scalar!(
        "SELECT COUNT(DISTINCT funcionaria_id) FROM producao WHERE date(data_hora) = ?",
        hoje
    )
    .fetch_one(&pool)
    .await? as i64;

    let percentual = if total_cortado > 0 {
        (total_produzido as f64 / total_cortado as f64 * 100.0).round()
    } else { 0.0 };

    let prod_por_func = sqlx::query!(
        "SELECT u.nome, COALESCE(SUM(p.quantidade), 0) as total
         FROM usuarios u
         LEFT JOIN producao p ON p.funcionaria_id = u.id
         WHERE u.cargo = 'funcionaria' AND u.ativo = 1
         GROUP BY u.id, u.nome
         ORDER BY total DESC"
    )
    .fetch_all(&pool)
    .await?;

    let prod_por_op = sqlx::query!(
        "SELECT operacao, SUM(quantidade) as total
         FROM producao
         GROUP BY operacao
         ORDER BY total DESC"
    )
    .fetch_all(&pool)
    .await?;

    let cortes_abertos = sqlx::query!(
        "SELECT c.id, c.numero, c.tamanho, c.total_pecas,
         COALESCE(SUM(p.quantidade), 0) as produzido
         FROM cortes c
         LEFT JOIN producao p ON p.corte_id = c.id
         GROUP BY c.id
         ORDER BY c.numero DESC
         LIMIT 10"
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(json!({
        "total_pecas_cortadas": total_cortado,
        "total_produzido": total_produzido,
        "percentual_conclusao": percentual,
        "funcionarias_ativas": funcionarias_ativas,
        "producao_por_funcionaria": prod_por_func.iter().map(|r| json!({
            "nome": r.nome,
            "total": r.total
        })).collect::<Vec<_>>(),
        "producao_por_operacao": prod_por_op.iter().map(|r| json!({
            "operacao": r.operacao,
            "total": r.total
        })).collect::<Vec<_>>(),
        "cortes_abertos": cortes_abertos.iter().map(|c| json!({
            "id": c.id,
            "numero": c.numero,
            "tamanho": c.tamanho,
            "total_pecas": c.total_pecas,
            "produzido": c.produzido,
            "percentual": if c.total_pecas > 0 {
                (c.produzido as f64 / c.total_pecas as f64 * 100.0).round()
            } else { 0.0 }
        })).collect::<Vec<_>>()
    })))
}

async fn resumo_corte(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Value>, AppError> {
    let linhas = sqlx::query!(
        r#"SELECT lc.cor, SUM(lc.total_geral) as total_geral,
           COALESCE(SUM(p.quantidade), 0) as produzido
           FROM linhas_corte lc
           LEFT JOIN producao p ON p.linha_corte_id = lc.id
           WHERE lc.corte_id = ?
           GROUP BY lc.cor
           ORDER BY lc.cor"#,
        id
    )
    .fetch_all(&pool)
    .await?;

    let resumo: Vec<_> = linhas.iter().map(|l| {
        let total_geral = l.total_geral;
        let produzido = l.produzido as i64;
        let percentual = if total_geral > 0 {
            (produzido as f64 / total_geral as f64 * 100.0).round()
        } else { 0.0 };

        json!({
            "cor": l.cor,
            "total_geral": total_geral,
            "produzido": produzido,
            "pendente": total_geral - produzido,
            "percentual": percentual
        })
    }).collect();

    Ok(Json(json!(resumo)))
}

async fn producao_funcionaria(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>,
) -> Result<Json<Value>, AppError> {
    let producoes = sqlx::query!(
        r#"SELECT p.operacao, SUM(p.quantidade) as total, date(p.data_hora) as dia
           FROM producao p
           WHERE p.funcionaria_id = ?
           GROUP BY p.operacao, date(p.data_hora)
           ORDER BY dia DESC"#,
        id
    )
    .fetch_all(&pool)
    .await?;

    let lista: Vec<_> = producoes.iter().map(|p| json!({
        "operacao": p.operacao,
        "total": p.total,
        "dia": p.dia
    })).collect();

    Ok(Json(json!(lista)))
}

#[derive(Deserialize)]
struct FiltroRelatorio {
    inicio: Option<String>,
    fim: Option<String>,
}

async fn relatorio(
    State(pool): State<SqlitePool>,
    Query(filtro): Query<FiltroRelatorio>,
) -> Result<Json<Value>, AppError> {
    let inicio = filtro.inicio.unwrap_or_else(|| "2000-01-01".to_string());
    let fim = filtro.fim.unwrap_or_else(|| "2099-12-31".to_string());

    let dados = sqlx::query!(
        r#"SELECT p.id, u.nome as funcionaria_nome, c.numero as corte_numero,
           lc.cor, p.operacao, p.quantidade, p.data_hora
           FROM producao p
           JOIN usuarios u ON u.id = p.funcionaria_id
           JOIN cortes c ON c.id = p.corte_id
           JOIN linhas_corte lc ON lc.id = p.linha_corte_id
           WHERE date(p.data_hora) BETWEEN ? AND ?
           ORDER BY p.data_hora DESC"#,
        inicio,
        fim
    )
    .fetch_all(&pool)
    .await?;

    let lista: Vec<_> = dados.iter().map(|d| json!({
        "id": d.id,
        "funcionaria_nome": d.funcionaria_nome,
        "corte_numero": d.corte_numero,
        "cor": d.cor,
        "operacao": d.operacao,
        "quantidade": d.quantidade,
        "data_hora": d.data_hora
    })).collect();

    Ok(Json(json!(lista)))
}

pub fn rotas(pool: SqlitePool) -> Router {
    Router::new()
        .route("/dashboard/resumo", get(resumo_geral))
        .route("/dashboard/corte/:id", get(resumo_corte))
        .route("/dashboard/funcionaria/:id", get(producao_funcionaria))
        .route("/dashboard/relatorio", get(relatorio))
        .with_state(pool)
}
