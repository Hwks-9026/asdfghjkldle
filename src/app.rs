use crate::guess::{GuessResult, LetterStatus};

pub struct App {
    pub target: String,
    pub max_guesses: usize,
    pub word_length: usize,
    pub history: Vec<Vec<GuessResult>>, // Completed guesses
    pub current_guess: String,          // What the user is typing now
    pub game_over: bool,
    pub won: bool,
}

impl App {
    pub fn new(target: String, max_guesses: usize, word_length: usize) -> Self {
        Self {
            target,
            max_guesses,
            word_length,
            history: Vec::new(),
            current_guess: String::new(),
            game_over: false,
            won: false,
        }
    }

    pub fn submit_guess(&mut self) {
        if self.current_guess.len() == self.word_length {
            let result = crate::guess::check_guess(&self.target, &self.current_guess);
            
            // Check if all are correct
            if result.iter().all(|r| r.status == LetterStatus::Correct) {
                self.won = true;
                self.game_over = true;
            }
            
            self.history.push(result);
            self.current_guess.clear();

            if self.history.len() >= self.max_guesses {
                self.game_over = true;
            }
        }
    }
}
