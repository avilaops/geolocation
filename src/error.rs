use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeolocationError {
    #[error("Erro ao ler arquivo XML: {0}")]
    XmlReadError(String),

    #[error("Erro ao parsear XML: {0}")]
    XmlParseError(String),

    #[error("Estrutura XML inválida: {0}")]
    InvalidXmlStructure(String),

    #[error("Assinatura digital inválida")]
    InvalidSignature,

    #[error("Chave de acesso inválida: {0}")]
    InvalidAccessKey(String),

    #[error("Erro no banco de dados: {0}")]
    DatabaseError(String),

    #[error("Tipo de documento não suportado: {0}")]
    UnsupportedDocumentType(String),

    #[error("Campo obrigatório ausente: {0}")]
    MissingRequiredField(String),

    #[error("Erro de IO: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Erro de encoding: {0}")]
    EncodingError(String),

    #[error("Erro desconhecido: {0}")]
    Unknown(String),

    #[error("Documento já existente (chave duplicada): {0}")]
    DuplicateDocument(String),
}

impl From<quick_xml::Error> for GeolocationError {
    fn from(err: quick_xml::Error) -> Self {
        GeolocationError::XmlParseError(err.to_string())
    }
}

impl From<serde_json::Error> for GeolocationError {
    fn from(err: serde_json::Error) -> Self {
        GeolocationError::Unknown(format!("Erro ao serializar JSON: {}", err))
    }
}

impl From<sqlx::Error> for GeolocationError {
    fn from(err: sqlx::Error) -> Self {
        match &err {
            sqlx::Error::Database(db_err) => {
                // Postgres: código 23505 para unique violation
                if let Some(code) = db_err.code() {
                    if code == "23505" {
                        return GeolocationError::DuplicateDocument(db_err.message().to_string());
                    }
                }
                // SQLite: mensagem costuma conter "UNIQUE constraint failed"
                let msg = db_err.message().to_lowercase();
                if msg.contains("unique constraint failed") || msg.contains("constraint failed") {
                    return GeolocationError::DuplicateDocument(db_err.message().to_string());
                }
                GeolocationError::DatabaseError(db_err.message().to_string())
            }
            _ => GeolocationError::DatabaseError(err.to_string()),
        }
    }
}

impl From<anyhow::Error> for GeolocationError {
    fn from(err: anyhow::Error) -> Self {
        GeolocationError::Unknown(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, GeolocationError>;
