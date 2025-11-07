use crate::google_maps::{
    DayTime, GoogleMapsClient, Location, NearbySearchRequest, NearbySearchResult, OpeningHours,
    Period, Photo, PlaceDetails, PlaceSummary, Review, TextSearchRequest,
};
use anyhow::Result;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct PlacesResponse {
    results: Vec<PlaceResult>,
    status: String,
    next_page_token: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PlaceResult {
    place_id: String,
    name: String,
    vicinity: Option<String>,
    formatted_address: Option<String>,
    geometry: PlaceGeometry,
    types: Vec<String>,
    rating: Option<f32>,
    user_ratings_total: Option<u32>,
    business_status: Option<String>,
}

#[derive(Debug, Deserialize)]
struct PlaceGeometry {
    location: PlaceLocation,
}

#[derive(Debug, Deserialize)]
struct PlaceLocation {
    lat: f64,
    lng: f64,
}

#[derive(Debug, Deserialize)]
struct PlaceDetailsResponse {
    result: PlaceDetailsResult,
    status: String,
}

#[derive(Debug, Deserialize)]
struct PlaceDetailsResult {
    place_id: String,
    name: String,
    formatted_address: String,
    geometry: PlaceGeometry,
    types: Vec<String>,
    business_status: Option<String>,
    formatted_phone_number: Option<String>,
    international_phone_number: Option<String>,
    website: Option<String>,
    rating: Option<f32>,
    user_ratings_total: Option<u32>,
    opening_hours: Option<ApiOpeningHours>,
    price_level: Option<u8>,
    reviews: Option<Vec<ApiReview>>,
    photos: Option<Vec<ApiPhoto>>,
}

#[derive(Debug, Deserialize)]
struct ApiOpeningHours {
    open_now: Option<bool>,
    weekday_text: Option<Vec<String>>,
    periods: Option<Vec<ApiPeriod>>,
}

#[derive(Debug, Deserialize)]
struct ApiPeriod {
    open: ApiDayTime,
    close: Option<ApiDayTime>,
}

#[derive(Debug, Deserialize)]
struct ApiDayTime {
    day: u8,
    time: String,
}

#[derive(Debug, Deserialize)]
struct ApiReview {
    author_name: String,
    rating: u8,
    text: String,
    time: i64,
}

#[derive(Debug, Deserialize)]
struct ApiPhoto {
    photo_reference: String,
    width: u32,
    height: u32,
}

impl GoogleMapsClient {
    /// Busca estabelecimentos próximos a uma localização
    pub async fn nearby_search(&self, request: NearbySearchRequest) -> Result<NearbySearchResult> {
        let location_str = format!("{},{}", request.location.lat, request.location.lng);
        let radius_str = request.radius.to_string();

        let mut params = vec![
            ("location", location_str.as_str()),
            ("radius", radius_str.as_str()),
        ];

        if let Some(ref place_type) = request.place_type {
            params.push(("type", place_type.as_str()));
        }

        if let Some(ref keyword) = request.keyword {
            params.push(("keyword", keyword.as_str()));
        }

        let url = self.build_url("place/nearbysearch/json", &params);
        let response: PlacesResponse = self.get_json(&url).await?;

        if response.status != "OK" && response.status != "ZERO_RESULTS" {
            anyhow::bail!("Nearby search falhou: {}", response.status);
        }

        let places = response
            .results
            .into_iter()
            .map(|r| PlaceSummary {
                place_id: r.place_id,
                name: r.name,
                vicinity: r.vicinity.unwrap_or_default(),
                location: Location {
                    lat: r.geometry.location.lat,
                    lng: r.geometry.location.lng,
                },
                types: r.types,
                rating: r.rating,
                user_ratings_total: r.user_ratings_total,
            })
            .collect();

        Ok(NearbySearchResult {
            places,
            next_page_token: response.next_page_token,
        })
    }

    /// Busca textual de estabelecimentos
    pub async fn text_search(&self, request: TextSearchRequest) -> Result<NearbySearchResult> {
        let mut params = vec![("query", request.query.as_str())];

        let location_str;
        if let Some(ref loc) = request.location {
            location_str = format!("{},{}", loc.lat, loc.lng);
            params.push(("location", &location_str));
        }

        let radius_str;
        if let Some(radius) = request.radius {
            radius_str = radius.to_string();
            params.push(("radius", &radius_str));
        }

        let url = self.build_url("place/textsearch/json", &params);
        let response: PlacesResponse = self.get_json(&url).await?;

        if response.status != "OK" && response.status != "ZERO_RESULTS" {
            anyhow::bail!("Text search falhou: {}", response.status);
        }

        let places = response
            .results
            .into_iter()
            .map(|r| PlaceSummary {
                place_id: r.place_id,
                name: r.name,
                vicinity: r.vicinity.unwrap_or_default(),
                location: Location {
                    lat: r.geometry.location.lat,
                    lng: r.geometry.location.lng,
                },
                types: r.types,
                rating: r.rating,
                user_ratings_total: r.user_ratings_total,
            })
            .collect();

        Ok(NearbySearchResult {
            places,
            next_page_token: response.next_page_token,
        })
    }

    /// Obtém detalhes completos de um estabelecimento
    pub async fn place_details(&self, place_id: &str) -> Result<PlaceDetails> {
        let url = self.build_url("place/details/json", &[
            ("place_id", place_id),
            ("fields", "place_id,name,formatted_address,geometry,types,business_status,formatted_phone_number,international_phone_number,website,rating,user_ratings_total,opening_hours,price_level,reviews,photos"),
        ]);

        let response: PlaceDetailsResponse = self.get_json(&url).await?;

        if response.status != "OK" {
            anyhow::bail!("Place details falhou: {}", response.status);
        }

        let r = response.result;

        Ok(PlaceDetails {
            place_id: r.place_id,
            name: r.name,
            formatted_address: r.formatted_address,
            location: Location {
                lat: r.geometry.location.lat,
                lng: r.geometry.location.lng,
            },
            types: r.types,
            business_status: r.business_status,
            phone_number: r.formatted_phone_number.or(r.international_phone_number),
            website: r.website,
            rating: r.rating,
            user_ratings_total: r.user_ratings_total,
            opening_hours: r.opening_hours.and_then(|oh| {
                Some(OpeningHours {
                    open_now: oh.open_now.unwrap_or(false),
                    weekday_text: oh.weekday_text.unwrap_or_default(),
                    periods: oh
                        .periods
                        .map(|periods| {
                            periods
                                .into_iter()
                                .map(|p| Period {
                                    open: DayTime {
                                        day: p.open.day,
                                        time: p.open.time,
                                    },
                                    close: p.close.map(|c| DayTime {
                                        day: c.day,
                                        time: c.time,
                                    }),
                                })
                                .collect()
                        })
                        .unwrap_or_default(),
                })
            }),
            price_level: r.price_level,
            reviews: r
                .reviews
                .map(|reviews| {
                    reviews
                        .into_iter()
                        .map(|r| Review {
                            author_name: r.author_name,
                            rating: r.rating,
                            text: r.text,
                            time: r.time,
                        })
                        .collect()
                })
                .unwrap_or_default(),
            photos: r
                .photos
                .map(|photos| {
                    photos
                        .into_iter()
                        .map(|p| Photo {
                            photo_reference: p.photo_reference,
                            width: p.width,
                            height: p.height,
                        })
                        .collect()
                })
                .unwrap_or_default(),
        })
    }

    /// Busca empresas em uma cidade específica
    pub async fn search_companies_in_city(
        &self,
        city: &str,
        state: &str,
        business_type: Option<&str>,
    ) -> Result<Vec<PlaceSummary>> {
        let mut query = format!("empresas em {}, {}", city, state);

        if let Some(biz_type) = business_type {
            query = format!("{} {}", biz_type, query);
        }

        let request = TextSearchRequest {
            query,
            location: None,
            radius: None,
        };

        let result = self.text_search(request).await?;
        Ok(result.places)
    }

    /// URL da foto de um estabelecimento
    pub fn photo_url(&self, photo_reference: &str, max_width: u32) -> String {
        format!(
            "{}/place/photo?key={}&photoreference={}&maxwidth={}",
            self.base_url(),
            self.api_key(),
            photo_reference,
            max_width
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_nearby_search() {
        let api_key = std::env::var("GOOGLE_MAPS_API_KEY").unwrap();
        let client = GoogleMapsClient::new(api_key).unwrap();

        let request = NearbySearchRequest {
            location: Location {
                lat: -23.561684,
                lng: -46.655981,
            },
            radius: 1000,
            place_type: Some("restaurant".to_string()),
            keyword: None,
        };

        let result = client.nearby_search(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_text_search() {
        let api_key = std::env::var("GOOGLE_MAPS_API_KEY").unwrap();
        let client = GoogleMapsClient::new(api_key).unwrap();

        let request = TextSearchRequest {
            query: "padarias em São Paulo".to_string(),
            location: None,
            radius: None,
        };

        let result = client.text_search(request).await;
        assert!(result.is_ok());
    }
}
