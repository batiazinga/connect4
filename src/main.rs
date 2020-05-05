use connect4::{self, State, Token};
use std::io;

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
            if mv >= connect4::WIDTH {
                println!("Please type a number between 0 and {}", connect4::WIDTH - 1);
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
