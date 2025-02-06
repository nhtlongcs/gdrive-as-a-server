# Gdrive As A Server

This guide provides instructions for installing Rust, setting up dependencies, and running a GDrive CDN server. The server allows you to list and retrieve files stored in your Google Drive. Follow the steps below to get started.

## Installing Rust
Install Rust using the following command:
```
curl https://sh.rustup.rs -sSf | sh
```

## Setting Up Gdrive 

This repository uses the `gdrive` command line tool to interact with Google Drive. Follow the [gdrive installation instruction](gdrive.md) to set up the tool.

## Installing Dependencies
Ensure you have `cargo` and install required crates:
```sh
cargo build
```

## Running the Server
```sh
cargo run
```

The server will start on `http://0.0.0.1:3000`.

## API Endpoints
- **List all files:** `GET /files`
- **Get a file URL:** `GET /file/{filename}`

Replace `{filename}` with the actual file name from the logs.

## Notes
- Ensure `upload_log.txt` exists in the root directory.
- Modify the `log_file` path in the Rust script if needed.
- If you encounter permission issues, use `sudo cargo run` (Linux/macOS).

## Stopping the Server
Press `Ctrl + C` in the terminal to stop the server.

