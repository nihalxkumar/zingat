pub mod query;
pub mod model;

use std::str::FromStr;

use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use sqlx::Sqlite;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub type AppDatabase = Database<Sqlite>;
pub type DatabasePool = sqlx::sqlite::SqlitePool;
pub type Transaction<'t> = sqlx::Transaction<'t, Sqlite>;
pub type AppDatabaseRow = sqlx::sqlite::SqliteRow;
pub type AppQueryResult = sqlx::sqlite::SqliteQueryResult;

pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

impl Database<Sqlite> {
    pub async fn new(connection_str: &str) -> Self {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(connection_str)
            .await;
        match pool {
            Ok(pool) => Self(pool),
            Err(e) => panic!("{}", e),
        }
    }
    pub fn get_pool(&self) -> &DatabasePool {
        &self.0
    }
}

#[derive(Clone, Debug, From, Display, Deserialize, Serialize)]
pub struct DbId(Uuid);

impl DbId {
    pub fn new() -> Self {
        Uuid::new_v4().into()
    }
    pub fn nil() -> DbId {
        Self(Uuid::nil())
    }
}

impl From<DbId> for String {
    fn from(id: DbId) -> Self {
        format!("{}", id.0)
    }
}

impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for DbId {
    type Err = uuid::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Ok(DbId(Uuid::parse_str(id)?))
    }
}
