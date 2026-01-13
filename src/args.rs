use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Davis Amaral", version = "0.1.0", about = "A chaotic, dictionary-free Wordle clone", long_about = None)]
pub struct Args {
    /// Length of the random string to guess
    #[arg(short, long, default_value_t = 5)]
    pub length: usize,

    /// Optional: Use a specific seed instead of today's date
    #[arg(short, long)]
    pub seed: Option<String>,

    /// Reveal the answer immediately (for the weak of heart)
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
}
