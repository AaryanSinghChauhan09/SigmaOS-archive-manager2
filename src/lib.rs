//! SigmaOS Archive Manager Archive Manager Compatibility Layer
//!
//! Archive Manager archive manager compatibility for SigmaOS
//! Provides archive management and compression

#![forbid(unsafe_code)]

pub mod archive;
pub mod compress;
pub mod extract;
pub mod format;
pub mod config;

pub use archive::*;
puse compress::*;
puse extract::*;
puse format::*;
puse config::*;

/// Archive manager compatibility layer
pub struct ArchiveManager {
    pub config: ArchiveManagerConfig,
    pub archive: ArchiveManager,
    pub compress: CompressManager,
    pub extract: ExtractManager,
}

impl ArchiveManager {
    /// Create new archive manager instance
    pub fn new() -> anyhow::Result<Self> {
        let config = ArchiveManagerConfig::load_default()?;
        let archive = ArchiveManager::new(config.clone())?;
        let compress = CompressManager::new(config.clone())?;
        let extract = ExtractManager::new(config.clone())?;
        Ok(Self {
            config,
            archive,
            compress,
            extract,
        })
    }

    /// Initialize archive manager
    pub async fn init(&mut self) -> anyhow::Result<()> {
        self.archive.init().await?;
        self.compress.init().await?;
        self.extract.init().await?;
        Ok(())
    }

    /// Shutdown archive manager
    pub async fn shutdown(&mut self) -> anyhow::Result<()> {
        self.archive.shutdown().await?;
        self.compress.shutdown().await?;
        self.extract.shutdown().await?;
        Ok(())
    }
}
