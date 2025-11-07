use crate::google_maps::{Address, GeocodingResult, GoogleMapsClient, Location};
use anyhow::{Context, Result};
use serde::Deserialize;

/// Resposta da API de Geocoding
#[derive(Debug, Deserialize)]
struct GeocodeResponse {
    results: Vec<GeocodeResult>,
    status: String,
}

#[derive(Debug, Deserialize)]
struct GeocodeResult {
    address_components: Vec<AddressComponent>,
    formatted_address: String,
    geometry: Geometry,
    place_id: String,
    types: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct AddressComponent {
    long_name: String,
    short_name: String,
    types: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Geometry {
    location: LatLng,
}

#[derive(Debug, Deserialize)]
struct LatLng {
    lat: f64,
    lng: f64,
}

impl GoogleMapsClient {
    /// Geocodifica um endereço (converte texto em coordenadas)
    pub async fn geocode(&self, address: &str) -> Result<Vec<GeocodingResult>> {
        let url = self.build_url("geocode/json", &[("address", address), ("region", "br")]);

        let response: GeocodeResponse = self.get_json(&url).await?;

        if response.status != "OK" {
            anyhow::bail!("Geocoding falhou: {}", response.status);
        }

        let results = response
            .results
            .into_iter()
            .map(|r| self.parse_geocode_result(r))
            .collect::<Result<Vec<_>>>()?;

        Ok(results)
    }

    /// Reverse geocoding (converte coordenadas em endereço)
    pub async fn reverse_geocode(&self, lat: f64, lng: f64) -> Result<Vec<GeocodingResult>> {
        let latlng = format!("{},{}", lat, lng);
        let url = self.build_url("geocode/json", &[("latlng", &latlng)]);

        let response: GeocodeResponse = self.get_json(&url).await?;

        if response.status != "OK" {
            anyhow::bail!("Reverse geocoding falhou: {}", response.status);
        }

        let results = response
            .results
            .into_iter()
            .map(|r| self.parse_geocode_result(r))
            .collect::<Result<Vec<_>>>()?;

        Ok(results)
    }

    /// Geocodifica endereço de empresa a partir de NF-e/CT-e
    pub async fn geocode_company(
        &self,
        street: &str,
        number: &str,
        neighborhood: Option<&str>,
        city: &str,
        state: &str,
        postal_code: Option<&str>,
    ) -> Result<GeocodingResult> {
        let mut address_parts = vec![format!("{}, {}", street, number)];

        if let Some(bairro) = neighborhood {
            address_parts.push(bairro.to_string());
        }

        address_parts.push(city.to_string());
        address_parts.push(state.to_string());

        if let Some(cep) = postal_code {
            address_parts.push(cep.to_string());
        }

        address_parts.push("Brasil".to_string());

        let full_address = address_parts.join(", ");

        tracing::info!("Geocodificando: {}", full_address);

    let results = self.geocode(&full_address).await?;

        results
            .first()
            .cloned()
            .context("Nenhum resultado encontrado para o endereço")
    }

    /// Busca coordenadas por CEP brasileiro
    pub async fn geocode_by_postal_code(&self, postal_code: &str) -> Result<Location> {
        let cep = postal_code.replace("-", "").replace(".", "");
        let address = format!("{}, Brasil", cep);

    let results = self.geocode(&address).await?;

        let result = results.first().context("CEP não encontrado")?;

        Ok(result.address.location.clone())
    }

    fn parse_geocode_result(&self, result: GeocodeResult) -> Result<GeocodingResult> {
        let components = &result.address_components;

        let street_number = Self::find_component(components, "street_number");
        let route = Self::find_component(components, "route");
        let neighborhood = Self::find_component(components, "sublocality")
            .or_else(|| Self::find_component(components, "neighborhood"));
        let city = Self::find_component(components, "locality")
            .or_else(|| Self::find_component(components, "administrative_area_level_2"))
            .context("Cidade não encontrada")?;
        let state = Self::find_component(components, "administrative_area_level_1")
            .context("Estado não encontrado")?;
        let country = Self::find_component(components, "country").context("País não encontrado")?;
        let postal_code = Self::find_component(components, "postal_code");

        Ok(GeocodingResult {
            address: Address {
                formatted_address: result.formatted_address,
                street_number,
                route,
                neighborhood,
                city,
                state,
                country,
                postal_code,
                location: Location {
                    lat: result.geometry.location.lat,
                    lng: result.geometry.location.lng,
                },
            },
            place_id: result.place_id,
            types: result.types,
        })
    }

    fn find_component(components: &[AddressComponent], component_type: &str) -> Option<String> {
        components
            .iter()
            .find(|c| c.types.contains(&component_type.to_string()))
            .map(|c| c.long_name.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requer API key válida
    async fn test_geocode() {
        let api_key = std::env::var("GOOGLE_MAPS_API_KEY").unwrap();
        let client = GoogleMapsClient::new(api_key).unwrap();

        let results = client
            .geocode("Avenida Paulista, 1578, São Paulo, SP")
            .await;
        assert!(results.is_ok());

        let results = results.unwrap();
        assert!(!results.is_empty());

        let first = &results[0];
        assert_eq!(first.address.city, "São Paulo");
        assert_eq!(first.address.state, "SP");
    }

    #[tokio::test]
    #[ignore]
    async fn test_reverse_geocode() {
        let api_key = std::env::var("GOOGLE_MAPS_API_KEY").unwrap();
        let client = GoogleMapsClient::new(api_key).unwrap();

        // Coordenadas da Av. Paulista
        let results = client.reverse_geocode(-23.561684, -46.655981).await;
        assert!(results.is_ok());

        let results = results.unwrap();
        assert!(!results.is_empty());
    }
}
