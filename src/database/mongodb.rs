use anyhow::{Context, Result};
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime as BsonDateTime, Document},
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, Collection, Database as MongoDatabase,
};
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, sync::Arc};

/// Configuração do MongoDB
#[derive(Clone)]
pub struct MongoDB {
    client: Client,
    database: MongoDatabase,
}

/// Documento de pesquisa de geolocalização
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRecord {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub search_type: String, // "geocode", "reverse_geocode", "distance_matrix", "nearby", "text_search"
    pub query: String,
    pub response: serde_json::Value,
    pub timestamp: BsonDateTime,
    pub user_id: Option<String>,
    pub duration_ms: Option<i64>,
    pub error: Option<String>,
}

/// Cache de geocoding
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeocodingCache {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub address: String,
    pub normalized_address: String,
    pub latitude: f64,
    pub longitude: f64,
    pub formatted_address: String,
    pub place_id: Option<String>,
    pub cached_at: BsonDateTime,
    pub last_accessed: BsonDateTime,
    pub access_count: i32,
}

/// Cache de matriz de distância
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistanceMatrixCache {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub origin: String,
    pub destination: String,
    pub travel_mode: String,
    pub distance_meters: i64,
    pub duration_seconds: i64,
    pub cached_at: BsonDateTime,
    pub last_accessed: BsonDateTime,
    pub access_count: i32,
}

impl MongoDB {
    /// Conecta ao MongoDB Atlas
    pub async fn connect(uri: &str) -> Result<Self> {
        let mut client_options = ClientOptions::parse(uri).await?;
        
        // Define o Server API version
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        
        let client = Client::with_options(client_options)?;
        
        // Testa a conexão
        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await?;
        
        let database = client.database("geolocation");
        
        Ok(Self { client, database })
    }

    /// Retorna a coleção de pesquisas
    pub fn searches(&self) -> Collection<SearchRecord> {
        self.database.collection("searches")
    }

    /// Retorna a coleção de cache de geocoding
    pub fn geocoding_cache(&self) -> Collection<GeocodingCache> {
        self.database.collection("geocoding_cache")
    }

    /// Retorna a coleção de cache de matriz de distância
    pub fn distance_cache(&self) -> Collection<DistanceMatrixCache> {
        self.database.collection("distance_matrix_cache")
    }

    /// Cria índices otimizados
    pub async fn setup_indexes(&self) -> Result<()> {
        use mongodb::options::IndexOptions;
        use mongodb::IndexModel;

        // Índice para pesquisas por tipo e timestamp
        let searches_type_idx = IndexModel::builder()
            .keys(doc! { "search_type": 1, "timestamp": -1 })
            .build();
        
        let searches_user_idx = IndexModel::builder()
            .keys(doc! { "user_id": 1, "timestamp": -1 })
            .build();

        self.searches()
            .create_indexes(vec![searches_type_idx, searches_user_idx], None)
            .await?;

        // Índice único para cache de geocoding (evita duplicatas)
        let geocoding_address_idx = IndexModel::builder()
            .keys(doc! { "normalized_address": 1 })
            .options(IndexOptions::builder().unique(true).build())
            .build();

        let geocoding_access_idx = IndexModel::builder()
            .keys(doc! { "last_accessed": -1 })
            .build();

        self.geocoding_cache()
            .create_indexes(vec![geocoding_address_idx, geocoding_access_idx], None)
            .await?;

        // Índice composto para cache de distância
        let distance_composite_idx = IndexModel::builder()
            .keys(doc! { 
                "origin": 1, 
                "destination": 1, 
                "travel_mode": 1 
            })
            .options(IndexOptions::builder().unique(true).build())
            .build();

        let distance_access_idx = IndexModel::builder()
            .keys(doc! { "last_accessed": -1 })
            .build();

        self.distance_cache()
            .create_indexes(vec![distance_composite_idx, distance_access_idx], None)
            .await?;

        Ok(())
    }

    /// Salva um registro de pesquisa
    pub async fn save_search(&self, record: SearchRecord) -> Result<ObjectId> {
        let result = self.searches().insert_one(record, None).await?;
        Ok(result.inserted_id.as_object_id().unwrap())
    }

    /// Busca cache de geocoding
    pub async fn get_geocoding_cache(&self, address: &str) -> Result<Option<GeocodingCache>> {
        let normalized = address.to_lowercase().trim().to_string();
        let filter = doc! { "normalized_address": normalized };
        
        if let Some(mut cache) = self.geocoding_cache().find_one(filter.clone(), None).await? {
            // Atualiza contadores de acesso
            cache.access_count += 1;
            cache.last_accessed = BsonDateTime::now();
            
            self.geocoding_cache()
                .update_one(
                    filter,
                    doc! {
                        "$set": {
                            "last_accessed": cache.last_accessed,
                            "access_count": cache.access_count
                        }
                    },
                    None,
                )
                .await?;
            
            return Ok(Some(cache));
        }
        
        Ok(None)
    }

    /// Salva cache de geocoding
    pub async fn save_geocoding_cache(&self, cache: GeocodingCache) -> Result<()> {
        let filter = doc! { "normalized_address": &cache.normalized_address };
        let update = doc! {
            "$setOnInsert": {
                "address": &cache.address,
                "normalized_address": &cache.normalized_address,
                "latitude": cache.latitude,
                "longitude": cache.longitude,
                "formatted_address": &cache.formatted_address,
                "place_id": &cache.place_id,
                "cached_at": cache.cached_at,
                "access_count": 1
            },
            "$set": {
                "last_accessed": BsonDateTime::now()
            }
        };
        
        self.geocoding_cache()
            .update_one(filter, update, mongodb::options::UpdateOptions::builder().upsert(true).build())
            .await?;
        
        Ok(())
    }

    /// Busca cache de matriz de distância
    pub async fn get_distance_cache(
        &self,
        origin: &str,
        destination: &str,
        travel_mode: &str,
    ) -> Result<Option<DistanceMatrixCache>> {
        let filter = doc! {
            "origin": origin,
            "destination": destination,
            "travel_mode": travel_mode
        };
        
        if let Some(mut cache) = self.distance_cache().find_one(filter.clone(), None).await? {
            // Atualiza contadores de acesso
            cache.access_count += 1;
            cache.last_accessed = BsonDateTime::now();
            
            self.distance_cache()
                .update_one(
                    filter,
                    doc! {
                        "$set": {
                            "last_accessed": cache.last_accessed,
                            "access_count": cache.access_count
                        }
                    },
                    None,
                )
                .await?;
            
            return Ok(Some(cache));
        }
        
        Ok(None)
    }

    /// Salva cache de matriz de distância
    pub async fn save_distance_cache(&self, cache: DistanceMatrixCache) -> Result<()> {
        let filter = doc! {
            "origin": &cache.origin,
            "destination": &cache.destination,
            "travel_mode": &cache.travel_mode
        };
        
        let update = doc! {
            "$setOnInsert": {
                "origin": &cache.origin,
                "destination": &cache.destination,
                "travel_mode": &cache.travel_mode,
                "distance_meters": cache.distance_meters,
                "duration_seconds": cache.duration_seconds,
                "cached_at": cache.cached_at,
                "access_count": 1
            },
            "$set": {
                "last_accessed": BsonDateTime::now()
            }
        };
        
        self.distance_cache()
            .update_one(filter, update, mongodb::options::UpdateOptions::builder().upsert(true).build())
            .await?;
        
        Ok(())
    }

    /// Lista histórico de pesquisas com paginação
    pub async fn list_searches(
        &self,
        search_type: Option<&str>,
        user_id: Option<&str>,
        limit: i64,
        skip: u64,
    ) -> Result<Vec<SearchRecord>> {
        use mongodb::options::FindOptions;

        let mut filter = doc! {};
        if let Some(st) = search_type {
            filter.insert("search_type", st);
        }
        if let Some(uid) = user_id {
            filter.insert("user_id", uid);
        }

        let options = FindOptions::builder()
            .sort(doc! { "timestamp": -1 })
            .limit(limit)
            .skip(skip)
            .build();

        let mut cursor = self.searches().find(filter, options).await?;
        let mut results = Vec::new();

        use futures::stream::StreamExt;
        while let Some(doc) = cursor.next().await {
            results.push(doc?);
        }

        Ok(results)
    }

    /// Retorna estatísticas de uso
    pub async fn get_stats(&self) -> Result<Document> {
        let searches_count = i64::try_from(
            self.searches()
                .count_documents(doc! {}, None)
                .await?,
        )
        .context("total_searches excede o limite suportado")?;
        let geocoding_cache_count = i64::try_from(
            self.geocoding_cache()
                .count_documents(doc! {}, None)
                .await?,
        )
        .context("geocoding_cache_size excede o limite suportado")?;
        let distance_cache_count = i64::try_from(
            self.distance_cache()
                .count_documents(doc! {}, None)
                .await?,
        )
        .context("distance_cache_size excede o limite suportado")?;

        Ok(doc! {
            "total_searches": searches_count,
            "geocoding_cache_size": geocoding_cache_count,
            "distance_cache_size": distance_cache_count
        })
    }

    // =====================================================
    // Métodos para Documentos Fiscais
    // =====================================================

    /// Collection para Notas Fiscais
    pub fn notas_fiscais(&self) -> Collection<crate::models::NotaFiscal> {
        self.database.collection("notas_fiscais")
    }

    /// Collection para Conhecimentos de Transporte
    pub fn conhecimentos_transporte(&self) -> Collection<crate::models::ConhecimentoTransporte> {
        self.database.collection("conhecimentos_transporte")
    }

    /// Collection para Validações Fiscais
    pub fn validacoes_fiscais(&self) -> Collection<crate::validators::fiscal::ValidationResult> {
        self.database.collection("validacoes_fiscais")
    }

    /// Insere uma Nota Fiscal
    pub async fn insert_nota_fiscal(&self, nfe: &crate::models::NotaFiscal) -> Result<ObjectId> {
        let result = self.notas_fiscais().insert_one(nfe, None).await?;
        Ok(result.inserted_id.as_object_id().unwrap())
    }

    /// Insere um Conhecimento de Transporte
    pub async fn insert_conhecimento_transporte(&self, cte: &crate::models::ConhecimentoTransporte) -> Result<ObjectId> {
        let result = self.conhecimentos_transporte().insert_one(cte, None).await?;
        Ok(result.inserted_id.as_object_id().unwrap())
    }

    /// Busca Nota Fiscal por chave de acesso
    pub async fn find_nota_fiscal_by_chave(&self, chave: &str) -> Result<Option<crate::models::NotaFiscal>> {
        let filter = doc! { "chave_acesso": chave };
        Ok(self.notas_fiscais().find_one(filter, None).await?)
    }

    /// Busca Conhecimento de Transporte por chave de acesso  
    pub async fn find_conhecimento_by_chave(&self, chave: &str) -> Result<Option<crate::models::ConhecimentoTransporte>> {
        let filter = doc! { "chave_acesso": chave };
        Ok(self.conhecimentos_transporte().find_one(filter, None).await?)
    }

    /// Insere resultado de validação fiscal
    pub async fn insert_validation(&self, validation: &crate::validators::fiscal::ValidationResult) -> Result<ObjectId> {
        let result = self.validacoes_fiscais().insert_one(validation, None).await?;
        Ok(result.inserted_id.as_object_id().unwrap())
    }

    /// Busca validação por chave de acesso
    pub async fn find_validation_by_chave(&self, chave: &str) -> Result<Option<crate::validators::fiscal::ValidationResult>> {
        let filter = doc! { "chave_acesso": chave };
        Ok(self.validacoes_fiscais().find_one(filter, None).await?)
    }

    /// Lista documentos com paginação
    pub async fn list_documents(&self, doc_type: Option<&str>, limit: i64, offset: i64) -> Result<Vec<crate::database::repository::DocumentSummary>> {
        let mut results = Vec::new();

        // Buscar NFes se não for especificado tipo ou se for "NFe"
        if doc_type.is_none() || doc_type == Some("NFe") {
            use mongodb::options::FindOptions;
            let options = FindOptions::builder()
                .skip(offset as u64)
                .limit(limit)
                .build();
            
            let mut cursor = self.notas_fiscais().find(doc! {}, options).await?;
            while cursor.advance().await? {
                let nfe = cursor.deserialize_current()?;
                results.push(crate::database::repository::DocumentSummary {
                    document_type: "NFe".to_string(),
                    chave_acesso: nfe.chave_acesso,
                    numero: nfe.numero,
                    serie: nfe.serie,
                    data_emissao: nfe.data_emissao.to_rfc3339(),
                    emitente: nfe.emitente.razao_social,
                    destinatario: nfe.destinatario.razao_social,
                    valor_total: nfe.totais.valor_total,
                });
            }
        }

        // Buscar CTes se não for especificado tipo ou se for "CTe"
        if doc_type.is_none() || doc_type == Some("CTe") {
            use mongodb::options::FindOptions;
            let options = FindOptions::builder()
                .skip(if doc_type.is_some() { offset } else { 0 } as u64)
                .limit(if doc_type.is_some() { limit } else { limit - results.len() as i64 })
                .build();
            
            let mut cursor = self.conhecimentos_transporte().find(doc! {}, options).await?;
            while cursor.advance().await? {
                let cte = cursor.deserialize_current()?;
                results.push(crate::database::repository::DocumentSummary {
                    document_type: "CTe".to_string(),
                    chave_acesso: cte.chave_acesso,
                    numero: cte.numero,
                    serie: cte.serie,
                    data_emissao: cte.data_emissao.to_rfc3339(),
                    emitente: cte.emitente.razao_social,
                    destinatario: cte.destinatario.razao_social,
                    valor_total: cte.valores_prestacao.valor_total,
                });
            }
        }

        Ok(results)
    }

    /// Conta total de documentos
    pub async fn count_documents(&self, doc_type: Option<&str>) -> Result<i64> {
        let mut total = 0i64;

        if doc_type.is_none() || doc_type == Some("NFe") {
            total += self.notas_fiscais().count_documents(doc! {}, None).await? as i64;
        }
        if doc_type.is_none() || doc_type == Some("CTe") {
            total += self.conhecimentos_transporte().count_documents(doc! {}, None).await? as i64;
        }

        Ok(total)
    }
}

/// Wrapper Arc para compartilhamento entre threads
pub type MongoDBConnection = Arc<MongoDB>;
