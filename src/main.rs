use connect4::{self, ai::BruteForce, State, Token};
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

fn main() {
    println!("Welcome to the connect four game!");

    connect4::play(BruteForce::new(5), Human::new("Human"));
}
