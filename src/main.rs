use std::{io::Write, result};

use tictactoe::{display_board, flip_current_player, update_board, GameObj};

fn main() {
    let mut grid = vec!['-', '-', '-', '-', '-', '-', '-', '-', '-'];
    tictactoe::display_board(&mut grid);

    let mut running = true;

    let symbol = GameObj { x: 'X', o: 'O' };

    let mut current_player = symbol.x;

    while running {
        // managing input
        print!("Enter a number between 1-9: ");

        std::io::stdout().flush().unwrap();

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        input = input.trim().to_string();

        let index: usize;

        match input.parse::<usize>() {
            Ok(e) => index = e,
            Err(_) => continue,
        }

        print!("Passed");
        update_board(&mut grid, &index, &mut current_player, &symbol);
        display_board(&mut grid);
    }
}


