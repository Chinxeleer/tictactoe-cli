use colored::Colorize;
pub fn display_board(grid: &mut Vec<char>) {
    println!("\t\t\t{}", "-------------".green());
    println!(
        "\t\t\t{} {} {} {} {} {} {}",
        "|".green(),
        grid[0],
        "|".green(),
        grid[1],
        "|".green(),
        grid[2].to_string(),
        "|".green()
    );
    println!("\t\t\t{}", "-------------".green());
    println!(
        "\t\t\t{} {} {} {} {} {} {}",
        "|".green(),
        grid[3],
        "|".green(),
        grid[4],
        "|".green(),
        grid[5],
        "|".green()
    );
    println!("\t\t\t{}", "-------------".green());
    println!(
        "\t\t\t{} {} {} {} {} {} {}",
        "|".green(),
        grid[6],
        "|".green(),
        grid[7],
        "|".green(),
        grid[8],
        "|".green()
    );
    println!("\t\t\t{}", "-------------".green());
}
pub struct GameObj {
    pub x: char,
    pub o: char,
}

pub fn flip_current_player(current_player: &mut char, symbol: &GameObj) {
    if *current_player == symbol.x {
        *current_player = symbol.o;
    } else if *current_player == symbol.o {
        *current_player = symbol.x
    } else {
        *current_player = symbol.x
    }
}

pub fn update_board(
    grid: &mut Vec<char>,
    index: &usize,
    current_player: &mut char,
    symbol: &GameObj,
) {
    // update the board using the current player
    grid[*index - 1] = *current_player;
    flip_current_player(current_player, symbol);
}

fn check_rows(grid: &Vec<char>) -> char {
    let result: char;

    let first_row = grid[0] == grid[1] && grid[1] == grid[2];
    let second_row: bool = grid[3] == grid[4] && grid[4] == grid[5];
    let third_row = grid[6] == grid[7] && grid[7] == grid[8];

    if first_row {
        result = grid[0];
    } else if second_row {
        result = grid[3];
    } else if third_row {
        result = grid[6];
    } else {
        result = '-';
    }

    result
}

fn check_columns(grid: &Vec<char>) -> char {
    let result: char;

    let first_col = grid[0] == grid[3] && grid[3] == grid[6];
    let second_col: bool = grid[1] == grid[4] && grid[4] == grid[7];
    let third_col = grid[2] == grid[5] && grid[5] == grid[8];

    if first_col {
        result = grid[0];
    } else if second_col {
        result = grid[1];
    } else if third_col {
        result = grid[2];
    } else {
        result = '-';
    }

    result
}
fn check_diagonals(grid: &Vec<char>) -> char {
    let result: char;

    let first_diag = grid[0] == grid[4] && grid[4] == grid[8];
    let second_diag: bool = grid[2] == grid[4] && grid[4] == grid[6];

    if first_diag {
        result = grid[0];
    } else if second_diag {
        result = grid[2];
    } else {
        result = '-';
    }

    result
}
// return two things in the function
// A bool to stop the game loop and a char for the winner
pub fn check_winner(grid: &Vec<char>) -> (char, bool) {
    let winner: char;

    let end_game: bool;

    if check_rows(grid) != '-' {
        winner = check_rows(grid);
        end_game = true;
    } else if check_diagonals(grid) != '-' {
        winner = check_diagonals(grid);
        end_game = true;
    } else if check_columns(grid) != '-' {
        winner = check_columns(grid);
        end_game = true;
    } else {
        winner = '-';
        end_game = false;
    }

    (winner, end_game)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_current_player() {
        let symbol = GameObj { x: 'X', o: 'O' };
        let mut current_player = symbol.x.clone();

        flip_current_player(&mut current_player, &symbol);

        assert_eq!('O', current_player);
    }
    #[test]
    fn rows_checker() {
        let grid = vec!['X', 'X', 'X', '-', '-', '-', '-', '-', '-'];

        assert_eq!('X', check_rows(&grid))
    }
    #[test]
    fn cols_checker() {
        let grid = vec!['X', '-', '-', 'X', '-', '-', 'X', '-', '-'];

        assert_eq!('X', check_columns(&grid))
    }
    #[test]
    fn diags_checker() {
        let grid = vec!['X', '-', 'X', '-', 'X', '-', 'X', '-', 'X'];

        assert_eq!('X', check_diagonals(&grid))
    }

    #[test]
    fn winner_checker() {
        let grid = vec!['X', '-', '-', 'X', '-', '-', '-', '-', '-'];
        assert_eq!(('-', false), check_winner(&grid))
    }
}
