use crate::{State, Token};
use rand::Rng;
use std::i32;

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

/// Player which maximizes the score of the next ply by brute force analysis.
pub struct BruteForce {
    color: Token,
    max_depth: usize,
}

impl BruteForce {
    pub fn new(max_depth: usize) -> BruteForce {
        BruteForce {
            color: Token::Red,
            max_depth,
        }
    }
}

impl crate::Player for BruteForce {
    fn start(&mut self, t: Token) {
        self.color = t;
    }

    fn play(&self, s: &State) -> usize {
        let (next, _) = score(&mut s.clone(), &self.color, &self.color, self.max_depth);
        next
    }

    fn win(&self, _s: &State) { /* no op */
    }
    fn lose(&self, _s: &State) { /* no op */
    }
    fn draw(&self, _s: &State) { /* no op */
    }
}

fn score(s: &mut State, player: &Token, current_player: &Token, max_depth: usize) -> (usize, i32) {
    // recursion stops if
    // - max depth is 0
    // - game is finished (win or draw)

    // max depth has been reached
    if max_depth == 0 {
        let score: i32 = match player {
            Token::Red => {
                let scores = marginal_scores(s);
                scores.0 - scores.1
            }
            Token::Yellow => {
                let scores = marginal_scores(s);
                scores.1 - scores.0
            }
        };
        return (0, score);
    }

    // game is finished
    match s.win() {
        Some(winner) => {
            if winner == *player {
                // player wins
                return (0, i32::MAX);
            } else {
                // player loses
                return (0, i32::MIN);
            }
        }
        None => {
            if s.plies_left() == 0 {
                // this is a draw
                return (0, 0);
            }
        }
    }

    // is current player minimizing or maximizing player's score?
    let max = player == current_player;

    // recurse
    // note that there must be at least one possible move
    // otherwise the game would be finished
    let mut best_next: Option<usize> = None;
    let mut best_score = 0i32;

    for i in 0..s.width() {
        // simulate next move
        if let Err(()) = s.append(i, &current_player) {
            // cannot play this move
            continue;
        }

        // recursively compute score for this move
        let next_player = match current_player {
            Token::Red => Token::Yellow,
            Token::Yellow => Token::Red,
        };
        let (_, score) = score(s, player, &next_player, max_depth - 1);

        match best_next {
            None => {
                // first valid ply: init best next ply and best score
                best_next = Some(i);
                best_score = score;
            }
            Some(_) => {
                if max && score > best_score {
                    best_next = Some(i);
                    best_score = score;
                } else if !max && score < best_score {
                    best_next = Some(i);
                    best_score = score;
                }
            }
        };

        // undo next ply
        s.pop(i);
    }

    // best next move cannot be None
    // if so, it's a bug
    if let None = best_next {
        // this must never happen!
        panic!("impossible to move");
    }

    (best_next.unwrap(), best_score)
}

fn marginal_scores(s: &State) -> (i32, i32) {
    let mut score_r = 0i32;
    let mut score_y = 0i32;

    for column in 0..s.width() {
        for row in 0..s.height() - s.line_length() + 1 {
            match s.count_vertical(row, column) {
                (4, _) => {
                    return (i32::MAX, 0);
                }
                (_, 4) => {
                    return (0, i32::MAX);
                }
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
                (4, _) => {
                    return (i32::MAX, 0);
                }
                (_, 4) => {
                    return (0, i32::MAX);
                }
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
                (4, _) => {
                    return (i32::MAX, 0);
                }
                (_, 4) => {
                    return (0, i32::MAX);
                }
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
                (4, _) => {
                    return (i32::MAX, 0);
                }
                (_, 4) => {
                    return (0, i32::MAX);
                }
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
