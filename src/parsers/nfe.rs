use crate::error::{GeolocationError, Result};
use crate::models::*;
use crate::parsers::{extract_access_key, FiscalDocumentParser};
use chrono::{DateTime, Utc};
use quick_xml::events::Event;
use quick_xml::Reader;
use std::fs;
use std::path::Path;
use uuid::Uuid;

pub struct NFeParser;

impl NFeParser {
    pub fn new() -> Self {
        NFeParser
    }
    
    fn parse_xml_content(&self, xml: &str) -> Result<NotaFiscal> {
        let mut reader = Reader::from_str(xml);
        reader.trim_text(true);
        
        let mut buf = Vec::new();
        let mut current_path = Vec::new();
        
        // Dados principais
        let mut chave_acesso = String::new();
        let mut numero = String::new();
        let mut serie = String::new();
        let mut data_emissao = String::new();
        let mut tipo_nota = TipoNota::Saida;
        
        // Emitente
        let mut emit_cnpj = String::new();
        let mut emit_razao = String::new();
        let mut emit_fantasia = None;
        
        // Destinatário
        let mut dest_cnpj = String::new();
        let mut dest_razao = String::new();
        
        // Totais
        let mut valor_total = 0.0;
        let mut valor_produtos = 0.0;
        
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
                            "nNF" => numero = text,
                            "serie" => serie = text,
                            "dhEmi" => data_emissao = text,
                            "CNPJ" if current_path.contains(&"emit".to_string()) => {
                                emit_cnpj = text
                            }
                            "xNome" if current_path.contains(&"emit".to_string()) => {
                                emit_razao = text
                            }
                            "xFant" if current_path.contains(&"emit".to_string()) => {
                                emit_fantasia = Some(text)
                            }
                            "CNPJ" if current_path.contains(&"dest".to_string()) => {
                                dest_cnpj = text
                            }
                            "xNome" if current_path.contains(&"dest".to_string()) => {
                                dest_razao = text
                            }
                            "vNF" => valor_total = text.parse().unwrap_or(0.0),
                            "vProd" if current_path.contains(&"ICMSTot".to_string()) => {
                                valor_produtos = text.parse().unwrap_or(0.0)
                            }
                            _ => {}
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    return Err(GeolocationError::XmlParseError(format!(
                        "Erro ao parsear NF-e: {}",
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
            nome_fantasia: emit_fantasia,
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
        
        let totais = Totais {
            base_calculo_icms: 0.0,
            valor_icms: 0.0,
            valor_icms_desonerado: 0.0,
            valor_fcp: 0.0,
            base_calculo_icms_st: 0.0,
            valor_icms_st: 0.0,
            valor_produtos,
            valor_frete: 0.0,
            valor_seguro: 0.0,
            valor_desconto: 0.0,
            valor_ii: 0.0,
            valor_ipi: 0.0,
            valor_pis: 0.0,
            valor_cofins: 0.0,
            outras_despesas: 0.0,
            valor_total,
        };
        
        // Parse data de emissão
        let data_emissao_parsed = DateTime::parse_from_rfc3339(&data_emissao)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());
        
        Ok(NotaFiscal {
            id: Uuid::new_v4(),
            chave_acesso,
            numero,
            serie,
            data_emissao: data_emissao_parsed,
            tipo_nota,
            emitente,
            destinatario,
            itens: Vec::new(),
            totais,
            informacoes_adicionais: None,
            protocolo_autorizacao: None,
            status: ProcessingStatus::Completed,
            created_at: Utc::now(),
        })
    }
}

impl FiscalDocumentParser for NFeParser {
    type Output = NotaFiscal;
    
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

impl Default for NFeParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nfe_parser_basic() {
        let parser = NFeParser::new();
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <nfeProc>
            <NFe>
                <infNFe Id="NFe35210112345678901234567890123456789012345678">
                    <ide>
                        <nNF>12345</nNF>
                        <serie>1</serie>
                        <dhEmi>2021-01-01T10:00:00-03:00</dhEmi>
                    </ide>
                    <emit>
                        <CNPJ>12345678000190</CNPJ>
                        <xNome>Empresa Teste</xNome>
                    </emit>
                    <dest>
                        <CNPJ>98765432000100</CNPJ>
                        <xNome>Cliente Teste</xNome>
                    </dest>
                    <total>
                        <ICMSTot>
                            <vNF>1000.00</vNF>
                            <vProd>900.00</vProd>
                        </ICMSTot>
                    </total>
                </infNFe>
            </NFe>
        </nfeProc>"#;
        
        let result = parser.parse_string(xml);
        assert!(result.is_ok());
        
        let nfe = result.unwrap();
        assert_eq!(nfe.numero, "12345");
        assert_eq!(nfe.serie, "1");
        assert_eq!(nfe.totais.valor_total, 1000.0);
    }
}
