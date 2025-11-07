use crate::error::{GeolocationError, Result};
use crate::models::*;
use crate::parsers::{extract_access_key, FiscalDocumentParser};
use chrono::{DateTime, Utc};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub struct CTeParser;

impl CTeParser {
    pub fn new() -> Self {
        CTeParser
    }
    
    fn parse_xml_content(&self, xml: &str) -> Result<ConhecimentoTransporte> {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        
        let mut buf = Vec::new();
        let mut current_path = Vec::new();
        
        // Dados principais
        let mut chave_acesso = String::new();
        let mut numero = String::new();
        let mut serie = String::new();
        let mut data_emissao = String::new();
        let tipo_servico = TipoServicoCTe::Normal;
        
        // Emitente
        let mut emit_cnpj = String::new();
        let mut emit_razao = String::new();
        
        // Remetente
        let mut rem_cnpj = String::new();
        let mut rem_razao = String::new();
        
        // Destinatário
        let mut dest_cnpj = String::new();
        let mut dest_razao = String::new();
        
        // Valores
        let mut valor_total = 0.0;
        let mut valor_receber = 0.0;
        let mut valor_carga = 0.0;
        let mut produto_predominante = String::new();
        
        // Carga
        let mut peso_bruto = 0.0;
        
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    current_path.push(name);
                }
                Ok(Event::End(_)) => {
                    current_path.pop();
                }
                Ok(Event::Text(e)) => {
                    let text = e.unescape().unwrap_or_default().to_string();
                    
                    if let Some(tag) = current_path.last() {
                        match tag.as_str() {
                            "nCT" => numero = text,
                            "serie" => serie = text,
                            "dhEmi" => data_emissao = text,
                            "CNPJ" if current_path.contains(&"emit".to_string()) => {
                                emit_cnpj = text
                            }
                            "xNome" if current_path.contains(&"emit".to_string()) => {
                                emit_razao = text
                            }
                            "CNPJ" if current_path.contains(&"rem".to_string()) => {
                                rem_cnpj = text
                            }
                            "xNome" if current_path.contains(&"rem".to_string()) => {
                                rem_razao = text
                            }
                            "CNPJ" if current_path.contains(&"dest".to_string()) => {
                                dest_cnpj = text
                            }
                            "xNome" if current_path.contains(&"dest".to_string()) => {
                                dest_razao = text
                            }
                            "vTPrest" => valor_total = text.parse().unwrap_or(0.0),
                            "vRec" => valor_receber = text.parse().unwrap_or(0.0),
                            "vCarga" => valor_carga = text.parse().unwrap_or(0.0),
                            "proPred" => produto_predominante = text,
                            "qCarga" => peso_bruto = text.parse().unwrap_or(0.0),
                            _ => {}
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    return Err(GeolocationError::XmlParseError(format!(
                        "Erro ao parsear CT-e: {}",
                        e
                    )))
                }
                _ => {}
            }
            buf.clear();
        }
        
        // Extrai chave de acesso
        chave_acesso = extract_access_key(xml)
            .ok_or_else(|| GeolocationError::InvalidAccessKey("Chave não encontrada".into()))?;
        
        // Cria estruturas básicas
        let emitente = Participante {
            cnpj_cpf: emit_cnpj,
            razao_social: emit_razao,
            nome_fantasia: None,
            endereco: Endereco {
                logradouro: String::new(),
                numero: String::new(),
                complemento: None,
                bairro: String::new(),
                codigo_municipio: String::new(),
                municipio: String::new(),
                uf: String::new(),
                cep: String::new(),
                codigo_pais: "1058".to_string(),
                pais: "Brasil".to_string(),
            },
            inscricao_estadual: None,
            telefone: None,
            email: None,
        };
        
        let remetente = Participante {
            cnpj_cpf: rem_cnpj,
            razao_social: rem_razao,
            nome_fantasia: None,
            endereco: Endereco {
                logradouro: String::new(),
                numero: String::new(),
                complemento: None,
                bairro: String::new(),
                codigo_municipio: String::new(),
                municipio: String::new(),
                uf: String::new(),
                cep: String::new(),
                codigo_pais: "1058".to_string(),
                pais: "Brasil".to_string(),
            },
            inscricao_estadual: None,
            telefone: None,
            email: None,
        };
        
        let destinatario = Participante {
            cnpj_cpf: dest_cnpj,
            razao_social: dest_razao,
            nome_fantasia: None,
            endereco: Endereco {
                logradouro: String::new(),
                numero: String::new(),
                complemento: None,
                bairro: String::new(),
                codigo_municipio: String::new(),
                municipio: String::new(),
                uf: String::new(),
                cep: String::new(),
                codigo_pais: "1058".to_string(),
                pais: "Brasil".to_string(),
            },
            inscricao_estadual: None,
            telefone: None,
            email: None,
        };
        
        let valores_prestacao = ValoresPrestacaoCTe {
            valor_total,
            valor_receber,
            valor_total_carga: valor_carga,
            produto_predominante: produto_predominante.clone(),
            outras_caracteristicas_carga: None,
        };
        
        let informacoes_carga = InformacoesCarga {
            valor_carga,
            produto_predominante,
            peso_bruto,
            peso_cubado: None,
            quantidades: Vec::new(),
        };
        
        // Parse data de emissão
        let data_emissao_parsed = DateTime::parse_from_rfc3339(&data_emissao)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        
        Ok(ConhecimentoTransporte {
            id: Uuid::new_v4(),
            chave_acesso,
            numero,
            serie,
            data_emissao: data_emissao_parsed,
            tipo_servico,
            emitente,
            remetente,
            destinatario,
            expedidor: None,
            recebedor: None,
            valores_prestacao,
            informacoes_carga,
            documentos_referenciados: Vec::new(),
            modal: Modal::Rodoviario,
            informacoes_adicionais: None,
            protocolo_autorizacao: None,
            status: ProcessingStatus::Completed,
            created_at: Utc::now(),
        })
    }
}

impl FiscalDocumentParser for CTeParser {
    type Output = ConhecimentoTransporte;
    
    fn parse_file(&self, path: &Path) -> Result<Self::Output> {
        let content = fs::read_to_string(path)
            .map_err(|e| GeolocationError::XmlReadError(e.to_string()))?;
        self.parse_string(&content)
    }
    
    fn parse_bytes(&self, data: &[u8]) -> Result<Self::Output> {
        let content = String::from_utf8(data.to_vec())
            .map_err(|e| GeolocationError::EncodingError(e.to_string()))?;
        self.parse_string(&content)
    }
    
    fn parse_string(&self, xml: &str) -> Result<Self::Output> {
        self.parse_xml_content(xml)
    }
}

impl Default for CTeParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cte_parser_basic() {
        let parser = CTeParser::new();
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <cteProc>
            <CTe>
                <infCte Id="CTe35210112345678901234567890123456789012345678">
                    <ide>
                        <nCT>12345</nCT>
                        <serie>1</serie>
                        <dhEmi>2021-01-01T10:00:00-03:00</dhEmi>
                    </ide>
                    <emit>
                        <CNPJ>12345678000190</CNPJ>
                        <xNome>Transportadora Teste</xNome>
                    </emit>
                    <rem>
                        <CNPJ>11111111000100</CNPJ>
                        <xNome>Remetente Teste</xNome>
                    </rem>
                    <dest>
                        <CNPJ>22222222000100</CNPJ>
                        <xNome>Destinatario Teste</xNome>
                    </dest>
                    <vPrest>
                        <vTPrest>500.00</vTPrest>
                        <vRec>500.00</vRec>
                    </vPrest>
                    <infCarga>
                        <vCarga>5000.00</vCarga>
                        <proPred>Mercadorias</proPred>
                        <qCarga>1000.00</qCarga>
                    </infCarga>
                </infCte>
            </CTe>
        </cteProc>"#;
        
        let result = parser.parse_string(xml);
        assert!(result.is_ok());
        
        let cte = result.unwrap();
        assert_eq!(cte.numero, "12345");
        assert_eq!(cte.serie, "1");
        assert_eq!(cte.valores_prestacao.valor_total, 500.0);
    }
}
