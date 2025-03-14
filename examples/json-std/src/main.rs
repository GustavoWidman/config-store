use config_store::ConfigStore;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Clone)]
struct Config {
    database_url: String,
    password: String,
    port: u16,
}

fn main() -> anyhow::Result<()> {
    let mut config = ConfigStore::<Config>::read("cache/std-config.json")?;
    let config_stale = config.clone();

    // store implements deref and deref_mut for the inner type
    println!("Database URL: {}", config.database_url);
    println!("Password: {}", config.password);
    println!("Port: {}", config.port);

    config.database_url = "postgres://postgres:postgres@localhost:5432/postgres".into();
    config.password = "postgres".into();
    config.port = 5432;

    println!("Mutated Database URL: {}", config.database_url);
    println!("Mutated Password: {}", config.password);
    println!("Mutated Port: {}", config.port);

    config.save()?; // writes to the file

    println!("Stale Database URL: {}", config_stale.database_url);
    println!("Stale Password: {}", config_stale.password);
    println!("Stale Port: {}", config_stale.port);

    config.update()?; // reads again from the file (now saved)

    println!("Updated Database URL: {}", config.database_url);
    println!("Updated Password: {}", config.password);
    println!("Updated Port: {}", config.port);

    Ok(())
}
