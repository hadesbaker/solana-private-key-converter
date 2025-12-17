use std::{env, fs, process};

fn main() {
    let path = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: sol_keypair_to_b58 <path/to/wallet.json>");
        process::exit(1);
    });

    let raw = fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("Failed to read {}: {}", path, e);
        process::exit(1);
    });

    let bytes: Vec<u8> = serde_json::from_str(&raw).unwrap_or_else(|e| {
        eprintln!("Failed to parse JSON (expected [u8, u8, ...]): {}", e);
        process::exit(1);
    });

    if bytes.len() != 64 && bytes.len() != 32 {
        eprintln!(
            "Unexpected key length: {} bytes (expected 64 for Solana keypair JSON, or 32 seed bytes).",
            bytes.len()
        );
        process::exit(1);
    }

    eprintln!("WARNING: You are about to print a private key. Do NOT share this output.");

    // Base58 encode the bytes exactly as stored in the file
    let b58_full = bs58::encode(&bytes).into_string();
    println!("base58_full_{}bytes: {}", bytes.len(), b58_full);

    // If it's a 64-byte Solana CLI keypair, also print the first 32 bytes (seed) as Base58
    if bytes.len() == 64 {
        let b58_seed = bs58::encode(&bytes[..32]).into_string();
        println!("base58_seed_32bytes: {}", b58_seed);
    }
}
