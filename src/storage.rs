use crate::data::*;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqliteQueryResult;
use sqlx::{Pool, Sqlite, SqlitePool};
use std::fmt::Display;
use std::{error, fmt};

const SCHEMA_VERSION: u32 = 1;

pub struct Storage {
    db: Pool<Sqlite>,
}

impl Storage {
    pub async fn open(path: &str) -> Result<Self, sqlx::Error> {
        let db = if !Sqlite::database_exists(path).await.unwrap_or(false) {
            Sqlite::create_database(path).await?;
            let db = SqlitePool::connect(path).await?;
            sqlx::query(include_str!("../data/schema.sql"))
                .execute(&db)
                .await?;
            db
        } else {
            let db = SqlitePool::connect(path).await.unwrap();
            let version: u32 = sqlx::query_scalar("PRAGMA user_version")
                .fetch_one(&db)
                .await?;
            if version != SCHEMA_VERSION {
                panic!("Database version mismatch: {version} != {SCHEMA_VERSION}");
            }
            db
        };
        Ok(Self { db })
    }

    pub async fn get_kv(&self, key: &str) -> Result<String, QueryError> {
        sqlx::query_scalar::<_, String>("SELECT value FROM kv WHERE key = ?")
            .bind(key)
            .fetch_one(&self.db)
            .await
            .map_err(|_| QueryError::Custom(format!("cannot_get_kv_{key}")))
    }

    pub async fn insert_kv(&self, key: &str, value: &str) -> Result<SqliteQueryResult, QueryError> {
        sqlx::query("INSERT INTO kv VALUES (?, ?)")
            .bind(key)
            .bind(value)
            .execute(&self.db)
            .await
            .map_err(|_| QueryError::Custom(format!("cannot_insert_kv_{key}")))
    }

    pub async fn update_kv(&self, key: &str, value: &str) -> Result<SqliteQueryResult, QueryError> {
        sqlx::query("UPDATE kv SET value = ? WHERE key = ?")
            .bind(value)
            .bind(key)
            .execute(&self.db)
            .await
            .map_err(|_| QueryError::Custom(format!("cannot_update_kv_{key}")))
    }

    pub async fn get_group(&self, id: i64) -> Result<Group, QueryError> {
        sqlx::query_as("SELECT * FROM groups WHERE id = ?")
            .bind(id)
            .fetch_one(&self.db)
            .await
            .map_err(|_| QueryError::Custom(format!("cannot_get_group_{id}")))
    }

    pub async fn get_groups(&self) -> Result<Vec<Group>, QueryError> {
        sqlx::query_as("SELECT * FROM groups")
            .fetch_all(&self.db)
            .await
            .map_err(|_| QueryError::Custom("cannot_get_groups".into()))
    }

    pub async fn insert_group(&self, name: &str) -> Result<SqliteQueryResult, QueryError> {
        sqlx::query("INSERT INTO groups (id, epoch, name, schedule) VALUES (NULL, ?, ?, NULL)")
            .bind(None::<i64>)
            .bind(name)
            .execute(&self.db)
            .await
            .map_err(|_| QueryError::Custom(format!("cannot_insert_group_{name}")))
    }

    pub async fn delete_group(&self, id: i64) -> Result<SqliteQueryResult, QueryError> {
        sqlx::query("DELETE FROM groups WHERE id = ?")
            .bind(id)
            .execute(&self.db)
            .await
            .map_err(|_| QueryError::Custom(format!("cannot_delete_group_{id}")))
    }

    pub async fn update_schedule(
        &self,
        group_id: i64,
        schedule: Option<sqlx::types::Json<Vec<Schedule>>>,
    ) -> Result<SqliteQueryResult, QueryError> {
        sqlx::query("UPDATE groups SET schedule = ? WHERE id = ?")
            .bind(schedule)
            .bind(group_id)
            .execute(&self.db)
            .await
            .map_err(|_| QueryError::Custom(format!("cannot_update_schedule_{group_id}")))
    }

    pub async fn update_epoch(
        &self,
        group_id: i64,
        epoch: Option<i64>,
    ) -> Result<SqliteQueryResult, QueryError> {
        sqlx::query("UPDATE groups SET epoch = ? WHERE id = ?")
            .bind(epoch)
            .bind(group_id)
            .execute(&self.db)
            .await
            .map_err(|_| QueryError::Custom(format!("cannot_update_epoch_{group_id}")))
    }
}

#[derive(Debug)]
pub enum QueryError {
    Custom(String),
}

impl Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for QueryError {}

impl IntoResponse for QueryError {
    fn into_response(self) -> Response {
        match self {
            QueryError::Custom(e) => (
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::<()>::Err(e.to_string())),
            )
                .into_response(),
        }
    }
}
