use colored::Colorize;
use std::io::Write;
use tictactoe::{check_winner, display_board, update_board, GameObj};

fn main() {
    let mut grid = vec!['-', '-', '-', '-', '-', '-', '-', '-', '-'];
    let mut possible_moves = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
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

        if possible_moves.contains(&index) {
            match possible_moves.binary_search(&index) {
                Ok(e) => {
                    possible_moves.remove(e);
                }
                Err(_) => (),
            }

            update_board(&mut grid, &index, &mut current_player, &symbol);
            display_board(&mut grid);

            let (winner, end_game) = check_winner(&grid);

            if end_game {
                println!("The winner is {}", winner);
                running = false;
            }
        } else if possible_moves.is_empty() {
            println!("Tie");
        } else if index >= 10 {
            println!(
                "{}",
                "\n\t\tNumber already is out of bounds ‼️ 😡\n\n"
                    .red()
                    .bold()
            );
            continue;
        } else {
            println!(
                "{}",
                "\n\t\tNumber has already been picked, pick another number beautiful  😃💕 \n\n"
                    .yellow()
                    .bold()
            );
            continue;
        }
    }
}
