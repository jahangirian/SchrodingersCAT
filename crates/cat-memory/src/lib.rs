pub mod geometry;
pub mod store;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::RwLock;

/// The core ChunkId encoding Radius and Spoke.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkId(pub u64);

impl ChunkId {
    pub fn new(radius: u64, spoke: u8) -> Self {
        ChunkId((radius << 8) | (spoke as u64))
    }
    pub fn radius(&self) -> u64 { self.0 >> 8 }
    pub fn spoke(&self) -> u8 { (self.0 & 0xFF) as u8 }
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Aspect {
    Time,
    Truth,
    Origin,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub id: ChunkId,
    pub embedding: Vec<f32>,
    pub aspects: HashMap<Aspect, Vec<f32>>,
    pub payload: Vec<u8>,
}

/// The Long-Term Memory Unit.
pub struct LTMU {
    base_path: PathBuf,
    index: RwLock<HashMap<ChunkId, PathBuf>>, // In-memory index
}

impl LTMU {
    pub fn new(base_path: &str) -> std::io::Result<Self> {
        std::fs::create_dir_all(base_path)?;
        Ok(Self {
            base_path: PathBuf::from(base_path),
            index: RwLock::new(HashMap::new()),
        })
    }

    /// Appends a chunk deterministically.
    pub fn append(&self, chunk: &Chunk) -> std::io::Result<()> {
        let radius = chunk.id.radius();
        let segment_path = self.base_path.join(format!("shell_{}.seg", radius));
        
        // Use append-only file IO
        use std::fs::OpenOptions;
        use std::io::Write;
        
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&segment_path)?;
            
        // Serialize chunk to binary
        let encoded = bincode::serialize(chunk).unwrap();
        let len = encoded.len() as u32;
        
        file.write_all(&len.to_le_bytes())?;
        file.write_all(&encoded)?;
        
        // Update index
        self.index.write().unwrap().insert(chunk.id, segment_path);
        Ok(())
    }

    /// Retrieves a chunk by ID.
    pub fn get(&self, id: ChunkId) -> std::io::Result<Option<Chunk>> {
        // In a real implementation, we would seek to the offset stored in an index file.
        // For this operational prototype, we scan the shell file.
        let radius = id.radius();
        let segment_path = self.base_path.join(format!("shell_{}.seg", radius));
        
        if !segment_path.exists() { return Ok(None); }
        
        let data = std::fs::read(segment_path)?;
        let mut cursor = std::io::Cursor::new(&data);
        
        while (cursor.position() as usize) < data.len() {
            let mut len_bytes = [0u8; 4];
            if cursor.read_exact(&mut len_bytes).is_err() { break; }
            let len = u32::from_le_bytes(len_bytes) as usize;
            
            let mut buffer = vec![0u8; len];
            cursor.read_exact(&mut buffer)?;
            
            if let Ok(chunk) = bincode::deserialize::<Chunk>(&buffer) {
                if chunk.id == id {
                    return Ok(Some(chunk));
                }
            }
        }
        Ok(None)
    }
}

use std::io::Read; // Needed for Read trait
