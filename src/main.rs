use connect4::{self, State, Token};
use std::io;

struct Human {
    color: Token,
}

impl Human {
    fn new() -> Human {
        Human { color: Token::Red }
    }
}

impl connect4::Player for Human {
    fn start(&mut self, t: Token) {
        self.color = t;
    }

    fn play(&self, state: &State) -> usize {
        // show state of the game...
        println!("\n{}\n", state);
        let scores = state.score();
        println!("Red: {}", scores.0);
        println!("Yellow: {}", scores.1);

        // ... and ask player to play
        println!("Play:");
        let mut mv = String::new();
        io::stdin()
            .read_line(&mut mv)
            .expect("Failed to read player's move");

        let mv: usize = mv.trim().parse().expect("Please type a number!");
        mv
    }

    fn win(&self, state: &State) {
        println!("\n{}\n", state);
    }

    fn lose(&self, state: &State) {
        println!("\n{}\n", state);
    }

    fn draw(&self, state: &State) {
        println!("\n{}\n", state);
    }
}

fn main() {
    println!("Welcome to the connect four game!");

    connect4::play(Human::new(), Human::new());
}
