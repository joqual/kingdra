use kingdra::board;

fn main() {
    let board: board::Board = board::Board::new();
    board.print_state();
}
