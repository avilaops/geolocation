use crate::database::DatabasePool;
use crate::error::Result;
use crate::models::{ConhecimentoTransporte, DocumentType, NotaFiscal};
use crate::validators::ValidationResult;
use chrono::Utc;
use sqlx::Row;

pub struct Repository {
    pool: DatabasePool,
}

impl Repository {
    pub fn new(pool: DatabasePool) -> Self {
        Repository { pool }
    }

    /// Insere uma Nota Fiscal no banco de dados
    pub async fn insert_nota_fiscal(&self, nf: &NotaFiscal) -> Result<()> {
        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO notas_fiscais (
                        id, chave_acesso, numero, serie, data_emissao, tipo_nota,
                        protocolo_autorizacao, status, created_at,
                        emit_cnpj_cpf, emit_razao_social, emit_nome_fantasia,
                        emit_logradouro, emit_numero, emit_bairro, emit_municipio, emit_uf, emit_cep,
                        dest_cnpj_cpf, dest_razao_social,
                        dest_logradouro, dest_numero, dest_bairro, dest_municipio, dest_uf, dest_cep,
                        valor_produtos, valor_total, valor_icms, valor_ipi, valor_pis, valor_cofins,
                        informacoes_adicionais
                    ) VALUES (
                        ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9,
                        ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18,
                        ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26,
                        ?27, ?28, ?29, ?30, ?31, ?32, ?33
                    )
                    "#,
                )
                .bind(nf.id.to_string())
                .bind(&nf.chave_acesso)
                .bind(&nf.numero)
                .bind(&nf.serie)
                .bind(nf.data_emissao)
                .bind(format!("{:?}", nf.tipo_nota))
                .bind(&nf.protocolo_autorizacao)
                .bind(format!("{:?}", nf.status))
                .bind(nf.created_at)
                .bind(&nf.emitente.cnpj_cpf)
                .bind(&nf.emitente.razao_social)
                .bind(&nf.emitente.nome_fantasia)
                .bind(&nf.emitente.endereco.logradouro)
                .bind(&nf.emitente.endereco.numero)
                .bind(&nf.emitente.endereco.bairro)
                .bind(&nf.emitente.endereco.municipio)
                .bind(&nf.emitente.endereco.uf)
                .bind(&nf.emitente.endereco.cep)
                .bind(&nf.destinatario.cnpj_cpf)
                .bind(&nf.destinatario.razao_social)
                .bind(&nf.destinatario.endereco.logradouro)
                .bind(&nf.destinatario.endereco.numero)
                .bind(&nf.destinatario.endereco.bairro)
                .bind(&nf.destinatario.endereco.municipio)
                .bind(&nf.destinatario.endereco.uf)
                .bind(&nf.destinatario.endereco.cep)
                .bind(nf.totais.valor_produtos)
                .bind(nf.totais.valor_total)
                .bind(nf.totais.valor_icms)
                .bind(nf.totais.valor_ipi)
                .bind(nf.totais.valor_pis)
                .bind(nf.totais.valor_cofins)
                .bind(&nf.informacoes_adicionais)
                .execute(pool)
                .await?;
            }
            DatabasePool::Postgres(pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO notas_fiscais (
                        id, chave_acesso, numero, serie, data_emissao, tipo_nota,
                        protocolo_autorizacao, status, created_at,
                        emit_cnpj_cpf, emit_razao_social, emit_nome_fantasia,
                        emit_logradouro, emit_numero, emit_bairro, emit_municipio, emit_uf, emit_cep,
                        dest_cnpj_cpf, dest_razao_social,
                        dest_logradouro, dest_numero, dest_bairro, dest_municipio, dest_uf, dest_cep,
                        valor_produtos, valor_total, valor_icms, valor_ipi, valor_pis, valor_cofins,
                        informacoes_adicionais
                    ) VALUES (
                        $1, $2, $3, $4, $5, $6, $7, $8, $9,
                        $10, $11, $12, $13, $14, $15, $16, $17, $18,
                        $19, $20, $21, $22, $23, $24, $25, $26,
                        $27, $28, $29, $30, $31, $32, $33
                    )
                    "#,
                )
                .bind(nf.id)
                .bind(&nf.chave_acesso)
                .bind(&nf.numero)
                .bind(&nf.serie)
                .bind(nf.data_emissao)
                .bind(format!("{:?}", nf.tipo_nota))
                .bind(&nf.protocolo_autorizacao)
                .bind(format!("{:?}", nf.status))
                .bind(nf.created_at)
                .bind(&nf.emitente.cnpj_cpf)
                .bind(&nf.emitente.razao_social)
                .bind(&nf.emitente.nome_fantasia)
                .bind(&nf.emitente.endereco.logradouro)
                .bind(&nf.emitente.endereco.numero)
                .bind(&nf.emitente.endereco.bairro)
                .bind(&nf.emitente.endereco.municipio)
                .bind(&nf.emitente.endereco.uf)
                .bind(&nf.emitente.endereco.cep)
                .bind(&nf.destinatario.cnpj_cpf)
                .bind(&nf.destinatario.razao_social)
                .bind(&nf.destinatario.endereco.logradouro)
                .bind(&nf.destinatario.endereco.numero)
                .bind(&nf.destinatario.endereco.bairro)
                .bind(&nf.destinatario.endereco.municipio)
                .bind(&nf.destinatario.endereco.uf)
                .bind(&nf.destinatario.endereco.cep)
                .bind(nf.totais.valor_produtos)
                .bind(nf.totais.valor_total)
                .bind(nf.totais.valor_icms)
                .bind(nf.totais.valor_ipi)
                .bind(nf.totais.valor_pis)
                .bind(nf.totais.valor_cofins)
                .bind(&nf.informacoes_adicionais)
                .execute(pool)
                .await?;
            }
        }
        Ok(())
    }

    /// Busca uma Nota Fiscal pela chave de acesso
    pub async fn find_nota_fiscal_by_chave(&self, chave: &str) -> Result<Option<String>> {
        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                let row = sqlx::query("SELECT id FROM notas_fiscais WHERE chave_acesso = ?")
                    .bind(chave)
                    .fetch_optional(pool)
                    .await?;

                Ok(row.map(|r| r.get("id")))
            }
            DatabasePool::Postgres(pool) => {
                let row = sqlx::query("SELECT id FROM notas_fiscais WHERE chave_acesso = $1")
                    .bind(chave)
                    .fetch_optional(pool)
                    .await?;

                Ok(row.map(|r| r.get("id")))
            }
        }
    }

    /// Insere um Conhecimento de Transporte no banco de dados
    pub async fn insert_cte(&self, cte: &ConhecimentoTransporte) -> Result<()> {
        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO conhecimentos_transporte (
                        id, chave_acesso, numero, serie, data_emissao, tipo_servico, modal,
                        protocolo_autorizacao, status, created_at,
                        emit_cnpj_cpf, emit_razao_social, emit_uf,
                        rem_cnpj_cpf, rem_razao_social, rem_municipio, rem_uf,
                        dest_cnpj_cpf, dest_razao_social, dest_municipio, dest_uf,
                        valor_total, valor_receber, valor_carga, produto_predominante, peso_bruto,
                        informacoes_adicionais
                    ) VALUES (
                        ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
                        ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21,
                        ?22, ?23, ?24, ?25, ?26, ?27
                    )
                    "#,
                )
                .bind(cte.id.to_string())
                .bind(&cte.chave_acesso)
                .bind(&cte.numero)
                .bind(&cte.serie)
                .bind(cte.data_emissao)
                .bind(format!("{:?}", cte.tipo_servico))
                .bind(format!("{:?}", cte.modal))
                .bind(&cte.protocolo_autorizacao)
                .bind(format!("{:?}", cte.status))
                .bind(cte.created_at)
                .bind(&cte.emitente.cnpj_cpf)
                .bind(&cte.emitente.razao_social)
                .bind(&cte.emitente.endereco.uf)
                .bind(&cte.remetente.cnpj_cpf)
                .bind(&cte.remetente.razao_social)
                .bind(&cte.remetente.endereco.municipio)
                .bind(&cte.remetente.endereco.uf)
                .bind(&cte.destinatario.cnpj_cpf)
                .bind(&cte.destinatario.razao_social)
                .bind(&cte.destinatario.endereco.municipio)
                .bind(&cte.destinatario.endereco.uf)
                .bind(cte.valores_prestacao.valor_total)
                .bind(cte.valores_prestacao.valor_receber)
                .bind(cte.informacoes_carga.valor_carga)
                .bind(&cte.informacoes_carga.produto_predominante)
                .bind(cte.informacoes_carga.peso_bruto)
                .bind(&cte.informacoes_adicionais)
                .execute(pool)
                .await?;
            }
            DatabasePool::Postgres(pool) => {
                sqlx::query(
                    r#"
                    INSERT INTO conhecimentos_transporte (
                        id, chave_acesso, numero, serie, data_emissao, tipo_servico, modal,
                        protocolo_autorizacao, status, created_at,
                        emit_cnpj_cpf, emit_razao_social, emit_uf,
                        rem_cnpj_cpf, rem_razao_social, rem_municipio, rem_uf,
                        dest_cnpj_cpf, dest_razao_social, dest_municipio, dest_uf,
                        valor_total, valor_receber, valor_carga, produto_predominante, peso_bruto,
                        informacoes_adicionais
                    ) VALUES (
                        $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
                        $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21,
                        $22, $23, $24, $25, $26, $27
                    )
                    "#,
                )
                .bind(cte.id)
                .bind(&cte.chave_acesso)
                .bind(&cte.numero)
                .bind(&cte.serie)
                .bind(cte.data_emissao)
                .bind(format!("{:?}", cte.tipo_servico))
                .bind(format!("{:?}", cte.modal))
                .bind(&cte.protocolo_autorizacao)
                .bind(format!("{:?}", cte.status))
                .bind(cte.created_at)
                .bind(&cte.emitente.cnpj_cpf)
                .bind(&cte.emitente.razao_social)
                .bind(&cte.emitente.endereco.uf)
                .bind(&cte.remetente.cnpj_cpf)
                .bind(&cte.remetente.razao_social)
                .bind(&cte.remetente.endereco.municipio)
                .bind(&cte.remetente.endereco.uf)
                .bind(&cte.destinatario.cnpj_cpf)
                .bind(&cte.destinatario.razao_social)
                .bind(&cte.destinatario.endereco.municipio)
                .bind(&cte.destinatario.endereco.uf)
                .bind(cte.valores_prestacao.valor_total)
                .bind(cte.valores_prestacao.valor_receber)
                .bind(cte.informacoes_carga.valor_carga)
                .bind(&cte.informacoes_carga.produto_predominante)
                .bind(cte.informacoes_carga.peso_bruto)
                .bind(&cte.informacoes_adicionais)
                .execute(pool)
                .await?;
            }
        }
        Ok(())
    }
    /// Lista documentos (NF-e e/ou CT-e) com paginação e filtro opcional por tipo.
    pub async fn list_documents(
        &self,
        doc_type: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<DocumentSummary>> {
        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                if let Some(t) = doc_type {
                    if t == "NFe" {
                        let rows = sqlx::query(
                            r#"SELECT 'NFe' as document_type, chave_acesso, numero, serie, data_emissao,
                                   emit_razao_social, dest_razao_social, valor_total
                                FROM notas_fiscais ORDER BY data_emissao DESC LIMIT ?1 OFFSET ?2"#,
                        )
                        .bind(limit)
                        .bind(offset)
                        .fetch_all(pool)
                        .await?;
                        return Ok(rows
                            .into_iter()
                            .map(|r| DocumentSummary::from_nf_row(&r))
                            .collect());
                    } else if t == "CTe" {
                        let rows = sqlx::query(
                            r#"SELECT 'CTe' as document_type, chave_acesso, numero, serie, data_emissao,
                                   emit_razao_social, dest_razao_social, valor_total
                                FROM conhecimentos_transporte ORDER BY data_emissao DESC LIMIT ?1 OFFSET ?2"#,
                        )
                        .bind(limit)
                        .bind(offset)
                        .fetch_all(pool)
                        .await?;
                        return Ok(rows
                            .into_iter()
                            .map(|r| DocumentSummary::from_cte_row(&r))
                            .collect());
                    }
                }
                let rows = sqlx::query(
                    r#"SELECT * FROM (
                            SELECT 'NFe' as document_type, chave_acesso, numero, serie, data_emissao,
                                   emit_razao_social, dest_razao_social, valor_total
                              FROM notas_fiscais
                            UNION ALL
                            SELECT 'CTe' as document_type, chave_acesso, numero, serie, data_emissao,
                                   emit_razao_social, dest_razao_social, valor_total
                              FROM conhecimentos_transporte
                        ) ORDER BY data_emissao DESC LIMIT ?1 OFFSET ?2"#,
                )
                .bind(limit)
                .bind(offset)
                .fetch_all(pool)
                .await?;
                Ok(rows
                    .into_iter()
                    .map(|r| {
                        let t: String = r.get("document_type");
                        if t == "NFe" {
                            DocumentSummary::from_nf_row(&r)
                        } else {
                            DocumentSummary::from_cte_row(&r)
                        }
                    })
                    .collect())
            }
            DatabasePool::Postgres(pool) => {
                if let Some(t) = doc_type {
                    if t == "NFe" {
                        let rows = sqlx::query(
                            r#"SELECT 'NFe' as document_type, chave_acesso, numero, serie, data_emissao::text as data_emissao,
                                   emit_razao_social, dest_razao_social, valor_total
                                FROM notas_fiscais ORDER BY data_emissao DESC LIMIT $1 OFFSET $2"#,
                        )
                        .bind(limit)
                        .bind(offset)
                        .fetch_all(pool)
                        .await?;
                        return Ok(rows
                            .into_iter()
                            .map(|r| DocumentSummary::from_nf_row_pg(&r))
                            .collect());
                    } else if t == "CTe" {
                        let rows = sqlx::query(
                            r#"SELECT 'CTe' as document_type, chave_acesso, numero, serie, data_emissao::text as data_emissao,
                                   emit_razao_social, dest_razao_social, valor_total
                                FROM conhecimentos_transporte ORDER BY data_emissao DESC LIMIT $1 OFFSET $2"#,
                        )
                        .bind(limit)
                        .bind(offset)
                        .fetch_all(pool)
                        .await?;
                        return Ok(rows
                            .into_iter()
                            .map(|r| DocumentSummary::from_cte_row_pg(&r))
                            .collect());
                    }
                }
                let rows = sqlx::query(
                    r#"SELECT * FROM (
                            SELECT 'NFe' as document_type, chave_acesso, numero, serie, data_emissao::text as data_emissao,
                                   emit_razao_social, dest_razao_social, valor_total FROM notas_fiscais
                            UNION ALL
                            SELECT 'CTe' as document_type, chave_acesso, numero, serie, data_emissao::text as data_emissao,
                                   emit_razao_social, dest_razao_social, valor_total FROM conhecimentos_transporte
                        ) t ORDER BY data_emissao DESC LIMIT $1 OFFSET $2"#,
                )
                .bind(limit)
                .bind(offset)
                .fetch_all(pool)
                .await?;
                Ok(rows
                    .into_iter()
                    .map(|r| {
                        let t: String = r.get("document_type");
                        if t == "NFe" {
                            DocumentSummary::from_nf_row_pg(&r)
                        } else {
                            DocumentSummary::from_cte_row_pg(&r)
                        }
                    })
                    .collect())
            }
        }
    }

    /// Busca um CT-e pela chave de acesso
    pub async fn find_cte_by_chave(&self, chave: &str) -> Result<Option<String>> {
        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                let row =
                    sqlx::query("SELECT id FROM conhecimentos_transporte WHERE chave_acesso = ?")
                        .bind(chave)
                        .fetch_optional(pool)
                        .await?;

                Ok(row.map(|r| r.get("id")))
            }
            DatabasePool::Postgres(pool) => {
                let row =
                    sqlx::query("SELECT id FROM conhecimentos_transporte WHERE chave_acesso = $1")
                        .bind(chave)
                        .fetch_optional(pool)
                        .await?;

                Ok(row.map(|r| r.get("id")))
            }
        }
    }

    /// Retorna contagem de notas fiscais
    pub async fn count_notas_fiscais(&self) -> Result<i64> {
        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM notas_fiscais")
                    .fetch_one(pool)
                    .await?;
                Ok(count)
            }
            DatabasePool::Postgres(pool) => {
                let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM notas_fiscais")
                    .fetch_one(pool)
                    .await?;
                Ok(count)
            }
        }
    }

    /// Retorna contagem de CT-es
    pub async fn count_ctes(&self) -> Result<i64> {
        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                let count =
                    sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM conhecimentos_transporte")
                        .fetch_one(pool)
                        .await?;
                Ok(count)
            }
            DatabasePool::Postgres(pool) => {
                let count =
                    sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM conhecimentos_transporte")
                        .fetch_one(pool)
                        .await?;
                Ok(count)
            }
        }
    }

    /// Retorna quantidade de documentos processados hoje (data_emissao na data corrente)
    pub async fn count_processed_today(&self) -> Result<i64> {
        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                let nf_today = sqlx::query_scalar::<_, i64>(
                    "SELECT COUNT(*) FROM notas_fiscais WHERE date(data_emissao) = date('now')",
                )
                .fetch_one(pool)
                .await?;
                let cte_today = sqlx::query_scalar::<_, i64>(
                    "SELECT COUNT(*) FROM conhecimentos_transporte WHERE date(data_emissao) = date('now')"
                )
                .fetch_one(pool)
                .await?;
                Ok(nf_today + cte_today)
            }
            DatabasePool::Postgres(pool) => {
                let nf_today = sqlx::query_scalar::<_, i64>(
                    "SELECT COUNT(*) FROM notas_fiscais WHERE date(data_emissao) = CURRENT_DATE",
                )
                .fetch_one(pool)
                .await?;
                let cte_today = sqlx::query_scalar::<_, i64>(
                    "SELECT COUNT(*) FROM conhecimentos_transporte WHERE date(data_emissao) = CURRENT_DATE"
                )
                .fetch_one(pool)
                .await?;
                Ok(nf_today + cte_today)
            }
        }
    }

    /// Insere resultado de validação fiscal
    pub async fn insert_validation(
        &self,
        chave_acesso: &str,
        document_type: &DocumentType,
        validation: &ValidationResult,
    ) -> Result<()> {
        let is_valid = validation.is_valid;
        let json = serde_json::to_string(validation)?;
        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                sqlx::query(
                    r#"INSERT INTO validacoes (chave_acesso, document_type, is_valid, validation_json, created_at)
                       VALUES (?1, ?2, ?3, ?4, datetime('now'))"#
                )
                .bind(chave_acesso)
                .bind(document_type.to_string())
                .bind(is_valid as i32)
                .bind(json)
                .execute(pool)
                .await?;
            }
            DatabasePool::Postgres(pool) => {
                sqlx::query(
                    r#"INSERT INTO validacoes (chave_acesso, document_type, is_valid, validation_json, created_at)
                       VALUES ($1, $2, $3, $4, NOW())"#
                )
                .bind(chave_acesso)
                .bind(document_type.to_string())
                .bind(is_valid)
                .bind(json)
                .execute(pool)
                .await?;
            }
        }
        Ok(())
    }

    /// Busca validação por chave
    pub async fn find_validation(&self, chave_acesso: &str) -> Result<Option<ValidationResult>> {
        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                let row = sqlx::query("SELECT validation_json FROM validacoes WHERE chave_acesso = ? ORDER BY id DESC LIMIT 1")
                    .bind(chave_acesso)
                    .fetch_optional(pool)
                    .await?;
                Ok(row.map(|r| {
                    let json: String = r.get("validation_json");
                    serde_json::from_str(&json).unwrap_or_else(|_| ValidationResult {
                        chave_acesso: chave_acesso.to_string(),
                        document_type: "unknown".to_string(),
                        is_valid: true,
                        errors: vec![],
                        warnings: vec![],
                        suggestions: vec![],
                        validated_at: Utc::now(),
                    })
                }))
            }
            DatabasePool::Postgres(pool) => {
                let row = sqlx::query("SELECT validation_json FROM validacoes WHERE chave_acesso = $1 ORDER BY id DESC LIMIT 1")
                    .bind(chave_acesso)
                    .fetch_optional(pool)
                    .await?;
                Ok(row.map(|r| {
                    let json: String = r.get("validation_json");
                    serde_json::from_str(&json).unwrap_or_else(|_| ValidationResult {
                        chave_acesso: chave_acesso.to_string(),
                        document_type: "unknown".to_string(),
                        is_valid: true,
                        errors: vec![],
                        warnings: vec![],
                        suggestions: vec![],
                        validated_at: Utc::now(),
                    })
                }))
            }
        }
    }

    /// Busca resumo de documento (NF-e ou CT-e) pela chave
    pub async fn find_document_summary(&self, chave: &str) -> Result<Option<DocumentSummary>> {
        // Tenta NF-e
        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                if let Some(row) = sqlx::query(
                    r#"SELECT chave_acesso, numero, serie, data_emissao, emit_razao_social, dest_razao_social, valor_total
                        FROM notas_fiscais WHERE chave_acesso = ?"#
                )
                .bind(chave)
                .fetch_optional(pool)
                .await? {
                    return Ok(Some(DocumentSummary::from_nf_row(&row)));
                }
                if let Some(row) = sqlx::query(
                    r#"SELECT chave_acesso, numero, serie, data_emissao, emit_razao_social, dest_razao_social, valor_total
                        FROM conhecimentos_transporte WHERE chave_acesso = ?"#
                )
                .bind(chave)
                .fetch_optional(pool)
                .await? {
                    return Ok(Some(DocumentSummary::from_cte_row(&row)));
                }
                Ok(None)
            }
            DatabasePool::Postgres(pool) => {
                if let Some(row) = sqlx::query(
                    r#"SELECT chave_acesso, numero, serie, data_emissao, emit_razao_social, dest_razao_social, valor_total
                        FROM notas_fiscais WHERE chave_acesso = $1"#
                )
                .bind(chave)
                .fetch_optional(pool)
                .await? {
                    return Ok(Some(DocumentSummary::from_nf_row_pg(&row)));
                }
                if let Some(row) = sqlx::query(
                    r#"SELECT chave_acesso, numero, serie, data_emissao, emit_razao_social, dest_razao_social, valor_total
                        FROM conhecimentos_transporte WHERE chave_acesso = $1"#
                )
                .bind(chave)
                .fetch_optional(pool)
                .await? {
                    return Ok(Some(DocumentSummary::from_cte_row_pg(&row)));
                }
                Ok(None)
            }
        }
    }

    /// Retorna estatísticas agregadas
    pub async fn stats(&self) -> Result<Stats> {
        let (nf, cte, today) = tokio::try_join!(
            self.count_notas_fiscais(),
            self.count_ctes(),
            self.count_processed_today()
        )?;
        Ok(Stats {
            notas_fiscais: nf,
            ctes: cte,
            processed_today: today,
        })
    }
}

/// Estrutura agregada de estatísticas
pub struct Stats {
    pub notas_fiscais: i64,
    pub ctes: i64,
    pub processed_today: i64,
}

/// Resumo de documento retornado pela API
#[derive(Debug, Clone, serde::Serialize)]
pub struct DocumentSummary {
    pub document_type: String,
    pub chave_acesso: String,
    pub numero: String,
    pub serie: String,
    pub data_emissao: String,
    pub emitente: String,
    pub destinatario: String,
    pub valor_total: f64,
}

impl DocumentSummary {
    fn from_nf_row(row: &sqlx::sqlite::SqliteRow) -> Self {
        // For SQLite rows
        DocumentSummary {
            document_type: "NFe".to_string(),
            chave_acesso: row.get("chave_acesso"),
            numero: row.get("numero"),
            serie: row.get("serie"),
            data_emissao: row.get::<String, _>("data_emissao"),
            emitente: row.get("emit_razao_social"),
            destinatario: row.get("dest_razao_social"),
            valor_total: row.get("valor_total"),
        }
    }
    fn from_cte_row(row: &sqlx::sqlite::SqliteRow) -> Self {
        DocumentSummary {
            document_type: "CTe".to_string(),
            chave_acesso: row.get("chave_acesso"),
            numero: row.get("numero"),
            serie: row.get("serie"),
            data_emissao: row.get::<String, _>("data_emissao"),
            emitente: row.get("emit_razao_social"),
            destinatario: row.get("dest_razao_social"),
            valor_total: row.get("valor_total"),
        }
    }

    fn from_nf_row_pg(row: &sqlx::postgres::PgRow) -> Self {
        // For PostgreSQL rows
        use sqlx::Row;
        DocumentSummary {
            document_type: "NFe".to_string(),
            chave_acesso: row.get("chave_acesso"),
            numero: row.get("numero"),
            serie: row.get("serie"),
            data_emissao: row.get::<String, _>("data_emissao"),
            emitente: row.get("emit_razao_social"),
            destinatario: row.get("dest_razao_social"),
            valor_total: row.get("valor_total"),
        }
    }

    fn from_cte_row_pg(row: &sqlx::postgres::PgRow) -> Self {
        use sqlx::Row;
        DocumentSummary {
            document_type: "CTe".to_string(),
            chave_acesso: row.get("chave_acesso"),
            numero: row.get("numero"),
            serie: row.get("serie"),
            data_emissao: row.get::<String, _>("data_emissao"),
            emitente: row.get("emit_razao_social"),
            destinatario: row.get("dest_razao_social"),
            valor_total: row.get("valor_total"),
        }
    }
}

