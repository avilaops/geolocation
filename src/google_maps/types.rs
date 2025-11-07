use serde::{Deserialize, Serialize};

/// Coordenadas geográficas (latitude, longitude)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub lat: f64,
    pub lng: f64,
}

/// Endereço completo estruturado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub formatted_address: String,
    pub street_number: Option<String>,
    pub route: Option<String>,
    pub neighborhood: Option<String>,
    pub city: String,
    pub state: String,
    pub country: String,
    pub postal_code: Option<String>,
    pub location: Location,
}

/// Resultado de geocodificação
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeocodingResult {
    pub address: Address,
    pub place_id: String,
    pub types: Vec<String>,
}

/// Detalhes de um estabelecimento via Places API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceDetails {
    pub place_id: String,
    pub name: String,
    pub formatted_address: String,
    pub location: Location,
    pub types: Vec<String>,
    pub business_status: Option<String>,
    pub phone_number: Option<String>,
    pub website: Option<String>,
    pub rating: Option<f32>,
    pub user_ratings_total: Option<u32>,
    pub opening_hours: Option<OpeningHours>,
    pub price_level: Option<u8>,
    pub reviews: Vec<Review>,
    pub photos: Vec<Photo>,
}

/// Horário de funcionamento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpeningHours {
    pub open_now: bool,
    pub weekday_text: Vec<String>,
    pub periods: Vec<Period>,
}

/// Período de abertura/fechamento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Period {
    pub open: DayTime,
    pub close: Option<DayTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DayTime {
    pub day: u8,
    pub time: String,
}

/// Avaliação de usuário
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub author_name: String,
    pub rating: u8,
    pub text: String,
    pub time: i64,
}

/// Foto do estabelecimento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photo {
    pub photo_reference: String,
    pub width: u32,
    pub height: u32,
}

/// Busca de estabelecimentos próximos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearbySearchRequest {
    pub location: Location,
    pub radius: u32,
    pub place_type: Option<String>,
    pub keyword: Option<String>,
}

/// Resultado de busca nearby
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NearbySearchResult {
    pub places: Vec<PlaceSummary>,
    pub next_page_token: Option<String>,
}

/// Resumo de estabelecimento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceSummary {
    pub place_id: String,
    pub name: String,
    pub vicinity: String,
    pub location: Location,
    pub types: Vec<String>,
    pub rating: Option<f32>,
    pub user_ratings_total: Option<u32>,
}

/// Busca textual
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextSearchRequest {
    pub query: String,
    pub location: Option<Location>,
    pub radius: Option<u32>,
}

/// Cálculo de distância/tempo entre pontos
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistanceMatrixRequest {
    pub origins: Vec<Location>,
    pub destinations: Vec<Location>,
    pub mode: TravelMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TravelMode {
    Driving,
    Walking,
    Bicycling,
    Transit,
}

/// Resultado de matriz de distância
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistanceMatrixResult {
    pub rows: Vec<DistanceMatrixRow>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistanceMatrixRow {
    pub elements: Vec<DistanceMatrixElement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistanceMatrixElement {
    pub distance: Option<Distance>,
    pub duration: Option<Duration>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Distance {
    pub text: String,
    pub value: u32, // metros
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Duration {
    pub text: String,
    pub value: u32, // segundos
}

/// Autocomplete de endereços
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutocompleteRequest {
    pub input: String,
    pub location: Option<Location>,
    pub radius: Option<u32>,
    pub types: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutocompletePrediction {
    pub description: String,
    pub place_id: String,
    pub structured_formatting: StructuredFormatting,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuredFormatting {
    pub main_text: String,
    pub secondary_text: String,
}
