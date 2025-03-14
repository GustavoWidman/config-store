# config-store

A simple, flexible configuration management library for Rust that supports multiple serialization formats and both synchronous and asynchronous operations.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE.txt)

## Features

- Support for multiple serialization formats (JSON, TOML, YAML)
- Both synchronous and asynchronous APIs (via Tokio)
- Automatic creation of config files if they don't exist
- Easy updating and reading of configuration
- Transparent access to configuration values via Deref/DerefMut

## Installation

Add to your Cargo.toml:

```toml
[dependencies]
config-store = { version = "0.1.0", features = ["json"] }
```

Choose features based on your needs:

- `json` - JSON serialization support
- `toml` - TOML serialization support (preferred if multiple formats are enabled)
- `yaml` - YAML serialization support
- `tokio` - Async support via Tokio

## Usage

### Basic example

```rust
use config_store::ConfigStore;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Clone)]
struct Config {
    database_url: String,
    password: String,
    port: u16,
}

fn main() -> anyhow::Result<()> {
    // Read or create config file
    let mut config = ConfigStore::<Config>::read("config.json")?;

    // Access config values directly (via Deref)
    println!("Database URL: {}", config.database_url);

    // Modify values
    config.database_url = "postgres://localhost/mydb".into();

    // Save changes
    config.save()?;

    Ok(())
}
```

### Async example with Tokio

```rust
use config_store::ConfigStore;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Clone)]
struct Config {
    database_url: String,
    password: String,
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut config = ConfigStore::<Config>::async_read("config.toml").await?;

    config.password = "new_password".into();

    config.async_save().await?;

    Ok(())
}
```

## API Reference

### `ConfigStore<T>`

```rust
// Create or read config from a file (sync)
ConfigStore::<T>::read("path/to/config.json") -> Result<ConfigStore<T>, anyhow::Error>

// Create or read config from a file (async)
ConfigStore::<T>::async_read("path/to/config.json").await -> Result<ConfigStore<T>, anyhow::Error>

// Save config to file (sync)
config.save() -> Result<(), anyhow::Error>

// Save config to file (async)
config.async_save().await -> Result<(), anyhow::Error>

// Update from file (sync)
config.update() -> anyhow::Result<bool>

// Update from file (async)
config.async_update().await -> anyhow::Result<bool>

// Consume the ConfigStore and return the inner config
config.into_inner() -> T
```

## Examples

The library includes several examples demonstrating various configurations:

- JSON with standard API: json-std
- JSON with async API: json-async
- TOML with standard API: toml-std
- TOML with async API: toml-async
- YAML with standard API: yaml-std
- YAML with async API: yaml-async

To run an example:

```bash
cargo run --package json-std
```

## Serialization Format Priority

When multiple serialization format features are enabled, the library prioritizes them in this order:

1. TOML
2. JSON
3. YAML

## License

This project is licensed under the MIT License.
