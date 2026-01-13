#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LetterStatus {
    Correct,   // Green: Right letter, right spot
    Present,   // Yellow: Right letter, wrong spot
    Absent,    // Gray: Not in the string
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct GuessResult {
    pub letter: char,
    pub status: LetterStatus,
}

pub fn check_guess(target: &str, guess: &str) -> Vec<GuessResult> {
    let n = target.len();
    let mut results = vec![
        GuessResult {
            letter: ' ',
            status: LetterStatus::Absent
        };
        n
    ];
    
    let target_chars: Vec<char> = target.chars().collect();
    let guess_chars: Vec<char> = guess.chars().collect();
    
    // Track which target letters have been "used" to prevent double-counting yellows
    let mut target_used = vec![false; n];
    let mut guess_handled = vec![false; n];

    // First Pass: Find all Greens (Correct)
    for i in 0..n {
        results[i].letter = guess_chars[i];
        if guess_chars[i] == target_chars[i] {
            results[i].status = LetterStatus::Correct;
            target_used[i] = true;
            guess_handled[i] = true;
        }
    }

    // Second Pass: Find Yellows (Present) and Grays (Absent)
    for i in 0..n {
        if guess_handled[i] {
            continue;
        }

        let mut found_present = false;
        for j in 0..n {
            // If the letter exists elsewhere in target and hasn't been matched yet
            if !target_used[j] && guess_chars[i] == target_chars[j] {
                results[i].status = LetterStatus::Present;
                target_used[j] = true;
                found_present = true;
                break;
            }
        }

        if !found_present {
            results[i].status = LetterStatus::Absent;
        }
    }

    results
}
