use rand::{Rng, SeedableRng};
use sha2::Digest;

use crate::args::Args;

pub fn generate_word(args: &Args) -> String {
    // If a seed is provided in CLI, use it; otherwise, use today's date
    let seed_input = args.seed.clone().unwrap_or_else(|| {
        chrono::Local::now().format("%Y-%m-%d").to_string()
    });

    let mut hasher = sha2::Sha256::new();
    hasher.update(seed_input.as_bytes());
    let hash_result = hasher.finalize();

    let mut seed = [0u8; 32];
    seed.copy_from_slice(&hash_result);

    let mut rng = rand_chacha::ChaCha8Rng::from_seed(seed);
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect();
    
    (0..args.length)
        .map(|_| {
            let idx = rng.gen_range(0..alphabet.len());
            alphabet[idx]
        })
        .collect()
}
