# warthog

🚧 **Work in progress** 🚧

Warthog is a version control system designed for large binary files usually present in projects in media productions 
(e.g. DAW project files). It's written in Rust for efficiency and performance reasons. The main goal of the project is to provide a simple and efficient
way to version control large binary files and provide a solution like Git for media production workflows.

## How it works

This part of a larger application for version control and acts as a service that constantly checks for changes in the watched directories,
splits the files into chunks, run deduplication, and compresses them with gzip. It then sends the chunks to a warthog server for storage via Tus protocol for resumable uploads.
The main supported storage backend is planned to be S3, with support for other storage backends, including local storage, planned in the future.

For interfacing with the service there is an IPC pipe that can be used to send commands to the service via a Protobuf scheme. 
It's meant to be used with a CLI tool and a Desktop app that uses the pipe to communicate with the service.

## Configuration

### .env.example file

The application is configurable solely through environment variables. Included in this repository is the .env.example file for easy configuration.
The application has default values for the whole configuration and works without explicitly setting the environment variables.

| environment variable | Description                                                         | Default value       |
|----------------------|---------------------------------------------------------------------|---------------------|
| DATABASE_URL         | URL for the SQLite database                                         | sqlite://warthog.db | 
| BLOCK_SIZE           | Block size for splitting binary files into chunks for deduplication | 1024                |


For local development it can also be beneficial to set SQLX_OFFLINE=true to avoid the need for a running database when checking the SQL queries.

## Persistence

The application uses a SQLite database and the sqlx library solely for storing the key for creating a libp2p swarm. If the key is not present in the database, the application will generate a new one and store it.
If the database is not present, the application will create a new one.
