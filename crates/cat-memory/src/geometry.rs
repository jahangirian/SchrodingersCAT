//! Defines the Octahedral-Pentagram topology.

/// Manhattan distance on the lattice.
/// Used for deterministic traversal (shell expansion).
pub fn manhattan_distance(a: (u64, u8), b: (u64, u8)) -> u64 {
    let dr = (a.0 as i64 - b.0 as i64).abs() as u64;
    let ds = (a.1 as i64 - b.1 as i64).abs() as u64;
    dr + ds
}

/// Calculates the shell capacity for a given radius R.
/// Formula: |Shell(R)| = 5 * R + 1
pub fn shell_capacity(radius: u64) -> u64 {
    5 * radius + 1
}
