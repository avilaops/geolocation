use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use mongodb::bson::DateTime as BsonDateTime;
use serde::{Deserialize, Serialize};

use crate::AppState;
use geolocation::database::mongodb::SearchRecord;

/// Rotas de histórico de pesquisas
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/searches", get(list_searches))
        .route("/searches/stats", get(get_search_stats))
}

#[derive(Debug, Deserialize)]
struct SearchQuery {
    search_type: Option<String>,
    user_id: Option<String>,
    #[serde(default = "default_limit")]
    limit: i64,
    #[serde(default)]
    skip: u64,
}

fn default_limit() -> i64 {
    50
}

#[derive(Debug, Serialize)]
struct SearchListResponse {
    searches: Vec<SearchRecord>,
    total: usize,
    limit: i64,
    skip: u64,
}

/// Lista histórico de pesquisas com paginação
async fn list_searches(
    State(state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<SearchListResponse>, (StatusCode, String)> {
    let searches = state
        .mongo
        .list_searches(
            params.search_type.as_deref(),
            params.user_id.as_deref(),
            params.limit,
            params.skip,
        )
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Erro ao listar pesquisas: {}", e),
            )
        })?;

    let total = searches.len();

    Ok(Json(SearchListResponse {
        searches,
        total,
        limit: params.limit,
        skip: params.skip,
    }))
}

#[derive(Debug, Serialize)]
struct StatsResponse {
    total_searches: u64,
    geocoding_cache_size: u64,
    distance_cache_size: u64,
}

/// Retorna estatísticas de uso do MongoDB
async fn get_search_stats(
    State(state): State<AppState>,
) -> Result<Json<StatsResponse>, (StatusCode, String)> {
    let stats = state.mongo.get_stats().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro ao obter estatísticas: {}", e),
        )
    })?;

    Ok(Json(StatsResponse {
        total_searches: stats.get_i64("total_searches").unwrap_or(0) as u64,
        geocoding_cache_size: stats.get_i64("geocoding_cache_size").unwrap_or(0) as u64,
        distance_cache_size: stats.get_i64("distance_cache_size").unwrap_or(0) as u64,
    }))
}

/// Helper para criar registro de pesquisa
pub fn create_search_record(
    search_type: &str,
    query: &str,
    response: serde_json::Value,
    duration_ms: Option<i64>,
    error: Option<String>,
) -> SearchRecord {
    SearchRecord {
        id: None,
        search_type: search_type.to_string(),
        query: query.to_string(),
        response,
        timestamp: BsonDateTime::now(),
        user_id: None, // TODO: Adicionar autenticação futura
        duration_ms,
        error,
    }
}
