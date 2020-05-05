use std::fmt;

pub const WIDTH: usize = 7;
const HEIGHT: usize = 6;
const LINE: usize = 4;

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

pub struct State {
    content: [[u8; HEIGHT]; WIDTH],
}

impl State {
    pub fn new() -> State {
        State {
            content: [[0; HEIGHT]; WIDTH],
        }
    }

    fn get(&self, row: usize, column: usize) -> Option<Token> {
        match self.content[column][row] {
            1 => Some(Token::Red),
            2 => Some(Token::Yellow),
            _ => None, // 0 (or anything else) is considered "empty"
        }
    }

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

    pub fn rounds_left(&self) -> u8 {
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

    pub fn won(&self) -> Option<Token> {
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

    pub fn score(&self) -> (i32, i32) {
        let mut score_r = 0i32;
        let mut score_y = 0i32;

        for column in 0..WIDTH {
            for row in 0..HEIGHT - LINE + 1 {
                match self.count_vertical(row, column) {
                    (n, 0) => {
                        score_r += n;
                    }
                    (0, n) => {
                        score_y += n;
                    }
                    _ => (),
                }
            }
        }

        for column in 0..WIDTH - LINE + 1 {
            for row in 0..HEIGHT {
                match self.count_horizontal(row, column) {
                    (n, 0) => {
                        score_r += n;
                    }
                    (0, n) => {
                        score_y += n;
                    }
                    _ => (),
                }
            }
        }

        for column in 0..WIDTH - LINE + 1 {
            for row in 0..HEIGHT - LINE + 1 {
                match self.count_diag_ne(row, column) {
                    (n, 0) => {
                        score_r += n;
                    }
                    (0, n) => {
                        score_y += n;
                    }
                    _ => (),
                }
            }
        }

        for column in LINE - 1..WIDTH {
            for row in 0..HEIGHT - LINE + 1 {
                match self.count_diag_nw(row, column) {
                    (n, 0) => {
                        score_r += n;
                    }
                    (0, n) => {
                        score_y += n;
                    }
                    _ => (),
                }
            }
        }

        (score_r, score_y)
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