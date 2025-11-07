use lazy_static::lazy_static;
use prometheus::{Encoder, IntCounter, Registry, TextEncoder};

lazy_static! {
    pub static ref METRICS_REGISTRY: Registry = Registry::new();
    pub static ref DOCS_PROCESSED: IntCounter = IntCounter::new(
        "documents_processed_total",
        "Total de documentos processados"
    )
    .unwrap();
    pub static ref DOCS_DUPLICATE: IntCounter = IntCounter::new(
        "documents_duplicate_total",
        "Total de documentos duplicados detectados"
    )
    .unwrap();
    pub static ref VALIDATIONS_SAVED: IntCounter =
        IntCounter::new("validations_saved_total", "Total de validações persistidas").unwrap();
}

pub fn register_metrics() {
    let _ = METRICS_REGISTRY.register(Box::new(DOCS_PROCESSED.clone()));
    let _ = METRICS_REGISTRY.register(Box::new(DOCS_DUPLICATE.clone()));
    let _ = METRICS_REGISTRY.register(Box::new(VALIDATIONS_SAVED.clone()));
}

pub fn docs_processed_counter() -> Option<&'static IntCounter> {
    Some(&DOCS_PROCESSED)
}
pub fn docs_duplicate_counter() -> Option<&'static IntCounter> {
    Some(&DOCS_DUPLICATE)
}
pub fn validations_saved_counter() -> Option<&'static IntCounter> {
    Some(&VALIDATIONS_SAVED)
}

pub fn gather_metrics() -> Result<String, String> {
    let metric_families = METRICS_REGISTRY.gather();
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&metric_families, &mut buffer) {
        return Err(format!("Erro ao codificar métricas: {}", e));
    }
    String::from_utf8(buffer).map_err(|e| format!("Erro UTF-8 métricas: {}", e))
}
