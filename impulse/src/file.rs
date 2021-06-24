use std::path::Path;
use tokio::io::Result;

/// Allows reading from and writing to a `tokio::fs::File`.
pub struct File {
    file: tokio::fs::File,
}

impl File {
    /// Opens the file.
    pub async fn open(path: impl AsRef<Path>) -> Result<File> {
        let file = tokio::fs::File::open(path).await?;
        Ok(File { file })
    }

    /// Read bytes from file.
    pub async fn read(&self) -> Result<Vec<u8>> {
        todo!()
    }

    /// Write bytes to file.
    pub async fn write(&self, _buf: &[u8]) -> Result<()> {
        todo!()
    }
}
