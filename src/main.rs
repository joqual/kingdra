use kingdra::board;

fn main() {
    println!("Initialize Board");
    let board: board::Board = board::Board::new();
    for (i, item) in board.squares.iter().enumerate() {
        if i % board::BOARD_SIZE == 0 {
            println!();
        }

        match item {
            board::Square::Border => print!("X"),
            board::Square::Empty => print!("_"),
            board::Square::Full(p) => print!("p"),
        }
    }
}
