pub mod ai;

use std::fmt;

const WIDTH: usize = 7;
const HEIGHT: usize = 6;
const LINE: usize = 4;

pub trait Player {
    fn start(&mut self, tok: Token);
    fn play(&self, s: &State) -> usize;
    fn win(&self, s: &State);
    fn lose(&self, s: &State);
    fn draw(&self, s: &State);
}

pub fn play(mut player1: impl Player, mut player2: impl Player) {
    // player 1 is red
    player1.start(Token::Red);
    player2.start(Token::Yellow);

    // keep track of the number of moves
    let mut count = 0i32;

    // keep track of the current state of the game
    let mut state = State::new();

    // run the game!
    let mut end_of_game = false;
    while !end_of_game {
        // player 1 is always red and always start
        let tok = if count % 2 == 0 {
            Token::Red
        } else {
            Token::Yellow
        };

        let mut valid_move = false;
        while !valid_move {
            let mv = match tok {
                Token::Red => player1.play(&state),
                Token::Yellow => player2.play(&state),
            };

            if mv >= WIDTH {
                continue;
            }
            if let Err(()) = state.append(mv, &tok) {
                continue;
            }

            // validate move
            valid_move = true;
        }

        // is this a win?
        match state.win() {
            None => {
                if state.plies_left() == 0 {
                    player1.draw(&state);
                    player2.draw(&state);
                    end_of_game = true;
                }
            }
            Some(t) => {
                match t {
                    Token::Red => {
                        player1.win(&state);
                        player2.lose(&state);
                    }
                    Token::Yellow => {
                        player1.lose(&state);
                        player2.win(&state);
                    }
                }
                end_of_game = true;
            }
        }

        // next round
        count += 1;
    }
}

pub enum Token {
    Red,
    Yellow,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Red => write!(f, "R"),
            Token::Yellow => write!(f, "Y"),
        }
    }
}

/// Represents the state of the game at some point.
pub struct State {
    content: [[u8; HEIGHT]; WIDTH],
}

impl State {
    pub fn new() -> State {
        State {
            content: [[0; HEIGHT]; WIDTH],
        }
    }

    /// Width of the board.
    pub fn width(&self) -> usize {
        WIDTH
    }

    /// Height of the board.
    pub fn height(&self) -> usize {
        HEIGHT
    }

    /// Target length of the line.
    pub fn line_length(&self) -> usize {
        LINE
    }

    /// Returns the color of the token at the given position.
    ///
    /// Returns None if there is no token.
    pub fn get(&self, row: usize, column: usize) -> Option<Token> {
        match self.content[column][row] {
            1 => Some(Token::Red),
            2 => Some(Token::Yellow),
            _ => None, // 0 (or anything else) is considered "empty"
        }
    }

    /// Returns whether or not a token can be appended to the column.
    pub fn can_append(&self, column: usize) -> bool {
        // is there at least one free cell in column?
        self.content[column][HEIGHT - 1] == 0
    }

    /// Append a token to the column.
    ///
    /// Returns Err(()) if the column is already full.
    pub fn append(&mut self, column: usize, t: &Token) -> Result<(), ()> {
        for i in 0..HEIGHT {
            // is there already a token at this position?
            if self.content[column][i] != 0 {
                continue;
            }

            // add token to this column
            match t {
                Token::Red => {
                    self.content[column][i] = 1;
                }
                Token::Yellow => {
                    self.content[column][i] = 2;
                }
            }
            return Ok(()); // add it only once!
        }

        // could not add token because this column is already full
        Err(())
    }

    /// Removes last token appended to the column and returns it.
    ///
    /// Returns None if there was no token to remove.
    pub fn pop(&mut self, column: usize) -> Option<Token> {
        for i in (0..HEIGHT).rev() {
            let t = self.content[column][i];

            // if empty, try below
            if t == 0 {
                continue;
            }

            // not empty, so empty it and return the token
            self.content[column][i] = 0;
            match t {
                1 => {
                    return Some(Token::Red);
                }
                2 => {
                    return Some(Token::Yellow);
                }
                _ => (),
            }
        }

        // cound not find any token to pop
        None
    }

    /// Maximum number of plies left before the game ends.
    pub fn plies_left(&self) -> u8 {
        let mut count = 0u8;
        for column in self.content.iter() {
            for cell in column.iter() {
                if *cell == 0 {
                    count += 1;
                }
            }
        }
        count
    }

    /// Returns the token color of the player who wins.
    ///
    /// Returns None if the game isn't over yet.
    pub fn win(&self) -> Option<Token> {
        for column in 0..WIDTH {
            for row in 0..HEIGHT - LINE + 1 {
                match self.count_vertical(row, column) {
                    (4, _) => {
                        return Some(Token::Red);
                    }
                    (_, 4) => {
                        return Some(Token::Yellow);
                    }
                    _ => (),
                }
            }
        }

        for column in 0..WIDTH - LINE + 1 {
            for row in 0..HEIGHT {
                match self.count_horizontal(row, column) {
                    (4, _) => {
                        return Some(Token::Red);
                    }
                    (_, 4) => {
                        return Some(Token::Yellow);
                    }
                    _ => (),
                }
            }
        }

        for column in 0..WIDTH - LINE + 1 {
            for row in 0..HEIGHT - LINE + 1 {
                match self.count_diag_ne(row, column) {
                    (4, _) => {
                        return Some(Token::Red);
                    }
                    (_, 4) => {
                        return Some(Token::Yellow);
                    }
                    _ => (),
                }
            }
        }

        for column in LINE - 1..WIDTH {
            for row in 0..HEIGHT - LINE + 1 {
                match self.count_diag_nw(row, column) {
                    (4, _) => {
                        return Some(Token::Red);
                    }
                    (_, 4) => {
                        return Some(Token::Yellow);
                    }
                    _ => (),
                }
            }
        }

        None
    }

    fn count_vertical(&self, row: usize, column: usize) -> (i32, i32) {
        let mut count_r = 0i32;
        let mut count_y = 0i32;

        for i in 0..LINE {
            if let Some(t) = self.get(row + i, column) {
                match t {
                    Token::Red => {
                        count_r += 1;
                    }
                    Token::Yellow => {
                        count_y += 1;
                    }
                }
            }
        }

        (count_r, count_y)
    }

    fn count_horizontal(&self, row: usize, column: usize) -> (i32, i32) {
        let mut count_r = 0i32;
        let mut count_y = 0i32;

        for i in 0..LINE {
            if let Some(t) = self.get(row, column + i) {
                match t {
                    Token::Red => {
                        count_r += 1;
                    }
                    Token::Yellow => {
                        count_y += 1;
                    }
                }
            }
        }

        (count_r, count_y)
    }

    fn count_diag_ne(&self, row: usize, column: usize) -> (i32, i32) {
        let mut count_r = 0i32;
        let mut count_y = 0i32;

        for i in 0..LINE {
            if let Some(t) = self.get(row + i, column + i) {
                match t {
                    Token::Red => {
                        count_r += 1;
                    }
                    Token::Yellow => {
                        count_y += 1;
                    }
                }
            }
        }

        (count_r, count_y)
    }

    fn count_diag_nw(&self, row: usize, column: usize) -> (i32, i32) {
        let mut count_r = 0i32;
        let mut count_y = 0i32;

        for i in 0..LINE {
            if let Some(t) = self.get(row + i, column - i) {
                match t {
                    Token::Red => {
                        count_r += 1;
                    }
                    Token::Yellow => {
                        count_y += 1;
                    }
                }
            }
        }

        (count_r, count_y)
    }
}

impl Clone for State {
    fn clone(&self) -> Self {
        State {
            content: self.content,
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::with_capacity((HEIGHT + 2) * (WIDTH + 3));

        s.push('+');
        for _i in 0..WIDTH {
            s.push('-');
        }
        s.push('+');
        s.push('\n');

        for i in (0..HEIGHT).rev() {
            s.push('|');
            for j in 0..WIDTH {
                match self.get(i, j) {
                    None => {
                        s.push(' ');
                    }
                    Some(t) => {
                        s.push_str(&format!("{}", t)[..]);
                    }
                }
            }
            s.push('|');
            s.push('\n');
        }

        s.push('+');
        for _i in 0..WIDTH {
            s.push('-');
        }
        s.push('+');
        s.push('\n');

        write!(f, "{}", s)
    }
}
