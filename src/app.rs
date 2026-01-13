use crate::guess::{GuessResult, LetterStatus};

pub struct App {
    pub target: String,
    pub max_guesses: usize,
    pub word_length: usize,
    pub history: Vec<Vec<GuessResult>>,
    pub current_guess: Vec<char>,
    pub cursor_pos: usize,
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
            current_guess: vec!['~'; word_length],
            cursor_pos: 0,
            game_over: false,
            won: false,
        }
    }

    pub fn submit_guess(&mut self) {
        if !self.current_guess.contains(&'~') {
            let guess_str: String = self.current_guess.iter().collect();
            let result = crate::guess::check_guess(&self.target, &guess_str);
            
            if result.iter().all(|r| r.status == LetterStatus::Correct) {
                self.won = true;
                self.game_over = true;
            }
            
            self.history.push(result);
            self.current_guess = vec!['~'; self.word_length];
            self.cursor_pos = 0;

            if self.history.len() >= self.max_guesses {
                self.game_over = true;
            }
        }
    }
}
