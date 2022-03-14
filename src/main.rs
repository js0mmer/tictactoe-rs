use std::io::{self, Write};

pub struct Game {
    board: Vec<Vec<String>>,
    turn: bool // true is X false is O
}

impl Game {
    pub fn new() -> Game {
        let mut board: Vec<Vec<String>> = vec![];

        for _ in 0..3 {
            let mut row: Vec<String> = vec![];

            for _ in 0..3 {
                row.push(String::from(" "));
            }

            board.push(row);
        }

        Game {
            board: board,
            turn: false
        }
    }

    pub fn drop_piece(&mut self, row: usize, col: usize) -> Result<(), &'static str> {
        if self.board[row][col] != " " {
            return Err("Cell already filled! Choose another.");
        } else if self.turn {
            self.board[row][col] = String::from("X");
            self.turn = !self.turn;
        } else {
            self.board[row][col] = String::from("O");
            self.turn = !self.turn;
        }

        Ok(())
    }

    pub fn get_winner(&self) -> &str {
        for i in 0..3 { // check rows
            if self.board[i][0] != " " && self.board[i][0] == self.board[i][1] && self.board[i][0] == self.board[i][2] {
                return &self.board[i][0];
            }
        }

        for j in 0..3 { // check cols
            if self.board[0][j] != " " && self.board[0][j] == self.board[1][j] && self.board[0][j] == self.board[2][j] {
                return &self.board[0][j];
            }
        }

        if self.board[0][0] != " " && self.board[0][0] == self.board[1][1] && self.board[0][0] == self.board[2][2] { // check diag top left to bottom right
            return &self.board[0][0];
        }

        if self.board[0][2] != " " && self.board[0][2] == self.board[1][1] && self.board[0][2] == self.board[2][0] { // check diag bottom left to top right
            return &self.board[0][2];
        }

        return " ";
    }
}

fn print_board(game: &Game) {
    let mut row_num = 0;
    println!("             ");
    println!("=============");
    println!("             ");
    println!("  1   2   3");
    for row in game.board.iter() {
        println!("{} {} | {} | {} ", row_num + 1, row[0], row[1], row[2]);
        row_num += 1;
        if row_num < 3 {
            println!(" -----------")
        }
    }
}

fn prompt_input() -> [usize; 2] {
    let mut row = String::new();

    print!("Row: ");
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut row) {
        Ok(_) => {},
        Err(_) => {
            println!("Enter a valid number from 1-3");
            return prompt_input();
        }
    }

    let row: usize = match row.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter a valid number from 1-3");
            return prompt_input();
        }
    };

    let mut col = String::new();

    print!("Column: ");
    io::stdout().flush().unwrap();
    match io::stdin().read_line(&mut col) {
        Ok(_) => {},
        Err(_) => {
            println!("Enter a valid number from 1-3");
            return prompt_input();
        }
    }

    let col: usize = match col.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Enter a valid number from 1-3");
            return prompt_input();
        }
    };

    return [row, col];
}

fn main() {
    println!("Welcome to tic-tac-toe!");

    let mut game = Game::new();

    print_board(&game);

    let mut winner = " ";

    while winner == " " {
        if game.turn {
            println!("X's turn!");
        } else {
            println!("O's turn!");
        }

        let mut valid_input = false;

        while !valid_input {
            let [row, col] = prompt_input();
    
            match game.drop_piece(row - 1, col - 1) {
                Ok(_) => {
                    valid_input = true;
                },
                Err(e) => {
                    println!("{}", e);
                }
            }
        }

        print_board(&game);
        winner = game.get_winner();
    }

    println!("{} wins!", winner);

    print!("Play again (y/n)? ");
    io::stdout().flush().unwrap();

    let mut yes = String::new();

    match io::stdin().read_line(&mut yes) {
        Ok(_) => if yes.trim().to_lowercase() == "y" {
            println!();
            main();
        },
        Err(_) => {}
    }
}
