use std::ffi::OsString;
use std::os::unix::ffi::OsStringExt;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use log::{error, info};
use prost::Message;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{UnixListener, UnixStream};
use tokio::sync::broadcast::Sender;

use crate::ipc::ipc_command::IpcCommand;
use crate::ipc::ipc_command::IpcCommand::{UnwatchFolder, WatchFolder};
use crate::protos::pipe::{GenericResponse, PipeMessage, UnwatchProject, WatchProject};
use crate::protos::pipe::pipe_message::Message::{ProjectToAdd, ProjectToRemove};

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
        let msg = PipeMessage::decode(&buf[..n])?;

        let response = match msg.message {
            Some(ProjectToAdd(watch_project)) => {
                match self.handle_watch_project(watch_project).await {
                    Ok(_) => GenericResponse {
                        success: true,
                        response_message: "Message received".to_string(),
                    },
                    Err(_) => GenericResponse {
                        success: false,
                        response_message: "Message can't be processed".to_string(),
                    },
                }
            }
            Some(ProjectToRemove(unwatch_project)) => {
                match self.handle_unwatch_project(unwatch_project).await {
                    Ok(_) => GenericResponse {
                        success: true,
                        response_message: "Message received".to_string(),
                    },
                    Err(_) => GenericResponse {
                        success: false,
                        response_message: "Message can't be processed".to_string(),
                    },
                }
            }
            None => GenericResponse {
                success: false,
                response_message: "Message is missing".to_string(),
            },
        };

        let response_buf = response.encode_to_vec();
        stream.write_all(&response_buf).await?;
        Ok(())
    }

    async fn handle_watch_project(&self, project: WatchProject) -> Result<()> {
        self.tx.send(WatchFolder {
            path: PathBuf::from(OsString::from_vec(project.project_path)),
        })?;

        Ok(())
    }

    async fn handle_unwatch_project(&self, project: UnwatchProject) -> Result<()> {
        self.tx.send(UnwatchFolder {
            path: PathBuf::from(OsString::from_vec(project.project_path)),
        })?;

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
