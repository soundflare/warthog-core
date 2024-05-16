use crate::db::database::Database;
use crate::watcher::watcher_command::WatcherCommand;
use log::info;
use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;

pub struct EventProcessor {
    receiver: Receiver<WatcherCommand>,
    database: Arc<Mutex<Database>>,
}

impl EventProcessor {
    pub fn new(receiver: Receiver<WatcherCommand>, database: Arc<Mutex<Database>>) -> Self {
        Self { receiver, database }
    }

    pub async fn process_commands(&mut self) {
        while let Some(command) = self.receiver.recv().await {
            match command {
                WatcherCommand::ChangeDetected { paths } => {
                    self.handle_change_detected(paths).await
                }
            }
        }
    }

    async fn handle_change_detected(&self, paths: Vec<String>) {
        info!("Received event");

        let database_paths = match self.database.lock().await.get_all_project_paths() {
            Ok(paths) => paths,
            Err(err) => {
                info!("Failed to get database paths: {:?}", err);
                return;
            }
        };
        info!("Database paths: {:?}", database_paths);

        let changed_project_path = Self::find_matching_path(&database_paths, &paths[0]);
        match changed_project_path {
            Some(path) => info!("Event path matches database path: {}", path),
            None => info!("Event path does not match any database path"),
        }
    }

    fn find_matching_path<'a>(
        database_paths: &'a [String],
        event_path: &'a str,
    ) -> Option<&'a String> {
        database_paths
            .iter()
            .find(|&db_path| event_path.starts_with(db_path))
    }
}
