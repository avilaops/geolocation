use anyhow::{Context, Result};
use reqwest::Client;
use std::time::Duration;

/// Cliente HTTP para Google Maps Platform APIs
#[derive(Clone)]
pub struct GoogleMapsClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl GoogleMapsClient {
    /// Cria novo cliente com API key
    pub fn new(api_key: String) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .context("Falha ao criar HTTP client")?;

        Ok(Self {
            client,
            api_key,
            base_url: "https://maps.googleapis.com/maps/api".to_string(),
        })
    }

    /// Retorna a API key
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// Retorna o cliente HTTP interno
    pub fn http_client(&self) -> &Client {
        &self.client
    }

    /// URL base das APIs
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Constrói URL com parâmetros
    pub fn build_url(&self, endpoint: &str, params: &[(&str, &str)]) -> String {
        let mut url = format!("{}/{}", self.base_url, endpoint);
        url.push_str(&format!("?key={}", self.api_key));

        for (key, value) in params {
            url.push_str(&format!("&{}={}", key, urlencoding::encode(value)));
        }

        url
    }

    /// Faz requisição GET e retorna JSON
    pub async fn get_json<T>(&self, url: &str) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        tracing::debug!("Google Maps API request: {}", url);

        let response = self
            .client
            .get(url)
            .send()
            .await
            .context("Falha na requisição HTTP")?;

        let status = response.status();
        let body = response
            .text()
            .await
            .context("Falha ao ler resposta HTTP")?;

        if !status.is_success() {
            anyhow::bail!("Google Maps API error: {} - {}", status, body);
        }

        serde_json::from_str(&body).context("Falha ao deserializar resposta JSON")
    }

    /// Valida se a API key está funcionando
    pub async fn validate_api_key(&self) -> Result<bool> {
        let url = self.build_url("geocode/json", &[("address", "São Paulo, Brasil")]);

        match self.get_json::<serde_json::Value>(&url).await {
            Ok(response) => {
                let status = response["status"].as_str().unwrap_or("UNKNOWN");
                Ok(status != "REQUEST_DENIED")
            }
            Err(e) => {
                tracing::error!("Erro ao validar API key: {}", e);
                Ok(false)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_client_creation() {
        let client = GoogleMapsClient::new("test_key".to_string());
        assert!(client.is_ok());
    }

    #[test]
    fn test_build_url() {
        let client = GoogleMapsClient::new("my_key".to_string()).unwrap();
        let url = client.build_url(
            "geocode/json",
            &[("address", "Rua ABC, 123"), ("region", "br")],
        );

        assert!(url.contains("key=my_key"));
        assert!(url.contains("address=Rua"));
        assert!(url.contains("region=br"));
    }
}
