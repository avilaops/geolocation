use crate::error::Result;
use sqlx::postgres::PgPool;
use sqlx::{Pool, Sqlite};

/// Cria as tabelas para SQLite
pub async fn create_tables_sqlite(pool: &Pool<Sqlite>) -> Result<()> {
    // Tabela de Notas Fiscais
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS notas_fiscais (
            id TEXT PRIMARY KEY,
            chave_acesso TEXT UNIQUE NOT NULL,
            numero TEXT NOT NULL,
            serie TEXT NOT NULL,
            data_emissao DATETIME NOT NULL,
            tipo_nota TEXT NOT NULL,
            protocolo_autorizacao TEXT,
            status TEXT NOT NULL,
            created_at DATETIME NOT NULL,
            
            -- Emitente
            emit_cnpj_cpf TEXT NOT NULL,
            emit_razao_social TEXT NOT NULL,
            emit_nome_fantasia TEXT,
            emit_logradouro TEXT,
            emit_numero TEXT,
            emit_bairro TEXT,
            emit_municipio TEXT,
            emit_uf TEXT,
            emit_cep TEXT,
            
            -- Destinatário
            dest_cnpj_cpf TEXT NOT NULL,
            dest_razao_social TEXT NOT NULL,
            dest_logradouro TEXT,
            dest_numero TEXT,
            dest_bairro TEXT,
            dest_municipio TEXT,
            dest_uf TEXT,
            dest_cep TEXT,
            
            -- Totais
            valor_produtos REAL NOT NULL,
            valor_total REAL NOT NULL,
            valor_icms REAL,
            valor_ipi REAL,
            valor_pis REAL,
            valor_cofins REAL,
            
            informacoes_adicionais TEXT
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Tabela de Itens de Nota Fiscal
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS itens_nota_fiscal (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nota_fiscal_id TEXT NOT NULL,
            numero_item INTEGER NOT NULL,
            codigo_produto TEXT NOT NULL,
            descricao TEXT NOT NULL,
            ncm TEXT NOT NULL,
            cfop TEXT NOT NULL,
            unidade_comercial TEXT NOT NULL,
            quantidade_comercial REAL NOT NULL,
            valor_unitario REAL NOT NULL,
            valor_total REAL NOT NULL,
            ean TEXT,
            informacoes_adicionais TEXT,
            
            FOREIGN KEY (nota_fiscal_id) REFERENCES notas_fiscais(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Tabela de Conhecimentos de Transporte
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS conhecimentos_transporte (
            id TEXT PRIMARY KEY,
            chave_acesso TEXT UNIQUE NOT NULL,
            numero TEXT NOT NULL,
            serie TEXT NOT NULL,
            data_emissao DATETIME NOT NULL,
            tipo_servico TEXT NOT NULL,
            modal TEXT NOT NULL,
            protocolo_autorizacao TEXT,
            status TEXT NOT NULL,
            created_at DATETIME NOT NULL,
            
            -- Emitente
            emit_cnpj_cpf TEXT NOT NULL,
            emit_razao_social TEXT NOT NULL,
            emit_uf TEXT,
            
            -- Remetente
            rem_cnpj_cpf TEXT NOT NULL,
            rem_razao_social TEXT NOT NULL,
            rem_municipio TEXT,
            rem_uf TEXT,
            
            -- Destinatário
            dest_cnpj_cpf TEXT NOT NULL,
            dest_razao_social TEXT NOT NULL,
            dest_municipio TEXT,
            dest_uf TEXT,
            
            -- Valores
            valor_total REAL NOT NULL,
            valor_receber REAL NOT NULL,
            valor_carga REAL NOT NULL,
            produto_predominante TEXT NOT NULL,
            peso_bruto REAL NOT NULL,
            
            informacoes_adicionais TEXT
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Tabela de Documentos Referenciados
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS documentos_referenciados (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            cte_id TEXT NOT NULL,
            tipo TEXT NOT NULL,
            chave_acesso TEXT,
            numero TEXT,
            serie TEXT,
            
            FOREIGN KEY (cte_id) REFERENCES conhecimentos_transporte(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Índices para otimização
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_nf_chave ON notas_fiscais(chave_acesso)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_nf_data ON notas_fiscais(data_emissao)")
        .execute(pool)
        .await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_cte_chave ON conhecimentos_transporte(chave_acesso)",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_cte_data ON conhecimentos_transporte(data_emissao)",
    )
    .execute(pool)
    .await?;

    // Tabela de validações fiscais (armazenamento simples em JSON)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS validacoes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            chave_acesso TEXT NOT NULL,
            document_type TEXT NOT NULL,
            is_valid INTEGER NOT NULL,
            validation_json TEXT NOT NULL,
            created_at DATETIME NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_validacoes_chave ON validacoes(chave_acesso)")
        .execute(pool)
        .await?;

    Ok(())
}

/// Cria as tabelas para PostgreSQL
pub async fn create_tables_postgres(pool: &PgPool) -> Result<()> {
    // Tabela de Notas Fiscais
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS notas_fiscais (
            id UUID PRIMARY KEY,
            chave_acesso VARCHAR(44) UNIQUE NOT NULL,
            numero VARCHAR(20) NOT NULL,
            serie VARCHAR(10) NOT NULL,
            data_emissao TIMESTAMPTZ NOT NULL,
            tipo_nota VARCHAR(20) NOT NULL,
            protocolo_autorizacao VARCHAR(50),
            status VARCHAR(20) NOT NULL,
            created_at TIMESTAMPTZ NOT NULL,
            
            -- Emitente
            emit_cnpj_cpf VARCHAR(14) NOT NULL,
            emit_razao_social VARCHAR(200) NOT NULL,
            emit_nome_fantasia VARCHAR(200),
            emit_logradouro VARCHAR(200),
            emit_numero VARCHAR(20),
            emit_bairro VARCHAR(100),
            emit_municipio VARCHAR(100),
            emit_uf VARCHAR(2),
            emit_cep VARCHAR(8),
            
            -- Destinatário
            dest_cnpj_cpf VARCHAR(14) NOT NULL,
            dest_razao_social VARCHAR(200) NOT NULL,
            dest_logradouro VARCHAR(200),
            dest_numero VARCHAR(20),
            dest_bairro VARCHAR(100),
            dest_municipio VARCHAR(100),
            dest_uf VARCHAR(2),
            dest_cep VARCHAR(8),
            
            -- Totais
            valor_produtos NUMERIC(15,2) NOT NULL,
            valor_total NUMERIC(15,2) NOT NULL,
            valor_icms NUMERIC(15,2),
            valor_ipi NUMERIC(15,2),
            valor_pis NUMERIC(15,2),
            valor_cofins NUMERIC(15,2),
            
            informacoes_adicionais TEXT
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Tabela de Itens de Nota Fiscal
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS itens_nota_fiscal (
            id SERIAL PRIMARY KEY,
            nota_fiscal_id UUID NOT NULL,
            numero_item INTEGER NOT NULL,
            codigo_produto VARCHAR(60) NOT NULL,
            descricao VARCHAR(500) NOT NULL,
            ncm VARCHAR(8) NOT NULL,
            cfop VARCHAR(4) NOT NULL,
            unidade_comercial VARCHAR(6) NOT NULL,
            quantidade_comercial NUMERIC(15,4) NOT NULL,
            valor_unitario NUMERIC(15,4) NOT NULL,
            valor_total NUMERIC(15,2) NOT NULL,
            ean VARCHAR(14),
            informacoes_adicionais TEXT,
            
            FOREIGN KEY (nota_fiscal_id) REFERENCES notas_fiscais(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Tabela de Conhecimentos de Transporte
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS conhecimentos_transporte (
            id UUID PRIMARY KEY,
            chave_acesso VARCHAR(44) UNIQUE NOT NULL,
            numero VARCHAR(20) NOT NULL,
            serie VARCHAR(10) NOT NULL,
            data_emissao TIMESTAMPTZ NOT NULL,
            tipo_servico VARCHAR(50) NOT NULL,
            modal VARCHAR(20) NOT NULL,
            protocolo_autorizacao VARCHAR(50),
            status VARCHAR(20) NOT NULL,
            created_at TIMESTAMPTZ NOT NULL,
            
            -- Emitente
            emit_cnpj_cpf VARCHAR(14) NOT NULL,
            emit_razao_social VARCHAR(200) NOT NULL,
            emit_uf VARCHAR(2),
            
            -- Remetente
            rem_cnpj_cpf VARCHAR(14) NOT NULL,
            rem_razao_social VARCHAR(200) NOT NULL,
            rem_municipio VARCHAR(100),
            rem_uf VARCHAR(2),
            
            -- Destinatário
            dest_cnpj_cpf VARCHAR(14) NOT NULL,
            dest_razao_social VARCHAR(200) NOT NULL,
            dest_municipio VARCHAR(100),
            dest_uf VARCHAR(2),
            
            -- Valores
            valor_total NUMERIC(15,2) NOT NULL,
            valor_receber NUMERIC(15,2) NOT NULL,
            valor_carga NUMERIC(15,2) NOT NULL,
            produto_predominante VARCHAR(200) NOT NULL,
            peso_bruto NUMERIC(15,3) NOT NULL,
            
            informacoes_adicionais TEXT
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Índices para otimização
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_nf_chave ON notas_fiscais(chave_acesso)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_nf_data ON notas_fiscais(data_emissao)")
        .execute(pool)
        .await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_cte_chave ON conhecimentos_transporte(chave_acesso)",
    )
    .execute(pool)
    .await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_cte_data ON conhecimentos_transporte(data_emissao)",
    )
    .execute(pool)
    .await?;

    // Tabela de validações fiscais (Postgres)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS validacoes (
            id SERIAL PRIMARY KEY,
            chave_acesso VARCHAR(44) NOT NULL,
            document_type VARCHAR(10) NOT NULL,
            is_valid BOOLEAN NOT NULL,
            validation_json TEXT NOT NULL,
            created_at TIMESTAMPTZ NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_validacoes_chave ON validacoes(chave_acesso)")
        .execute(pool)
        .await?;

    // Tabela de localizações geográficas de empresas
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS company_locations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            cnpj_cpf TEXT UNIQUE NOT NULL,
            razao_social TEXT NOT NULL,
            nome_fantasia TEXT,
            
            -- Endereço completo
            logradouro TEXT,
            numero TEXT,
            complemento TEXT,
            bairro TEXT,
            municipio TEXT NOT NULL,
            uf TEXT NOT NULL,
            cep TEXT,
            
            -- Coordenadas geográficas
            latitude REAL,
            longitude REAL,
            
            -- Dados do Google Maps
            place_id TEXT,
            formatted_address TEXT,
            google_types TEXT, -- JSON array
            
            -- Detalhes do negócio
            business_status TEXT,
            phone_number TEXT,
            website TEXT,
            rating REAL,
            user_ratings_total INTEGER,
            price_level INTEGER,
            
            -- Horário de funcionamento (JSON)
            opening_hours TEXT,
            
            -- Metadados
            last_geocoded_at DATETIME,
            last_places_update_at DATETIME,
            created_at DATETIME NOT NULL,
            updated_at DATETIME NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_company_cnpj ON company_locations(cnpj_cpf)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_company_city ON company_locations(municipio, uf)")
        .execute(pool)
        .await?;

    sqlx::query(
        "CREATE INDEX IF NOT EXISTS idx_company_coords ON company_locations(latitude, longitude)",
    )
    .execute(pool)
    .await?;

    // Tabela de fotos de estabelecimentos
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS company_photos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            company_location_id INTEGER NOT NULL,
            photo_reference TEXT NOT NULL,
            width INTEGER NOT NULL,
            height INTEGER NOT NULL,
            photo_url TEXT,
            created_at DATETIME NOT NULL,
            
            FOREIGN KEY (company_location_id) REFERENCES company_locations(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Tabela de avaliações
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS company_reviews (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            company_location_id INTEGER NOT NULL,
            author_name TEXT NOT NULL,
            rating INTEGER NOT NULL,
            text TEXT NOT NULL,
            review_time INTEGER NOT NULL,
            created_at DATETIME NOT NULL,
            
            FOREIGN KEY (company_location_id) REFERENCES company_locations(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
