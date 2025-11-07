use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use crate::parsers::{parse_nfe, parse_cte};

#[cfg(feature = "wasm")]
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[derive(Serialize, Deserialize)]
pub struct WasmParseResult {
    pub success: bool,
    pub document_type: String,
    pub chave_acesso: String,
    pub numero: String,
    pub serie: String,
    pub data_emissao: String,
    pub emitente_nome: String,
    pub emitente_cnpj: String,
    pub destinatario_nome: String,
    pub destinatario_cnpj: String,
    pub valor_total: f64,
    pub error: Option<String>,
}

/// Parse NF-e XML content and return structured data
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn parse_nfe_wasm(xml_content: &str) -> JsValue {
    let result = match parse_nfe(xml_content) {
        Ok(nfe) => WasmParseResult {
            success: true,
            document_type: "NFe".to_string(),
            chave_acesso: nfe.chave_acesso.clone(),
            numero: nfe.numero.clone(),
            serie: nfe.serie.clone(),
            data_emissao: nfe.data_emissao.to_string(),
            emitente_nome: nfe.emitente.nome.clone(),
            emitente_cnpj: nfe.emitente.cnpj.clone(),
            destinatario_nome: nfe.destinatario.nome.clone(),
            destinatario_cnpj: nfe.destinatario.cnpj.clone(),
            valor_total: nfe.totais.valor_total,
            error: None,
        },
        Err(e) => WasmParseResult {
            success: false,
            document_type: "NFe".to_string(),
            chave_acesso: String::new(),
            numero: String::new(),
            serie: String::new(),
            data_emissao: String::new(),
            emitente_nome: String::new(),
            emitente_cnpj: String::new(),
            destinatario_nome: String::new(),
            destinatario_cnpj: String::new(),
            valor_total: 0.0,
            error: Some(e.to_string()),
        },
    };

    serde_wasm_bindgen::to_value(&result).unwrap()
}

/// Parse CT-e XML content and return structured data
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn parse_cte_wasm(xml_content: &str) -> JsValue {
    let result = match parse_cte(xml_content) {
        Ok(cte) => WasmParseResult {
            success: true,
            document_type: "CTe".to_string(),
            chave_acesso: cte.chave_acesso.clone(),
            numero: cte.numero.clone(),
            serie: cte.serie.clone(),
            data_emissao: cte.data_emissao.to_string(),
            emitente_nome: cte.emitente.nome.clone(),
            emitente_cnpj: cte.emitente.cnpj.clone(),
            destinatario_nome: cte.destinatario.nome.clone(),
            destinatario_cnpj: cte.destinatario.cnpj.clone(),
            valor_total: cte.valores_prestacao.valor_total_prestacao,
            error: None,
        },
        Err(e) => WasmParseResult {
            success: false,
            document_type: "CTe".to_string(),
            chave_acesso: String::new(),
            numero: String::new(),
            serie: String::new(),
            data_emissao: String::new(),
            emitente_nome: String::new(),
            emitente_cnpj: String::new(),
            destinatario_nome: String::new(),
            destinatario_cnpj: String::new(),
            valor_total: 0.0,
            error: Some(e.to_string()),
        },
    };

    serde_wasm_bindgen::to_value(&result).unwrap()
}

/// Auto-detect document type and parse
#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn parse_document_wasm(xml_content: &str) -> JsValue {
    if xml_content.contains("<nfeProc") || xml_content.contains("<NFe") {
        parse_nfe_wasm(xml_content)
    } else if xml_content.contains("<cteProc") || xml_content.contains("<CTe") {
        parse_cte_wasm(xml_content)
    } else {
        let result = WasmParseResult {
            success: false,
            document_type: "Unknown".to_string(),
            chave_acesso: String::new(),
            numero: String::new(),
            serie: String::new(),
            data_emissao: String::new(),
            emitente_nome: String::new(),
            emitente_cnpj: String::new(),
            destinatario_nome: String::new(),
            destinatario_cnpj: String::new(),
            valor_total: 0.0,
            error: Some("Tipo de documento não reconhecido".to_string()),
        };
        serde_wasm_bindgen::to_value(&result).unwrap()
    }
}
