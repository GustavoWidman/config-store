use anyhow::bail;
use serde::{Serialize, de::DeserializeOwned};

mod serializer;

use std::{
    ops::{Deref, DerefMut},
    path::PathBuf,
};

#[derive(Debug, Clone)]
pub struct ConfigStore<T: Default + Serialize + DeserializeOwned + PartialEq> {
    pub path: PathBuf,
    cached: T,
}

impl<T: Default + Serialize + DeserializeOwned + PartialEq> ConfigStore<T> {
    fn preflight(path: PathBuf) -> Result<Option<Self>, anyhow::Error> {
        if path.is_dir() {
            bail!(
                "Given config path is a directory... either change the path or delete the directory."
            );
        }

        if !path.exists() {
            return Ok(Some(Self::new(path)?));
        }

        if !path.is_file() {
            bail!(
                "Given config path exists and is not a file... either change the path or delete the file."
            );
        }

        Ok(None)
    }

    pub fn read(path: impl Into<PathBuf>) -> Result<Self, anyhow::Error> {
        let path = path.into();

        if let Some(config) = Self::preflight(path.clone())? {
            return Ok(config);
        }

        let config_str = std::fs::read_to_string(&path)?;

        Ok(Self {
            path,
            cached: serializer::from_str(&config_str)?,
        })
    }

    #[cfg(feature = "tokio")]
    pub async fn async_read(path: impl Into<PathBuf>) -> Result<Self, anyhow::Error> {
        let path = path.into();

        if let Some(config) = Self::preflight(path.clone())? {
            return Ok(config);
        }

        let config_str = tokio::fs::read_to_string(&path).await?;

        Ok(Self {
            path,
            cached: serializer::from_str(&config_str)?,
        })
    }

    pub fn update(&mut self) -> anyhow::Result<bool> {
        let new = Self::read(self.path.clone())?;

        Ok(match self.cached == new.cached {
            true => false,
            false => {
                self.cached = new.cached;
                true
            }
        })
    }

    #[cfg(feature = "tokio")]
    pub async fn async_update(&mut self) -> anyhow::Result<bool> {
        let new = Self::async_read(self.path.clone()).await?;

        Ok(match self.cached == new.cached {
            true => false,
            false => {
                self.cached = new.cached;
                true
            }
        })
    }

    fn new(path: PathBuf) -> Result<Self, anyhow::Error> {
        std::fs::create_dir_all(path.parent().unwrap())?;

        let config = Self {
            path,
            cached: T::default(),
        };

        config.save()?;

        Ok(config)
    }

    pub fn into_inner(self) -> T {
        self.cached
    }

    pub fn save(&self) -> Result<(), anyhow::Error> {
        std::fs::write(&self.path, serializer::to_string(&self.cached)?)?;

        Ok(())
    }

    #[cfg(feature = "tokio")]
    pub async fn async_save(&self) -> Result<(), anyhow::Error> {
        tokio::fs::write(&self.path, serializer::to_string(&self.cached)?).await?;

        Ok(())
    }
}

impl<T: Default + Serialize + DeserializeOwned + PartialEq> Deref for ConfigStore<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.cached
    }
}

impl<T: Default + Serialize + DeserializeOwned + PartialEq> DerefMut for ConfigStore<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cached
    }
}

impl<T: Default + Serialize + DeserializeOwned + PartialEq> PartialEq for ConfigStore<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cached == other.cached
    }
}
impl<T: Default + Serialize + DeserializeOwned + PartialEq> Eq for ConfigStore<T> {}
