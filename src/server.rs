use axum::{
    extract::{Multipart, Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use geolocation::database::repository::DocumentSummary;
use geolocation::database::mongodb::MongoDB;
use geolocation::utils::metrics::{gather_metrics, register_metrics};
use geolocation::{process_document_content, DocumentType};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use tracing::{info, warn};
use tracing_subscriber::prelude::*;

mod search_api;

#[derive(Clone)]
pub struct AppState {
    mongo: Arc<MongoDB>,
    maps_client: Option<Arc<geolocation::google_maps::GoogleMapsClient>>,
}

#[derive(Serialize, Deserialize)]
struct UploadResponse {
    success: bool,
    document_type: String,
    chave_acesso: String,
    message: String,
    duplicate: bool,
}

#[derive(Serialize, Deserialize)]
struct StatsResponse {
    total_documents: i64,
    processed_today: i64,
    notas_fiscais: i64,
    ctes: i64,
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}

#[tokio::main]
async fn main() {
    // Inicializa tracing estruturado (substitui env_logger)
    use tracing_subscriber::{fmt, EnvFilter, Registry};
    Registry::default()
        .with(EnvFilter::from_default_env().add_directive("info".parse().unwrap()))
        .with(fmt::layer().with_target(false).compact())
        .init();

    // Conecta ao MongoDB
    let mongo_uri = std::env::var("MONGODB_URI").expect("MONGODB_URI deve ser definido");
    let mongo = MongoDB::connect(&mongo_uri)
        .await
        .expect("Falha ao conectar ao MongoDB");
    
    info!("✓ MongoDB conectado");
    
    // Cria índices
    if let Err(e) = mongo.setup_indexes().await {
        warn!("Erro ao criar índices MongoDB: {}", e);
    } else {
        info!("✓ Índices MongoDB criados");
    }

    // Inicializa Google Maps client (opcional)
    let maps_client = std::env::var("GOOGLE_MAPS_API_KEY").ok().and_then(|key| {
        if key.is_empty() {
            None
        } else {
            match geolocation::google_maps::GoogleMapsClient::new(key) {
                Ok(client) => {
                    info!("✓ Google Maps API inicializada");
                    Some(Arc::new(client))
                }
                Err(e) => {
                    warn!("Google Maps API não disponível: {}", e);
                    None
                }
            }
        }
    });

    if maps_client.is_none() {
        info!("⚠ Google Maps API desabilitada (defina GOOGLE_MAPS_API_KEY para habilitar)");
    }

    let state = AppState {
        mongo: Arc::new(mongo),
        maps_client,
    };

    // Configura CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // TODO: Configura rotas Google Maps (se habilitado) - DESABILITADO TEMPORARIAMENTE
    // let maps_routes = if state.maps_client.is_some() {
    //     let maps_state = crate::google_maps_api::AppState {
    //         db: state.db.clone(),
    //         maps_client: state.maps_client.clone(),
    //     };
    //     crate::google_maps_api::routes(maps_state)
    // } else {
    //     Router::new()
    // };
    let maps_routes = Router::new(); // Vazio por enquanto

    // Configura rotas de pesquisa (se MongoDB habilitado)
    let search_routes = search_api::routes();

    // Configura rotas
    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/documents/upload", post(upload_document))
        .route("/api/documents/stats", get(get_stats))
        .route("/api/documents/:chave", get(get_document_by_chave))
        .route("/api/documents", get(list_documents))
        .route("/metrics", get(metrics_handler))
        .nest("/api/maps", maps_routes)
        .nest("/api", search_routes)
        .nest_service("/", ServeDir::new("frontend/dist"))
        .layer(cors)
        .with_state(state);

    // Inicia o servidor
    let addr = "0.0.0.0:3000";
    info!(%addr, "Servidor rodando");
    info!("Frontend em http://localhost:3000");
    info!("API em http://localhost:3000/api");

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Falha ao bind no endereço");

    axum::serve(listener, app)
        .await
        .expect("Falha ao iniciar servidor");
}

async fn metrics_handler() -> Result<String, (StatusCode, Json<ErrorResponse>)> {
    register_metrics();
    match gather_metrics() {
        Ok(s) => Ok(s),
        Err(e) => Err(internal_error(e)),
    }
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "geolocation-api",
        "version": geolocation::VERSION
    }))
}

async fn upload_document(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<UploadResponse>, (StatusCode, Json<ErrorResponse>)> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| internal_error(e.to_string()))?
    {
        let name = field.name().unwrap_or("").to_string();

        if name == "file" {
            let data = field
                .bytes()
                .await
                .map_err(|e| internal_error(e.to_string()))?;

            let xml_content = String::from_utf8(data.to_vec())
                .map_err(|e| internal_error(format!("Arquivo não é UTF-8 válido: {}", e)))?;

            match process_document_content(&xml_content, state.mongo.as_ref()).await {
                Ok(result) => {
                    let document_type = match result.document_type {
                        DocumentType::NotaFiscal => "NFe".to_string(),
                        DocumentType::ConhecimentoTransporte => "CTe".to_string(),
                    };
                    return Ok(Json(UploadResponse {
                        success: result.success,
                        document_type,
                        chave_acesso: result.chave_acesso,
                        message: result.message,
                        duplicate: result.duplicate,
                    }));
                }
                Err(e) => {
                    warn!(error = %e, "Erro ao processar documento");
                    return Err(internal_error(format!(
                        "Erro ao processar documento: {}",
                        e
                    )));
                }
            }
        }
    }

    Err(internal_error("Nenhum arquivo foi enviado".to_string()))
}

async fn get_stats(
    State(state): State<AppState>,
) -> Result<Json<StatsResponse>, (StatusCode, Json<ErrorResponse>)> {
    let total_nfes = state.mongo.count_documents(Some("NFe")).await
        .map_err(|e| internal_error(format!("Erro ao contar NFes: {}", e)))?;
    let total_ctes = state.mongo.count_documents(Some("CTe")).await
        .map_err(|e| internal_error(format!("Erro ao contar CTes: {}", e)))?;
    
    let total_documents = total_nfes + total_ctes;
    
    // TODO: Implementar contagem de documentos processados hoje
    let processed_today = 0i64;

    Ok(Json(StatsResponse {
        total_documents,
        processed_today,
        notas_fiscais: total_nfes,
        ctes: total_ctes,
    }))
}

async fn list_documents(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<DocumentSummary>>, (StatusCode, Json<ErrorResponse>)> {
    let limit = query.limit.unwrap_or(50);
    let limit = if limit < 1 {
        1
    } else if limit > 500 {
        500
    } else {
        limit
    };
    let offset = query.offset.unwrap_or(0);
    match state.mongo
        .list_documents(query.doc_type.as_deref(), limit, offset)
        .await
    {
        Ok(list) => Ok(Json(list)),
        Err(e) => Err(internal_error(format!("Erro ao listar documentos: {}", e))),
    }
}

#[derive(Deserialize)]
struct ListQuery {
    doc_type: Option<String>,
    limit: Option<i64>,
    offset: Option<i64>,
}

/// Retorna detalhes de um documento (NF-e ou CT-e) incluindo última validação
async fn get_document_by_chave(
    State(state): State<AppState>,
    Path(chave): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    // Buscar primeiro em NFes
    if let Ok(Some(nfe)) = state.mongo.find_nota_fiscal_by_chave(&chave).await {
        let validation = state.mongo.find_validation_by_chave(&chave).await.ok().flatten();
        let json = serde_json::json!({
            "document_type": "NFe",
            "chave_acesso": nfe.chave_acesso,
            "numero": nfe.numero,
            "serie": nfe.serie,
            "data_emissao": nfe.data_emissao.to_rfc3339(),
            "emitente": nfe.emitente.razao_social,
            "destinatario": nfe.destinatario.razao_social,
            "valor_total": nfe.totais.valor_total,
            "validation": validation
        });
        return Ok(Json(json));
    }
    
    // Buscar em CTes
    if let Ok(Some(cte)) = state.mongo.find_conhecimento_by_chave(&chave).await {
        let validation = state.mongo.find_validation_by_chave(&chave).await.ok().flatten();
        let json = serde_json::json!({
            "document_type": "CTe",
            "chave_acesso": cte.chave_acesso,
            "numero": cte.numero,
            "serie": cte.serie,
            "data_emissao": cte.data_emissao.to_rfc3339(),
            "emitente": cte.emitente.razao_social,
            "destinatario": cte.destinatario.razao_social,
            "valor_total": cte.valores_prestacao.valor_total,
            "validation": validation
        });
        return Ok(Json(json));
    }
    
    // Documento não encontrado
    Err((
        StatusCode::NOT_FOUND,
        Json(ErrorResponse {
            error: "Documento não encontrado".to_string(),
        }),
    ))
}

fn internal_error(message: String) -> (StatusCode, Json<ErrorResponse>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(ErrorResponse { error: message }),
    )
}
