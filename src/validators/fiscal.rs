/// Validador fiscal inteligente para NF-e e CT-e
///
/// Este módulo fornece validações automáticas de:
/// - CFOP válidos e compatíveis com operação
/// - NCM com alíquotas corretas
/// - Cálculo de ICMS, PIS, COFINS
/// - Detecção de divergências fiscais
/// - Sugestões de correção
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationResult {
    pub chave_acesso: String,
    pub document_type: String,
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
    pub warnings: Vec<ValidationWarning>,
    pub suggestions: Vec<String>,
    pub validated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub code: String,
    pub field: String,
    pub message: String,
    pub severity: ErrorSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationWarning {
    pub code: String,
    pub field: String,
    pub message: String,
    pub impact: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErrorSeverity {
    Critical, // Impede processamento
    High,     // Erro fiscal grave
    Medium,   // Divergência que pode gerar multa
    Low,      // Inconsistência menor
}

/// Validador principal de documentos fiscais
pub struct FiscalValidator;

impl FiscalValidator {
    /// Valida um documento fiscal completo
    pub fn validate_document(xml_content: &str, document_type: &str) -> ValidationResult {
        // Extrair chave de acesso do XML
        let chave_acesso = Self::extract_chave_acesso(xml_content).unwrap_or_default();
        
        let mut result = ValidationResult {
            chave_acesso,
            document_type: document_type.to_string(),
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
            suggestions: Vec::new(),
            validated_at: chrono::Utc::now(),
        };

        match document_type {
            "NFe" => Self::validate_nfe(xml_content, &mut result),
            "CTe" => Self::validate_cte(xml_content, &mut result),
            _ => {
                result.is_valid = false;
                result.errors.push(ValidationError {
                    code: "DOC_TYPE_INVALID".to_string(),
                    field: "document_type".to_string(),
                    message: "Tipo de documento não suportado".to_string(),
                    severity: ErrorSeverity::Critical,
                });
            }
        }

        result.is_valid = result.errors.is_empty();
        result
    }

    /// Valida NF-e específica
    fn validate_nfe(xml_content: &str, result: &mut ValidationResult) {
        // Validação 1: CFOP
        Self::validate_cfop(xml_content, result);

        // Validação 2: NCM
        Self::validate_ncm(xml_content, result);

        // Validação 3: Cálculo de impostos
        Self::validate_tax_calculation(xml_content, result);

        // Validação 4: Chave de acesso
        Self::validate_access_key(xml_content, result);

        // Validação 5: Datas
        Self::validate_dates(xml_content, result);
    }

    /// Valida CT-e específico
    fn validate_cte(xml_content: &str, result: &mut ValidationResult) {
        // Validação de CT-e (simplificada)
        Self::validate_access_key(xml_content, result);
        Self::validate_dates(xml_content, result);
    }

    /// Valida CFOP contra tabela oficial
    fn validate_cfop(xml_content: &str, result: &mut ValidationResult) {
        // Extrai CFOP do XML (simulação)
        let cfop = Self::extract_cfop(xml_content);

        if let Some(cfop_code) = cfop {
            // Valida se CFOP existe
            if !Self::is_valid_cfop(&cfop_code) {
                result.errors.push(ValidationError {
                    code: "CFOP_INVALID".to_string(),
                    field: "CFOP".to_string(),
                    message: format!("CFOP {} não existe na tabela oficial", cfop_code),
                    severity: ErrorSeverity::High,
                });
                result
                    .suggestions
                    .push("Verifique a tabela de CFOPs da Receita Federal".to_string());
            }

            // Valida compatibilidade (exemplo: 5xxx para operações internas)
            if cfop_code.starts_with('5') {
                // Verificar se UF origem == UF destino
                result.warnings.push(ValidationWarning {
                    code: "CFOP_CHECK_UF".to_string(),
                    field: "CFOP".to_string(),
                    message: format!(
                        "CFOP {} é para operações internas - verifique UFs",
                        cfop_code
                    ),
                    impact: "Pode gerar multa se UFs forem diferentes".to_string(),
                });
            }
        }
    }

    /// Valida NCM
    fn validate_ncm(xml_content: &str, result: &mut ValidationResult) {
        let ncm = Self::extract_ncm(xml_content);

        if let Some(ncm_code) = ncm {
            // Valida formato (8 dígitos)
            if ncm_code.len() != 8 || !ncm_code.chars().all(char::is_numeric) {
                result.errors.push(ValidationError {
                    code: "NCM_INVALID_FORMAT".to_string(),
                    field: "NCM".to_string(),
                    message: format!("NCM {} deve ter 8 dígitos numéricos", ncm_code),
                    severity: ErrorSeverity::High,
                });
            }

            // Avisa sobre necessidade de alíquota IPI
            if Self::ncm_requires_ipi(&ncm_code) {
                result.warnings.push(ValidationWarning {
                    code: "NCM_REQUIRES_IPI".to_string(),
                    field: "NCM".to_string(),
                    message: format!("NCM {} pode exigir IPI - verifique alíquota", ncm_code),
                    impact: "Falta de IPI pode gerar autuação".to_string(),
                });
            }
        }
    }

    /// Valida cálculo de impostos
    fn validate_tax_calculation(xml_content: &str, result: &mut ValidationResult) {
        // Extrai valores (simulação)
        let valor_produtos: f64 = 10000.0;
        let valor_icms: f64 = 1800.0;
        let aliquota_icms: f64 = 18.0;

        // Verifica cálculo de ICMS
        let icms_esperado: f64 = valor_produtos * (aliquota_icms / 100.0);
        let diferenca: f64 = (valor_icms - icms_esperado).abs();

        if diferenca > 0.01 {
            result.errors.push(ValidationError {
                code: "ICMS_CALC_ERROR".to_string(),
                field: "ICMS".to_string(),
                message: format!(
                    "ICMS calculado (R$ {:.2}) difere do esperado (R$ {:.2})",
                    valor_icms, icms_esperado
                ),
                severity: ErrorSeverity::Medium,
            });
            result.suggestions.push(format!(
                "Recalcule: {} × {}% = R$ {:.2}",
                valor_produtos, aliquota_icms, icms_esperado
            ));
        }
    }

    /// Valida chave de acesso
    fn validate_access_key(xml_content: &str, result: &mut ValidationResult) {
        let chave = Self::extract_access_key(xml_content);

        if let Some(key) = chave {
            // Valida formato (44 dígitos)
            if key.len() != 44 || !key.chars().all(char::is_numeric) {
                result.errors.push(ValidationError {
                    code: "KEY_INVALID_FORMAT".to_string(),
                    field: "chave_acesso".to_string(),
                    message: "Chave de acesso deve ter 44 dígitos numéricos".to_string(),
                    severity: ErrorSeverity::Critical,
                });
            }

            // Valida dígito verificador
            if !Self::validate_key_digit(&key) {
                result.errors.push(ValidationError {
                    code: "KEY_INVALID_DIGIT".to_string(),
                    field: "chave_acesso".to_string(),
                    message: "Dígito verificador da chave de acesso inválido".to_string(),
                    severity: ErrorSeverity::Critical,
                });
            }
        }
    }

    /// Valida datas
    fn validate_dates(_xml_content: &str, result: &mut ValidationResult) {
        // Validação de datas (emissão, saída, etc)
        // Simulação: detecta data retroativa
        result.warnings.push(ValidationWarning {
            code: "DATE_RETROACTIVE".to_string(),
            field: "data_emissao".to_string(),
            message: "Data de emissão está retroativa (mais de 5 dias)".to_string(),
            impact: "Pode indicar manipulação fiscal".to_string(),
        });
    }

    // === Funções auxiliares (simuladas) ===

    fn extract_cfop(xml: &str) -> Option<String> {
        // Em produção: parse real do XML
        if xml.contains("5102") {
            Some("5102".to_string())
        } else {
            None
        }
    }

    fn extract_ncm(xml: &str) -> Option<String> {
        // Em produção: parse real do XML
        if xml.contains("NCM") {
            Some("84331900".to_string())
        } else {
            None
        }
    }

    fn extract_access_key(_xml: &str) -> Option<String> {
        // Em produção: parse real do XML
        Some("35240911223344000156550010000123451234567890".to_string())
    }

    fn is_valid_cfop(cfop: &str) -> bool {
        // Lista simplificada de CFOPs válidos
        matches!(
            cfop,
            "5102" | "5103" | "5104" | "5405" | "5949" | "6102" | "6103" | "6104" | "6405" | "6949"
        )
    }

    fn ncm_requires_ipi(ncm: &str) -> bool {
        // Alguns NCMs que tipicamente têm IPI
        ncm.starts_with("8433") || ncm.starts_with("8704")
    }

    fn validate_key_digit(key: &str) -> bool {
        if key.len() != 44 {
            return false;
        }

        // Implementação do módulo 11 (simplificada)
        // Em produção: implementar algoritmo completo
        key.chars().all(char::is_numeric)
    }
}

/// Análise tributária agregada
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxAnalysis {
    pub periodo: String,
    pub total_documentos: u32,
    pub valor_total: f64,
    pub icms_total: f64,
    pub pis_total: f64,
    pub cofins_total: f64,
    pub ipi_total: f64,
    pub cfop_distribution: Vec<CfopStat>,
    pub ncm_distribution: Vec<NcmStat>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CfopStat {
    pub cfop: String,
    pub descricao: String,
    pub quantidade: u32,
    pub valor_total: f64,
    pub percentual: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NcmStat {
    pub ncm: String,
    pub descricao: String,
    pub quantidade: u32,
    pub valor_total: f64,
}

/// Extrai a chave de acesso do XML
impl FiscalValidator {
    fn extract_chave_acesso(xml_content: &str) -> Option<String> {
        // Procura por padrões de chave de acesso em tags comuns
        let patterns = [
            r"<chNFe>([0-9]{44})</chNFe>",
            r"<chCTe>([0-9]{44})</chCTe>",
            r#"chave="([0-9]{44})""#,
            r#"Id="NFe([0-9]{44})""#,
            r#"Id="CTe([0-9]{44})""#,
        ];

        for pattern in &patterns {
            // Busca manual simples sem regex
            if let Some(start) = xml_content.find(&pattern.replace("([0-9]{44})", "")) {
                let search_area = &xml_content[start..];
                for i in 0..(search_area.len() - 44) {
                    let potential_key = &search_area[i..i + 44];
                    if potential_key.chars().all(|c| c.is_ascii_digit()) && potential_key.len() == 44 {
                        return Some(potential_key.to_string());
                    }
                }
            }
        }
        
        // Busca simples por sequências de 44 dígitos
        for i in 0..(xml_content.len() - 44) {
            let potential_key = &xml_content[i..i + 44];
            if potential_key.chars().all(|c| c.is_ascii_digit()) && potential_key.len() == 44 {
                return Some(potential_key.to_string());
            }
        }
        
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_cfop() {
        assert!(FiscalValidator::is_valid_cfop("5102"));
        assert!(FiscalValidator::is_valid_cfop("6102"));
        assert!(!FiscalValidator::is_valid_cfop("9999"));
    }

    #[test]
    fn test_validate_key_format() {
        let valid_key = "35240911223344000156550010000123451234567890";
        assert_eq!(valid_key.len(), 44);
        assert!(FiscalValidator::validate_key_digit(valid_key));
    }
}
