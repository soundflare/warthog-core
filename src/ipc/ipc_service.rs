use crate::processor::commands::IpcCommand;
use crate::processor::commands::IpcCommand::WatchFolder;
use crate::protos::pipe::schema::warthog_message::Message::WatchProject;
use crate::protos::pipe::schema::{Response, WarthogMessage};
use anyhow::{anyhow, Result};
use log::error;
use protobuf::Message;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::mpsc::Sender;

struct IpcService {
    tx: Sender<IpcCommand>,
}

impl IpcService {
    fn new(tx: Sender<IpcCommand>) -> Self {
        IpcService { tx }
    }

    async fn handle_connection(&self, mut stream: UnixStream) -> Result<()> {
        let mut buf = vec![0; 1024];
        let n = stream.read(&mut buf).await?;
        let msg = WarthogMessage::parse_from_bytes(&buf[..n])?;

        let mut response = Response::new();
        match msg.message.unwrap() {
            WatchProject(watch_project) => {
                self.tx
                    .send(WatchFolder {
                        name: watch_project.name,
                        path: watch_project.project_path,
                    })
                    .await?;

                response.success = true;
                response.response_message = "Message received".to_string();
            }
            None => {
                error!("Message is missing");

                response.success = false;
                response.response_message = "Message empty".to_string();
            }
        }

        let response_buf = response.write_to_bytes()?;
        stream.write_all(&response_buf).await?;
        Ok(())
    }

    async fn run(&self, path: &str) -> Result<()> {
        let _ = std::fs::remove_file(path);
        let listener = UnixListener::bind(path)?;

        loop {
            let (stream, _) = listener.accept().await?;
            let service = self.clone();

            tokio::spawn(async move {
                if let Err(e) = service.handle_connection(stream).await {
                    error!("Failed to handle connection: {}", e);
                }
            });
        }
    }
}
