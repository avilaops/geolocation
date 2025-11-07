pub mod asm;
pub mod database;
pub mod error;
// TODO: Habilitar após compilação básica funcionar
pub mod google_maps;
// pub mod google_maps_api;
pub mod models;
pub mod parsers;
pub mod utils;
pub mod validators;

#[cfg(feature = "wasm")]
pub mod wasm;

pub use database::{Database, DatabasePool};
pub use error::{GeolocationError, Result};
pub use models::{ConhecimentoTransporte, DocumentType, NotaFiscal};
pub use parsers::{cte::CTeParser, nfe::NFeParser, FiscalDocumentParser};

/// Versão do software
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Nome do software
pub const APP_NAME: &str = "Geolocation";

/// Processa um arquivo XML de documento fiscal
pub async fn process_document_file(file_path: &str, mongo: &database::mongodb::MongoDB) -> Result<ProcessingResult> {
    let content = std::fs::read_to_string(file_path)
        .map_err(|e| GeolocationError::XmlReadError(e.to_string()))?;

    process_document_content(&content, mongo).await
}

/// Processa o conteúdo XML de um documento fiscal
pub async fn process_document_content(
    xml_content: &str,
    mongo: &database::mongodb::MongoDB,
) -> Result<ProcessingResult> {
    // Detecta o tipo de documento
    let doc_type = parsers::detect_document_type(xml_content).ok_or_else(|| {
        GeolocationError::UnsupportedDocumentType(
            "Não foi possível detectar o tipo de documento".into(),
        )
    })?;

    // Valida o documento fiscalmente ANTES de processar
    let validation = validators::FiscalValidator::validate_document(xml_content, &doc_type);

    let result = match doc_type.as_str() {
        "NFe" => {
            let parser = NFeParser::new();
            let nf = parser.parse_string(xml_content)?;
            let chave = nf.chave_acesso.clone();
            let already = mongo.find_nota_fiscal_by_chave(&chave).await?;
            let mut duplicate = false;
            if already.is_none() {
                if let Err(e) = mongo.insert_nota_fiscal(&nf).await {
                    // Tratar erro de duplicata do MongoDB
                    if e.to_string().contains("duplicate") {
                        duplicate = true;
                    } else {
                        return Err(GeolocationError::DatabaseError(e.to_string()));
                    }
                }
            } else {
                duplicate = true;
            }
            if duplicate {
                tracing::info!(chave = %chave, "NF-e duplicada detectada");
                if let Some(counter) = crate::utils::metrics::docs_duplicate_counter() {
                    counter.inc();
                }
            } else {
                if let Some(counter) = crate::utils::metrics::docs_processed_counter() {
                    counter.inc();
                }
            }
            ProcessingResult {
                document_type: DocumentType::NotaFiscal,
                chave_acesso: chave,
                success: true,
                message: if duplicate {
                    "NF-e já existente".to_string()
                } else {
                    "NF-e processada com sucesso".to_string()
                },
                validation: Some(validation),
                duplicate,
            }
        }
        "CTe" => {
            let parser = CTeParser::new();
            let cte = parser.parse_string(xml_content)?;
            let chave = cte.chave_acesso.clone();
            let already = mongo.find_conhecimento_by_chave(&chave).await?;
            let mut duplicate = false;
            if already.is_none() {
                if let Err(e) = mongo.insert_conhecimento_transporte(&cte).await {
                    // Tratar erro de duplicata do MongoDB
                    if e.to_string().contains("duplicate") {
                        duplicate = true;
                    } else {
                        return Err(GeolocationError::DatabaseError(e.to_string()));
                    }
                }
            } else {
                duplicate = true;
            }
            if duplicate {
                tracing::info!(chave = %chave, "CT-e duplicado detectado");
                if let Some(counter) = crate::utils::metrics::docs_duplicate_counter() {
                    counter.inc();
                }
            } else {
                if let Some(counter) = crate::utils::metrics::docs_processed_counter() {
                    counter.inc();
                }
            }
            ProcessingResult {
                document_type: DocumentType::ConhecimentoTransporte,
                chave_acesso: chave,
                success: true,
                message: if duplicate {
                    "CT-e já existente".to_string()
                } else {
                    "CT-e processado com sucesso".to_string()
                },
                validation: Some(validation.clone()),
                duplicate,
            }
        }
        _ => {
            return Err(GeolocationError::UnsupportedDocumentType(format!(
                "Tipo de documento não suportado: {}",
                doc_type
            )))
        }
    };
    // Persiste a validação no MongoDB, se disponível
    if let Some(ref val) = result.validation {
        if let Err(e) = mongo.insert_validation(val).await {
            log::warn!(
                "Falha ao persistir validação para chave {}: {}",
                result.chave_acesso,
                e
            );
        }
        // Métricas de validação
        if let Some(counter) = crate::utils::metrics::validations_saved_counter() {
            counter.inc();
        }
    }
    Ok(result)
}

/// Resultado do processamento de um documento
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessingResult {
    pub document_type: DocumentType,
    pub chave_acesso: String,
    pub success: bool,
    pub message: String,
    pub validation: Option<validators::ValidationResult>,
    pub duplicate: bool,
}
