//! cat-memory: Long-Term Memory Unit
//! Implements the Octahedral Pentagram geometry with Aspected Retrieval.

pub mod geometry;
pub mod store;

/// Represents a single point in the Octahedral Pentagram space.
/// Encodes (Radius, Spoke) into a deterministic 64-bit ID.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChunkId(u64);

impl ChunkId {
    /// Encodes radius (shell) and spoke (axis) into a ChunkId.
    /// Upper 56 bits: Radius. Lower 8 bits: Spoke.
    pub fn new(radius: u64, spoke: u8) -> Self {
        ChunkId((radius << 8) | (spoke as u64))
    }

    pub fn radius(&self) -> u64 {
        self.0 >> 8
    }

    pub fn spoke(&self) -> u8 {
        (self.0 & 0xFF) as u8
    }
}

/// Aspects for Holographic Retrieval.
/// Maps to the Pentagram geometry to filter relevance, not just similarity.
#[derive(Clone, Copy, Debug)]
pub enum Aspect {
    Time,       // Temporal relevance
    Truth,      // Provenance/Verification status
    Structure,  // Logical or Data structure type
    Origin,     // Source/Ownership
    Sensitivity // Security/Privacy classification
}

/// A data chunk stored in the LTMU.
pub struct Chunk {
    pub id: ChunkId,
    pub embedding: [f32; 256], // Base semantic vector
    // Aspected embeddings allow multi-dimensional relevance filtering
    pub aspects: std::collections::HashMap<Aspect, Vec<f32>>,
    pub payload: Vec<u8>,
}
