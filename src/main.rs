use kingdra::board;

fn main() {
    println!("Initializing Kingdra");
    let board: board::Board = board::Board::new();
    board.print_state();
}
