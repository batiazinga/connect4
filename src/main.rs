use connect4::{self, State, Token};
use rand::Rng;
use std::io;

struct Human {
    name: String,
    color: Token,
}

impl Human {
    fn new(name: &str) -> Human {
        Human {
            name: name.to_string(),
            color: Token::Red,
        }
    }
}

impl connect4::Player for Human {
    fn start(&mut self, t: Token) {
        self.color = t;
        println!("Player {} is playing {}", self.name, self.color);
    }

    fn play(&self, state: &State) -> usize {
        // show state of the game...
        println!("\n{}\n", state);
        let scores = state.score();
        println!("Red: {}", scores.0);
        println!("Yellow: {}", scores.1);

        // ... and ask player to play
        println!("{} plays", self.name);
        let mut mv = String::new();
        io::stdin()
            .read_line(&mut mv)
            .expect("Failed to read player's move");

        let mv: usize = mv.trim().parse().expect("Please type a number!");
        mv
    }

    fn win(&self, state: &State) {
        println!("{} wins!", self.name);
        println!("\n{}\n", state);
    }

    fn lose(&self, state: &State) {
        println!("{} loses!", self.name);
        println!("\n{}\n", state);
    }

    fn draw(&self, state: &State) {
        println!("This is a draw!");
        println!("\n{}\n", state);
    }
}

struct Rand {}

impl connect4::Player for Rand {
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

struct BestNextPly {
    color: Token,
}

impl BestNextPly {
    fn new() -> BestNextPly {
        BestNextPly { color: Token::Red }
    }
}

impl connect4::Player for BestNextPly {
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

            let scores = cp.score();
            let mut score = scores.0;
            if let Token::Yellow = self.color {
                score = scores.1;
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

fn main() {
    println!("Welcome to the connect four game!");

    connect4::play(Human::new("player1"), BestNextPly::new());
}
