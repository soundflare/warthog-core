use crate::utils::config::Config;
use log::{debug, error, info};
use sqlx::sqlite::SqliteConnectOptions;
use sqlx::{Pool, Sqlite, SqlitePool};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

pub struct Database {
    pub pool: Pool<Sqlite>,
    cache: HashMap<i32, (String, String)>,
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

        let cache = HashMap::new();
        Database { pool, cache }
    }

    pub fn get_all_project_paths(&self) -> Result<Vec<String>, sqlx::Error> {
        Ok(self
            .cache
            .iter()
            .map(|(_, (_, path))| path.clone())
            .collect())
    }

    pub(crate) async fn populate_cache(&mut self) -> Result<(), sqlx::Error> {
        let result: Result<Vec<(i32, String, String)>, sqlx::Error> =
            sqlx::query_as("SELECT * FROM projects")
                .fetch_all(&self.pool)
                .await;

        match result {
            Ok(projects) => {
                for (id, name, path) in projects {
                    self.cache.insert(id, (name, path));
                }
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub async fn insert_project(&mut self, name: &str, path: &str) -> Result<(), sqlx::Error> {
        let result = sqlx::query!(
            "INSERT INTO projects (name, path) VALUES ($1, $2)",
            name,
            path
        )
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => {
                let last_id: i32 = sqlx::query_scalar("SELECT last_insert_rowid()")
                    .fetch_one(&self.pool)
                    .await?;
                self.cache
                    .insert(last_id, (name.to_string(), path.to_string()));
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub async fn remove_project(&mut self, path: &str) -> Result<i32, sqlx::Error> {
        let id_result: Option<i64> =
            sqlx::query_scalar!("SELECT id FROM projects WHERE path = $1", path)
                .fetch_one(&self.pool)
                .await?;

        let id = match id_result {
            Some(id) => id,
            None => return Err(sqlx::Error::RowNotFound),
        };

        let delete_result = sqlx::query!("DELETE FROM projects WHERE id = $1", id)
            .execute(&self.pool)
            .await;

        match delete_result {
            Ok(_) => {
                self.cache.remove(&(id as i32));
                Ok(id as i32)
            }
            Err(err) => Err(err),
        }
    }
}
