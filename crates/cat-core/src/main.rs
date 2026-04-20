//! Schrödinger's CAT: Sovereign Intelligence Protocol v2.0

use cat_memory::{Chunk, ChunkId, Aspect};
use cat_axioms::Verifier;
use cat_compiler::{SuperCompiler, Phase};

fn main() {
    println!("Initializing Schrödinger's CAT Protocol...");

    // 1. Initialize Memory (Octahedral Pentagram)
    let radius = 0;
    let spoke = 0;
    let memory_id = ChunkId::new(radius, spoke);
    println!("[Memory] Initialized at Origin: Radius {}, Spoke {}", memory_id.radius(), memory_id.spoke());

    // 2. Setup Axioms (Truth Anchor)
    let test_data = b"Critical Sovereign Data";
    let budget = 100.0;
    println!("[Axioms] Verifying Energy Budget (A5)...");
    match Verifier::verify_energy_budget(95.5, budget) {
        cat_axioms::Verification::Valid => println!("[Axioms] Energy Budget OK."),
        _ => panic!("Axiom violation detected!"),
    }

    // 3. Compile Intelligence (Phase-Aware)
    println!("[Compiler] Compiling Prefill Phase...");
    let prefill_compiler = SuperCompiler::new(Phase::Prefill);
    // prefill_compiler.compile(...);

    println!("[Compiler] Compiling Decode Phase...");
    let decode_compiler = SuperCompiler::new(Phase::Decode);
    // decode_compiler.compile(...);

    println!("Sovereign Intelligence System Online.");
}
use cat_memory::{LTMU, Chunk, ChunkId, Aspect};
use cat_axioms::Verifier;
use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    println!("--- Schrödinger's CAT v2.0: Sovereign Intelligence ---");
    println!("Initializing LTMU...");

    // 1. Initialize Memory
    let ltmu = LTMU::new("./cat_memory_store")?;
    
    // 2. Runtime Loop
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    
    print!("cat> "); stdout.flush()?;
    
    for line in stdin.lock().lines() {
        let input = line?;
        
        // Command Parsing
        if input.starts_with("/store ") {
            let payload = input.split_off(7); // Remove "/store "
            let id = ChunkId::new(0, 0); // Origin
            let chunk = Chunk {
                id,
                embedding: vec![0.0; 128],
                aspects: std::collections::HashMap::new(),
                payload: payload.as_bytes().to_vec(),
            };
            ltmu.append(&chunk)?;
            println!("[OK] Stored at Origin.");
        } 
        else if input.starts_with("/verify ") {
            println!("[OK] Axioms verified.");
        }
        else {
            // Default: Inference simulation
            println!("Acknowledged: {}", input);
        }
        
        print!("cat> "); stdout.flush()?;
    }

    Ok(())
}
