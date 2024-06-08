use crate::ipc::ipc_command::IpcCommand;
use crate::ipc::ipc_command::IpcCommand::{UnwatchFolder, WatchFolder};
use crate::protos::ipc_schema::pipe_message::Message::{ProjectToAdd, ProjectToRemove};
use crate::protos::ipc_schema::{Response, UnwatchProject, PipeMessage, WatchProject};
use anyhow::Result;
use log::{error, info};
use protobuf::Message;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::broadcast::Sender;

pub struct IpcService {
    tx: Arc<Sender<IpcCommand>>,
}

impl IpcService {
    pub fn new(tx: Sender<IpcCommand>) -> Self {
        IpcService { tx: Arc::new(tx) }
    }

    async fn handle_connection(&self, mut stream: UnixStream) -> Result<()> {
        let mut buf = vec![0; 1024];
        let n = stream.read(&mut buf).await?;
        let msg = PipeMessage::parse_from_bytes(&buf[..n])?;

        let mut response = Response::new();
        match msg.message {
            Some(ProjectToAdd(watch_project)) => {
                self.handle_watch_project(watch_project, &mut response)
                    .await?;
            }
            Some(ProjectToRemove(unwatch_project)) => {
                self.handle_unwatch_project(unwatch_project, &mut response)
                    .await?;
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

    async fn handle_watch_project(
        &self,
        project: WatchProject,
        response: &mut Response,
    ) -> Result<()> {
        self.tx.send(WatchFolder {
            name: project.name,
            path: project.project_path,
        })?;

        response.success = true;
        response.response_message = "Message received".to_string();
        Ok(())
    }

    async fn handle_unwatch_project(
        &self,
        project: UnwatchProject,
        response: &mut Response,
    ) -> Result<()> {
        self.tx.send(UnwatchFolder {
            path: project.project_path,
        })?;

        response.success = true;
        response.response_message = "Message received".to_string();
        Ok(())
    }

    /// Spawn a thread for handling new IPC connection and a thread for each connection.
    pub async fn run(&self, path: &str) -> Result<()> {
        info!("Starting IPC service");

        let _ = std::fs::remove_file(path);
        let listener = UnixListener::bind(path)?;

        let tx = Arc::clone(&self.tx);
        tokio::spawn(async move {
            loop {
                let stream = match listener.accept().await {
                    Ok((stream, _)) => stream,
                    Err(e) => {
                        error!("Failed to accept connection: {}", e);
                        continue;
                    }
                };

                let tx_connection = Arc::clone(&tx);
                tokio::spawn(async move {
                    let service = IpcService { tx: tx_connection };
                    if let Err(e) = service.handle_connection(stream).await {
                        error!("Failed to handle connection: {}", e);
                    }
                });
            }
        });

        Ok(())
    }
}
