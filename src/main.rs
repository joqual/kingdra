const BOARD_SIZE: usize = 8;
const BOARD_LEN: u8 = 8;
const BOARD_AREA: u8 = BOARD_LEN * BOARD_LEN;

struct Board {
    squares: [Square; 64],
}


enum Square {

    // Square is empty
    Empty,

    // Square is occupied
    Full(Piece),

    // Makes piece movement calculation easier by having non-movable borders outside of the board
    Border,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Role {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    Void
}

enum Color {
    White,
    Black,
}

struct Piece {
    role: Role,
    color: Color,
}

fn main() {
    println!("Initialize Board");

}
