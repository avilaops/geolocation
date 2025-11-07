pub mod repository;
pub mod schema;
pub mod mongodb;

use crate::error::Result;
use sqlx::{Pool, Sqlite, postgres::{PgPool, PgPoolOptions}};
use sqlx::sqlite::SqlitePoolOptions;

#[derive(Clone)]
pub enum DatabasePool {
    Sqlite(Pool<Sqlite>),
    Postgres(PgPool),
}

pub struct Database {
    pool: DatabasePool,
}

impl Database {
    /// Cria uma nova conexão com banco de dados SQLite
    pub async fn new_sqlite(database_url: &str) -> Result<Self> {
        use sqlx::sqlite::SqliteConnectOptions;
        use std::str::FromStr;
        
        // Formata URL como sqlite:// se necessário
        let url = if database_url.starts_with("sqlite://") {
            database_url.to_string()
        } else {
            format!("sqlite://{}", database_url)
        };
        
        let options = SqliteConnectOptions::from_str(&url)?
            .create_if_missing(true);
        
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await?;
        
        Ok(Database {
            pool: DatabasePool::Sqlite(pool),
        })
    }
    
    /// Cria uma nova conexão com banco de dados PostgreSQL
    pub async fn new_postgres(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;
        
        Ok(Database {
            pool: DatabasePool::Postgres(pool),
        })
    }
    
    /// Inicializa o schema do banco de dados
    pub async fn initialize_schema(&self) -> Result<()> {
        match &self.pool {
            DatabasePool::Sqlite(pool) => {
                schema::create_tables_sqlite(pool).await?;
            }
            DatabasePool::Postgres(pool) => {
                schema::create_tables_postgres(pool).await?;
            }
        }
        Ok(())
    }
    
    pub fn pool(&self) -> &DatabasePool {
        &self.pool
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sqlite_connection() {
        let db = Database::new_sqlite(":memory:").await;
        assert!(db.is_ok());
    }
}
