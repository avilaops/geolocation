use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Tipos de documentos fiscais suportados
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DocumentType {
    #[serde(rename = "NFe")]
    NotaFiscal,
    #[serde(rename = "CTe")]
    ConhecimentoTransporte,
}

/// Status do processamento do documento
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProcessingStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

/// Estrutura base para Nota Fiscal Eletrônica (NF-e)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotaFiscal {
    pub id: Uuid,
    pub chave_acesso: String,
    pub numero: String,
    pub serie: String,
    pub data_emissao: DateTime<Utc>,
    pub tipo_nota: TipoNota,
    pub emitente: Participante,
    pub destinatario: Participante,
    pub itens: Vec<ItemNota>,
    pub totais: Totais,
    pub informacoes_adicionais: Option<String>,
    pub protocolo_autorizacao: Option<String>,
    pub status: ProcessingStatus,
    pub created_at: DateTime<Utc>,
}

/// Tipo de Nota Fiscal
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TipoNota {
    Entrada,
    Saida,
}

/// Participante (Emitente ou Destinatário)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participante {
    pub cnpj_cpf: String,
    pub razao_social: String,
    pub nome_fantasia: Option<String>,
    pub endereco: Endereco,
    pub inscricao_estadual: Option<String>,
    pub telefone: Option<String>,
    pub email: Option<String>,
}

/// Endereço
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Endereco {
    pub logradouro: String,
    pub numero: String,
    pub complemento: Option<String>,
    pub bairro: String,
    pub codigo_municipio: String,
    pub municipio: String,
    pub uf: String,
    pub cep: String,
    pub codigo_pais: String,
    pub pais: String,
}

/// Item da Nota Fiscal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemNota {
    pub numero_item: i32,
    pub codigo_produto: String,
    pub descricao: String,
    pub ncm: String,
    pub cfop: String,
    pub unidade_comercial: String,
    pub quantidade_comercial: f64,
    pub valor_unitario: f64,
    pub valor_total: f64,
    pub ean: Option<String>,
    pub informacoes_adicionais: Option<String>,
}

/// Totais da Nota Fiscal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Totais {
    pub base_calculo_icms: f64,
    pub valor_icms: f64,
    pub valor_icms_desonerado: f64,
    pub valor_fcp: f64,
    pub base_calculo_icms_st: f64,
    pub valor_icms_st: f64,
    pub valor_produtos: f64,
    pub valor_frete: f64,
    pub valor_seguro: f64,
    pub valor_desconto: f64,
    pub valor_ii: f64,
    pub valor_ipi: f64,
    pub valor_pis: f64,
    pub valor_cofins: f64,
    pub outras_despesas: f64,
    pub valor_total: f64,
}

/// Estrutura para Conhecimento de Transporte Eletrônico (CT-e)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConhecimentoTransporte {
    pub id: Uuid,
    pub chave_acesso: String,
    pub numero: String,
    pub serie: String,
    pub data_emissao: DateTime<Utc>,
    pub tipo_servico: TipoServicoCTe,
    pub emitente: Participante,
    pub remetente: Participante,
    pub destinatario: Participante,
    pub expedidor: Option<Participante>,
    pub recebedor: Option<Participante>,
    pub valores_prestacao: ValoresPrestacaoCTe,
    pub informacoes_carga: InformacoesCarga,
    pub documentos_referenciados: Vec<DocumentoReferenciado>,
    pub modal: Modal,
    pub informacoes_adicionais: Option<String>,
    pub protocolo_autorizacao: Option<String>,
    pub status: ProcessingStatus,
    pub created_at: DateTime<Utc>,
}

/// Tipo de Serviço do CT-e
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TipoServicoCTe {
    Normal,
    Subcontratacao,
    Redespacho,
    RedespachIntermediario,
    ServicoVinculadoMultimodal,
}

/// Valores da Prestação do Serviço
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValoresPrestacaoCTe {
    pub valor_total: f64,
    pub valor_receber: f64,
    pub valor_total_carga: f64,
    pub produto_predominante: String,
    pub outras_caracteristicas_carga: Option<String>,
}

/// Informações da Carga
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InformacoesCarga {
    pub valor_carga: f64,
    pub produto_predominante: String,
    pub peso_bruto: f64,
    pub peso_cubado: Option<f64>,
    pub quantidades: Vec<QuantidadeCarga>,
}

/// Quantidade de Carga
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuantidadeCarga {
    pub codigo_unidade: String,
    pub tipo_medida: String,
    pub quantidade: f64,
}

/// Documento Referenciado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentoReferenciado {
    pub tipo: TipoDocumentoReferenciado,
    pub chave_acesso: Option<String>,
    pub numero: Option<String>,
    pub serie: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TipoDocumentoReferenciado {
    NotaFiscal,
    NotaFiscalProdutor,
    OutrosDocumentos,
}

/// Modal de Transporte
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Modal {
    Rodoviario,
    Aereo,
    Aquaviario,
    Ferroviario,
    Dutoviario,
}

impl Default for ProcessingStatus {
    fn default() -> Self {
        ProcessingStatus::Pending
    }
}

impl std::fmt::Display for DocumentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DocumentType::NotaFiscal => write!(f, "NF-e"),
            DocumentType::ConhecimentoTransporte => write!(f, "CT-e"),
        }
    }
}
