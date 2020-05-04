use std::fmt;
use std::io;

const HEIGHT: usize = 6;
const WIDTH: usize = 7;
const LINE: usize = 4;

enum Token {
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

struct State {
    content: [[u8; HEIGHT]; WIDTH],
}

impl State {
    fn new() -> State {
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

    fn append(&mut self, column: usize, t: &Token) -> Result<(), ()> {
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

    fn rounds_left(&self) -> u8 {
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

    fn won(&self) -> Option<Token> {
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

    fn score(&self) -> (i32, i32) {
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

fn main() {
    println!("Welcome to the connect four game!");

    // init the game
    // the board is empty
    let mut state = State::new();

    // counter of rounds
    let mut count = 0i32;

    // play until some player wins or there is a draw
    let mut end_of_game = false;
    while !end_of_game {
        // red always start for the moment
        let player = if count % 2 == 0 {
            Token::Red
        } else {
            Token::Yellow
        };

        // show state of the game...
        println!("\n{}\n", state);
        let scores = state.score();
        println!("Red: {}", scores.0);
        println!("Yellow: {}", scores.1);

        // ... and ask player to play
        println!("Player {} plays:", &player);
        let mut valid_mv = false;
        while !valid_mv {
            let mut mv = String::new();
            io::stdin()
                .read_line(&mut mv)
                .expect("Failed to read player's move");

            // validate it's move

            // valid number?
            let mv: usize = mv.trim().parse().expect("Please type a number!");
            if mv >= WIDTH {
                println!("Please type a number between 0 and {}", WIDTH - 1);
                continue;
            }

            // try to add token to the column
            if let Err(()) = state.append(mv, &player) {
                println!("This column is already full");
                continue;
            }

            // move is valid
            valid_mv = true;
        }

        // is this a win?
        match state.won() {
            None => {
                if state.rounds_left() == 0 {
                    println!("This is a draw!");
                    end_of_game = true;
                }
            }
            Some(t) => {
                println!("Player {} wins!", t);
                end_of_game = true;
            }
        }

        // next round
        count += 1;
    }

    // show final state of the game
    println!("\n{}\n", state);
}
