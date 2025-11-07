use crate::google_maps::{
    Distance, DistanceMatrixElement, DistanceMatrixRequest, DistanceMatrixResult,
    DistanceMatrixRow, Duration, GoogleMapsClient, Location, TravelMode,
};
use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DistanceMatrixResponse {
    rows: Vec<DistanceMatrixRowApi>,
    status: String,
}

#[derive(Debug, Deserialize)]
struct DistanceMatrixRowApi {
    elements: Vec<DistanceMatrixElementApi>,
}

#[derive(Debug, Deserialize)]
struct DistanceMatrixElementApi {
    distance: Option<DistanceApi>,
    duration: Option<DurationApi>,
    status: String,
}

#[derive(Debug, Deserialize)]
struct DistanceApi {
    text: String,
    value: u32,
}

#[derive(Debug, Deserialize)]
struct DurationApi {
    text: String,
    value: u32,
}

impl GoogleMapsClient {
    /// Calcula distância e tempo de viagem entre pontos
    pub async fn distance_matrix(
        &self,
        request: DistanceMatrixRequest,
    ) -> Result<DistanceMatrixResult> {
        let origins = request
            .origins
            .iter()
            .map(|l| format!("{},{}", l.lat, l.lng))
            .collect::<Vec<_>>()
            .join("|");

        let destinations = request
            .destinations
            .iter()
            .map(|l| format!("{},{}", l.lat, l.lng))
            .collect::<Vec<_>>()
            .join("|");

        let mode = match request.mode {
            TravelMode::Driving => "driving",
            TravelMode::Walking => "walking",
            TravelMode::Bicycling => "bicycling",
            TravelMode::Transit => "transit",
        };

        let url = self.build_url(
            "distancematrix/json",
            &[
                ("origins", &origins),
                ("destinations", &destinations),
                ("mode", mode),
                ("language", "pt-BR"),
            ],
        );

        let response: DistanceMatrixResponse = self.get_json(&url).await?;

        if response.status != "OK" {
            anyhow::bail!("Distance Matrix falhou: {}", response.status);
        }

        let rows = response
            .rows
            .into_iter()
            .map(|row| {
                let elements = row
                    .elements
                    .into_iter()
                    .map(|elem| DistanceMatrixElement {
                        distance: elem.distance.map(|d| Distance {
                            text: d.text,
                            value: d.value,
                        }),
                        duration: elem.duration.map(|d| Duration {
                            text: d.text,
                            value: d.value,
                        }),
                        status: elem.status,
                    })
                    .collect();

                DistanceMatrixRow { elements }
            })
            .collect();

        Ok(DistanceMatrixResult { rows })
    }

    /// Calcula distância simples entre dois pontos
    pub async fn calculate_distance(
        &self,
        origin: Location,
        destination: Location,
        mode: TravelMode,
    ) -> Result<(Distance, Duration)> {
        let request = DistanceMatrixRequest {
            origins: vec![origin],
            destinations: vec![destination],
            mode,
        };

        let result = self.distance_matrix(request).await?;

        let element = result
            .rows
            .first()
            .and_then(|row| row.elements.first())
            .context("Nenhum resultado na matriz de distância")?;

        if element.status != "OK" {
            anyhow::bail!("Cálculo de distância falhou: {}", element.status);
        }

        let distance = element
            .distance
            .clone()
            .context("Distância não disponível")?;
        let duration = element.duration.clone().context("Duração não disponível")?;

        Ok((distance, duration))
    }

    /// Calcula distâncias de um ponto para múltiplos destinos
    pub async fn calculate_distances_to_many(
        &self,
        origin: Location,
        destinations: Vec<Location>,
        mode: TravelMode,
    ) -> Result<Vec<(Distance, Duration)>> {
        let request = DistanceMatrixRequest {
            origins: vec![origin],
            destinations,
            mode,
        };

        let result = self.distance_matrix(request).await?;

        let row = result
            .rows
            .first()
            .context("Nenhuma linha na matriz de distância")?;

        let distances = row
            .elements
            .iter()
            .filter_map(|elem| {
                if elem.status == "OK" {
                    Some((elem.distance.clone()?, elem.duration.clone()?))
                } else {
                    None
                }
            })
            .collect();

        Ok(distances)
    }

    /// Encontra o destino mais próximo de uma origem
    pub async fn find_nearest(
        &self,
        origin: Location,
        destinations: Vec<Location>,
        mode: TravelMode,
    ) -> Result<(usize, Distance, Duration)> {
        let distances = self
            .calculate_distances_to_many(origin, destinations, mode)
            .await?;

        let (index, (distance, duration)) = distances
            .into_iter()
            .enumerate()
            .min_by_key(|(_, (d, _))| d.value)
            .context("Nenhum destino alcançável")?;

        Ok((index, distance, duration))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_distance_matrix() {
        let api_key = std::env::var("GOOGLE_MAPS_API_KEY").unwrap();
        let client = GoogleMapsClient::new(api_key).unwrap();

        // Av. Paulista -> Praça da Sé
        let origin = Location {
            lat: -23.561684,
            lng: -46.655981,
        };
        let destination = Location {
            lat: -23.550520,
            lng: -46.633309,
        };

        let (distance, duration) = client
            .calculate_distance(origin, destination, TravelMode::Driving)
            .await
            .unwrap();

        assert!(distance.value > 0);
        assert!(duration.value > 0);
        println!("Distância: {} ({})", distance.text, distance.value);
        println!("Tempo: {} ({}s)", duration.text, duration.value);
    }

    #[tokio::test]
    #[ignore]
    async fn test_find_nearest() {
        let api_key = std::env::var("GOOGLE_MAPS_API_KEY").unwrap();
        let client = GoogleMapsClient::new(api_key).unwrap();

        let origin = Location {
            lat: -23.561684,
            lng: -46.655981,
        };
        let destinations = vec![
            Location {
                lat: -23.550520,
                lng: -46.633309,
            }, // Sé
            Location {
                lat: -23.587416,
                lng: -46.632014,
            }, // Barra Funda
            Location {
                lat: -23.533773,
                lng: -46.625290,
            }, // Luz
        ];

        let (index, distance, duration) = client
            .find_nearest(origin, destinations, TravelMode::Driving)
            .await
            .unwrap();

        println!("Destino mais próximo: índice {}", index);
        println!("Distância: {}", distance.text);
        println!("Tempo: {}", duration.text);
    }
}
