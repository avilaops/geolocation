pub mod nfe;
pub mod cte;

use crate::error::Result;
use crate::models::{ConhecimentoTransporte, NotaFiscal};
use std::path::Path;

/// Parser trait para documentos fiscais
pub trait FiscalDocumentParser {
    type Output;
    
    fn parse_file(&self, path: &Path) -> Result<Self::Output>;
    fn parse_bytes(&self, data: &[u8]) -> Result<Self::Output>;
    fn parse_string(&self, xml: &str) -> Result<Self::Output>;
}

/// Detecta o tipo de documento fiscal baseado no conteúdo XML
pub fn detect_document_type(xml: &str) -> Option<String> {
    if xml.contains("<nfeProc") || xml.contains("<NFe") {
        Some("NFe".to_string())
    } else if xml.contains("<cteProc") || xml.contains("<CTe") {
        Some("CTe".to_string())
    } else {
        None
    }
}

/// Valida a chave de acesso de documentos fiscais (44 dígitos)
pub fn validate_access_key(key: &str) -> bool {
    if key.len() != 44 {
        return false;
    }
    
    key.chars().all(|c| c.is_ascii_digit())
}

/// Extrai a chave de acesso do XML
pub fn extract_access_key(xml: &str) -> Option<String> {
    // Procura por padrões comuns de chave de acesso
    let patterns = [
        "<chNFe>",
        "<chCTe>",
        "Id=\"NFe",
        "Id=\"CTe",
    ];
    
    for pattern in &patterns {
        if let Some(start) = xml.find(pattern) {
            let start_pos = start + pattern.len();
            let end_pos = start_pos + 44;
            
            if end_pos <= xml.len() {
                let key = &xml[start_pos..end_pos];
                if validate_access_key(key) {
                    return Some(key.to_string());
                }
            }
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_document_type() {
        let nfe_xml = r#"<?xml version="1.0"?><nfeProc><NFe></NFe></nfeProc>"#;
        assert_eq!(detect_document_type(nfe_xml), Some("NFe".to_string()));
        
        let cte_xml = r#"<?xml version="1.0"?><cteProc><CTe></CTe></cteProc>"#;
        assert_eq!(detect_document_type(cte_xml), Some("CTe".to_string()));
        
        let invalid_xml = r#"<?xml version="1.0"?><root></root>"#;
        assert_eq!(detect_document_type(invalid_xml), None);
    }

    #[test]
    fn test_validate_access_key() {
        assert!(validate_access_key("35210112345678901234567890123456789012345678"));
        assert!(!validate_access_key("123"));
        assert!(!validate_access_key("3521011234567890123456789012345678901234567X"));
    }
}
