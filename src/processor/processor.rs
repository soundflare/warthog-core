use crate::processor::command::Command;
use log::info;
use tokio::sync::mpsc::Receiver;

pub struct Processor {
    receiver: Receiver<Command>,
}

impl Processor {
    pub fn new(receiver: Receiver<Command>) -> Self {
        Self { receiver }
    }

    pub async fn process_commands(&mut self) {
        while let Some(command) = self.receiver.recv().await {
            match command {
                Command::ChangeDetected { paths } => {
                    info!("Received event");
                    // paths.
                }
                // Add more command handling cases as needed
                _ => {}
            }
        }
    }

    fn find_matching_path<'a>(database_paths: &'a[String], event_path: &'a str) -> Option<&'a String> {
        database_paths.iter().find(|&db_path| event_path.starts_with(db_path))
    }
}
