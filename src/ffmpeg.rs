use thiserror::Error;
use tokio::process::Command;

pub struct FFmpeg;

#[derive(Debug, Error)]
pub enum FFmpegError {
    #[error("file system error: {0}")]
    Io(#[from] std::io::Error),
    #[error("ffmpeg not in path")]
    FFmpegNotExist,
}

type FFmpegResult<T = ()> = Result<T, FFmpegError>;

impl FFmpeg {
    pub async fn exist() -> FFmpegResult<bool> {
        let status = Command::new("ffmpeg").arg("-h").status().await?;

        Ok(status.success())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn ffmpeg_exists() -> FFmpegResult {
        assert!(FFmpeg::exist().await?);

        Ok(())
    }
}
