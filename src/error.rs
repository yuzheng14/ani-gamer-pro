use thiserror::Error;
use tokio::io;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("encounter io error when manipulate config file: {0}")]
    IO(#[from] io::Error),
    #[error("encounter toml serialize error when writing config file: {0}")]
    TomlSer(#[from] toml::ser::Error),
    #[error("encounter toml parse error when reading config file: {0}")]
    TomlDe(#[from] toml::de::Error),
}

#[derive(Debug, Error)]
pub enum SnListError {
    #[error("encounter io error when manipulate sn list file: {0}")]
    IO(#[from] io::Error),
    #[error("encounter toml serialize error when writing sn list file: {0}")]
    TomlSer(#[from] toml::ser::Error),
    #[error("encounter toml parse error when reading sn list file: {0}")]
    TomlDe(#[from] toml::de::Error),
}

#[derive(Debug, Error)]
pub enum CookieError {
    #[error("encounter io error when manipulate cookie file: {0}")]
    IO(#[from] io::Error),
    #[error("could not find cookie file")]
    NotFound,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ConfigError(#[from] ConfigError),
    #[error(transparent)]
    SnListError(#[from] SnListError),
    #[error(transparent)]
    CookieError(#[from] CookieError),
}
