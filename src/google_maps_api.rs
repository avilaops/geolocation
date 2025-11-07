use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::{
    database::Database,
    google_maps::{
        DistanceMatrixRequest, GoogleMapsClient, Location, NearbySearchRequest, TextSearchRequest,
        TravelMode,
    },
};

/// Estado compartilhado da aplicação com Google Maps
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<Database>,
    pub maps_client: Option<Arc<GoogleMapsClient>>,
}

/// Rotas da API Google Maps
pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/geocode", post(geocode_address))
        .route("/reverse-geocode", post(reverse_geocode))
        .route("/nearby", post(search_nearby))
        .route("/search", post(text_search))
        .route("/place/:place_id", get(place_details))
        .route("/distance", post(calculate_distance))
        .route("/companies/city/:city/:state", get(companies_in_city))
        .route("/companies/enrich", post(enrich_company_data))
        .route("/companies/location/:cnpj", get(get_company_location))
        .with_state(state)
}

#[derive(Debug, Deserialize)]
struct GeocodeRequest {
    address: String,
}

#[derive(Debug, Serialize)]
struct GeocodeResponse {
    results: Vec<serde_json::Value>,
}

async fn geocode_address(
    State(state): State<AppState>,
    Json(req): Json<GeocodeRequest>,
) -> Result<Json<GeocodeResponse>, StatusCode> {
    let client = state
        .maps_client
        .as_ref()
        .ok_or(StatusCode::SERVICE_UNAVAILABLE)?;

    let results = client.geocode(&req.address).await.map_err(|e| {
        tracing::error!("Erro ao geocodificar: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(GeocodeResponse {
        results: results
            .iter()
            .map(|r| serde_json::to_value(r).unwrap())
            .collect(),
    }))
}

#[derive(Debug, Deserialize)]
struct ReverseGeocodeRequest {
    lat: f64,
    lng: f64,
}

async fn reverse_geocode(
    State(state): State<AppState>,
    Json(req): Json<ReverseGeocodeRequest>,
) -> Result<Json<GeocodeResponse>, StatusCode> {
    let client = state
        .maps_client
        .as_ref()
        .ok_or(StatusCode::SERVICE_UNAVAILABLE)?;

    let results = client
        .reverse_geocode(req.lat, req.lng)
        .await
        .map_err(|e| {
            tracing::error!("Erro no reverse geocoding: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(Json(GeocodeResponse {
        results: results
            .iter()
            .map(|r| serde_json::to_value(r).unwrap())
            .collect(),
    }))
}

#[derive(Debug, Deserialize)]
struct NearbyRequest {
    lat: f64,
    lng: f64,
    radius: u32,
    place_type: Option<String>,
    keyword: Option<String>,
}

async fn search_nearby(
    State(state): State<AppState>,
    Json(req): Json<NearbyRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let client = state
        .maps_client
        .as_ref()
        .ok_or(StatusCode::SERVICE_UNAVAILABLE)?;

    let request = NearbySearchRequest {
        location: Location {
            lat: req.lat,
            lng: req.lng,
        },
        radius: req.radius,
        place_type: req.place_type,
        keyword: req.keyword,
    };

    let result = client.nearby_search(request).await.map_err(|e| {
        tracing::error!("Erro na busca nearby: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(serde_json::to_value(result).unwrap()))
}

#[derive(Debug, Deserialize)]
struct TextSearchQuery {
    query: String,
    lat: Option<f64>,
    lng: Option<f64>,
    radius: Option<u32>,
}

async fn text_search(
    State(state): State<AppState>,
    Json(req): Json<TextSearchQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let client = state
        .maps_client
        .as_ref()
        .ok_or(StatusCode::SERVICE_UNAVAILABLE)?;

    let location = if let (Some(lat), Some(lng)) = (req.lat, req.lng) {
        Some(Location { lat, lng })
    } else {
        None
    };

    let request = TextSearchRequest {
        query: req.query,
        location,
        radius: req.radius,
    };

    let result = client.text_search(request).await.map_err(|e| {
        tracing::error!("Erro na busca textual: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(serde_json::to_value(result).unwrap()))
}

async fn place_details(
    State(state): State<AppState>,
    Path(place_id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let client = state
        .maps_client
        .as_ref()
        .ok_or(StatusCode::SERVICE_UNAVAILABLE)?;

    let details = client.place_details(&place_id).await.map_err(|e| {
        tracing::error!("Erro ao obter detalhes do place: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(serde_json::to_value(details).unwrap()))
}

#[derive(Debug, Deserialize)]
struct DistanceRequest {
    origins: Vec<LocationDto>,
    destinations: Vec<LocationDto>,
    mode: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct LocationDto {
    lat: f64,
    lng: f64,
}

async fn calculate_distance(
    State(state): State<AppState>,
    Json(req): Json<DistanceRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let client = state
        .maps_client
        .as_ref()
        .ok_or(StatusCode::SERVICE_UNAVAILABLE)?;

    let mode = match req.mode.as_deref() {
        Some("walking") => TravelMode::Walking,
        Some("bicycling") => TravelMode::Bicycling,
        Some("transit") => TravelMode::Transit,
        _ => TravelMode::Driving,
    };

    let request = DistanceMatrixRequest {
        origins: req
            .origins
            .iter()
            .map(|l| Location {
                lat: l.lat,
                lng: l.lng,
            })
            .collect(),
        destinations: req
            .destinations
            .iter()
            .map(|l| Location {
                lat: l.lat,
                lng: l.lng,
            })
            .collect(),
        mode,
    };

    let result = client.distance_matrix(request).await.map_err(|e| {
        tracing::error!("Erro ao calcular distância: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(serde_json::to_value(result).unwrap()))
}

#[derive(Debug, Deserialize)]
struct CompanyCityQuery {
    business_type: Option<String>,
    limit: Option<u32>,
}

async fn companies_in_city(
    State(state): State<AppState>,
    Path((city, state_uf)): Path<(String, String)>,
    Query(query): Query<CompanyCityQuery>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let client = state
        .maps_client
        .as_ref()
        .ok_or(StatusCode::SERVICE_UNAVAILABLE)?;

    let companies = client
        .search_companies_in_city(&city, &state_uf, query.business_type.as_deref())
        .await
        .map_err(|e| {
            tracing::error!("Erro ao buscar empresas: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let limit = query.limit.unwrap_or(20) as usize;
    let limited = companies.into_iter().take(limit).collect::<Vec<_>>();

    Ok(Json(serde_json::to_value(limited).unwrap()))
}

#[derive(Debug, Deserialize)]
struct EnrichCompanyRequest {
    cnpj: String,
    razao_social: String,
    logradouro: String,
    numero: String,
    bairro: Option<String>,
    municipio: String,
    uf: String,
    cep: Option<String>,
}

#[derive(Debug, Serialize)]
struct EnrichCompanyResponse {
    success: bool,
    location: Option<serde_json::Value>,
    place_details: Option<serde_json::Value>,
    error: Option<String>,
}

async fn enrich_company_data(
    State(state): State<AppState>,
    Json(req): Json<EnrichCompanyRequest>,
) -> Result<Json<EnrichCompanyResponse>, StatusCode> {
    let client = state
        .maps_client
        .as_ref()
        .ok_or(StatusCode::SERVICE_UNAVAILABLE)?;

    // Geocodificar endereço
    let geocode_result = client
        .geocode_company(
            &req.logradouro,
            &req.numero,
            req.bairro.as_deref(),
            &req.municipio,
            &req.uf,
            req.cep.as_deref(),
        )
        .await;

    match geocode_result {
        Ok(location) => {
            // Tentar buscar detalhes do place
            let place_details = if !location.place_id.is_empty() {
                client.place_details(&location.place_id).await.ok()
            } else {
                None
            };

            // TODO: Salvar no banco de dados (company_locations)
            tracing::info!("Empresa {} enriquecida com sucesso", req.cnpj);

            Ok(Json(EnrichCompanyResponse {
                success: true,
                location: Some(serde_json::to_value(&location).unwrap()),
                place_details: place_details.map(|d| serde_json::to_value(d).unwrap()),
                error: None,
            }))
        }
        Err(e) => {
            tracing::warn!("Falha ao enriquecer empresa {}: {}", req.cnpj, e);
            Ok(Json(EnrichCompanyResponse {
                success: false,
                location: None,
                place_details: None,
                error: Some(e.to_string()),
            }))
        }
    }
}

async fn get_company_location(
    State(state): State<AppState>,
    Path(cnpj): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    // TODO: Buscar do banco de dados (company_locations table)
    tracing::info!("Buscando localização da empresa {}", cnpj);

    Ok(Json(serde_json::json!({
        "message": "Funcionalidade em implementação",
        "cnpj": cnpj
    })))
}
