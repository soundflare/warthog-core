use crate::utils::config::Config;
use log::{debug, error, info};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Pool, Sqlite, SqlitePool};
use std::str::FromStr;
use std::sync::Arc;

pub struct Database {
    pub pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(config: Arc<Config>) -> Self {
        debug!("Setting up database");

        let connection_options = SqliteConnectOptions::from_str(&config.database_url)
            .unwrap()
            .create_if_missing(true);

        let pool = match SqlitePool::connect_with(connection_options).await {
            Ok(pool) => {
                info!("Connection to the database is successful");
                pool
            }
            Err(err) => {
                error!("Failed to connect to the database: {:?}", err);
                std::process::exit(1);
            }
        };

        Database { pool }
    }

    pub async fn get_all_projects(&self) -> Result<Vec<(i32, String, String)>, sqlx::Error> {
        let result: Result<Vec<(i32, String, String)>, sqlx::Error> = sqlx::query_as("SELECT * FROM projects")
            .fetch_all(&self.pool)
            .await;

        result
    }

    pub async fn insert_project(&mut self, name: &str, path: &str) -> Result<(), sqlx::Error> {
        let result = sqlx::query!("INSERT INTO projects (name, path) VALUES ($1, $2)", name, path)
            .execute(&self.pool)
            .await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}