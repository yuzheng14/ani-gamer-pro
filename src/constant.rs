use std::{path::PathBuf, sync::LazyLock};

pub static APP_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    directories::UserDirs::new()
        .expect("Could not get config dir")
        .home_dir()
        .to_owned()
        .join(".ani-gamer-pro")
});

#[cfg(not(test))]
pub static CONFIG_FILE_PATH: LazyLock<PathBuf> = LazyLock::new(|| APP_DIR.join("config.toml"));
#[cfg(test)]
pub static CONFIG_FILE_PATH: LazyLock<PathBuf> = LazyLock::new(|| APP_DIR.join("config.test.toml"));

#[cfg(not(test))]
pub static SN_LIST_FILE_PATH: LazyLock<PathBuf> = LazyLock::new(|| APP_DIR.join("sn_list.toml"));
#[cfg(test)]
pub static SN_LIST_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| APP_DIR.join("sn_list.test.toml"));

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::anyhow;
    use std::error::Error;

    fn get_home_dir() -> Result<PathBuf, anyhow::Error> {
        std::env::home_dir().ok_or(anyhow!("get home dir error"))
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn app_dir_on_macos() -> Result<(), Box<dyn Error>> {
        assert_eq!(APP_DIR.as_path(), get_home_dir()?.join(".ani-gamer-pro"));

        Ok(())
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn config_file_path_on_macos() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            CONFIG_FILE_PATH.as_path(),
            get_home_dir()?
                .join(".ani-gamer-pro")
                .join("config.test.toml")
        );

        Ok(())
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn sn_list_file_path_on_macos() -> Result<(), Box<dyn Error>> {
        assert_eq!(
            SN_LIST_FILE_PATH.as_path(),
            get_home_dir()?
                .join(".ani-gamer-pro")
                .join("sn_list.test.toml")
        );

        Ok(())
    }
}
