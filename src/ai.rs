use crate::{State, Token};
use rand::Rng;

/// Player which plays randomly.
pub struct Rand {}

impl crate::Player for Rand {
    fn start(&mut self, _t: Token) { /* no op */
    }

    fn play(&self, _s: &State) -> usize {
        let mv = rand::thread_rng().gen_range(0, 7);
        mv
    }

    fn win(&self, _s: &State) { /* no op */
    }
    fn lose(&self, _s: &State) { /* no op */
    }
    fn draw(&self, _s: &State) { /* no op */
    }
}

/// Player which maximizes the score of the next ply, and only the next ply.
pub struct BestNextPly {
    color: Token,
}

impl BestNextPly {
    pub fn new() -> BestNextPly {
        BestNextPly { color: Token::Red }
    }
}

impl crate::Player for BestNextPly {
    fn start(&mut self, t: Token) {
        self.color = t;
    }

    fn play(&self, s: &State) -> usize {
        // best next move and its associated score
        let mut best_next = 0usize;
        let mut max_score = 0i32;

        // clone the state to be able to simulate next moves
        let mut cp = s.clone();
        for i in 0..s.width() {
            // simulate next move
            if let Err(()) = cp.append(i, &self.color) {
                continue;
            }

            let scores = scores(&cp);
            let mut score = scores.0 - scores.1; // red by default
            if let Token::Yellow = self.color {
                score = -score;
            }

            if score > max_score {
                best_next = i;
                max_score = score;
            }

            // undo next move
            cp.pop(i);
        }

        best_next
    }

    fn win(&self, _s: &State) { /* no op */
    }
    fn lose(&self, _s: &State) { /* no op */
    }
    fn draw(&self, _s: &State) { /* no op */
    }
}

fn scores(s: &State) -> (i32, i32) {
    let mut score_r = 0i32;
    let mut score_y = 0i32;

    for column in 0..s.width() {
        for row in 0..s.height() - s.line_length() + 1 {
            match s.count_vertical(row, column) {
                (n, 0) => {
                    score_r += n * n;
                }
                (0, n) => {
                    score_y += n * n;
                }
                _ => (),
            }
        }
    }

    for column in 0..s.width() - s.line_length() + 1 {
        for row in 0..s.height() {
            match s.count_horizontal(row, column) {
                (n, 0) => {
                    score_r += n * n;
                }
                (0, n) => {
                    score_y += n * n;
                }
                _ => (),
            }
        }
    }

    for column in 0..s.width() - s.line_length() + 1 {
        for row in 0..s.height() - s.line_length() + 1 {
            match s.count_diag_ne(row, column) {
                (n, 0) => {
                    score_r += n * n;
                }
                (0, n) => {
                    score_y += n * n;
                }
                _ => (),
            }
        }
    }

    for column in s.line_length() - 1..s.width() {
        for row in 0..s.height() - s.line_length() + 1 {
            match s.count_diag_nw(row, column) {
                (n, 0) => {
                    score_r += n * n;
                }
                (0, n) => {
                    score_y += n * n;
                }
                _ => (),
            }
        }
    }

    (score_r, score_y)
}
