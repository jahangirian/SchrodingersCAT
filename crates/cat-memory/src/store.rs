//! Persistent storage layer for the LTMU.
//! Uses append-only logs per radius for deterministic storage.

use std::fs::{File, OpenOptions};
use std::io::{Write, Result};
use std::path::PathBuf;

pub struct DiskLayer {
    file: File,
    radius: u64,
}

impl DiskLayer {
    pub fn open(base_path: &str, radius: u64) -> Result<Self> {
        let path = PathBuf::from(base_path).join(format!("radius_{}.seg", radius));
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        Ok(Self { file, radius })
    }

    pub fn append(&mut self, id: super::ChunkId, data: &[u8]) -> Result<()> {
        // Format: [ID: u64][Len: u32][Data: bytes]
        self.file.write_all(&id.0.to_le_bytes())?;
        self.file.write_all(&(data.len() as u32).to_le_bytes())?;
        self.file.write_all(data)?;
        Ok(())
    }
}
