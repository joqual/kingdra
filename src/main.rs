const BOARD_SIZE: usize = 8;
const BOARD_LEN: u8 = 8;
const BOARD_AREA: u8 = BOARD_LEN * BOARD_LEN;

struct Board {
    squares: [Square; 64],
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
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
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Piece {
    role: Role,
    color: Color,
}

fn main() {
    println!("Initialize Board");
    let board: Board = INITIAL_BOARD;
    for i in board.squares {
        println!("{:?}", i);
    }
}

const INITIAL_BOARD: Board = {
    Board {
        squares: [
            // 1st rank (White pieces)
            Square::Full(Piece {
                role: Role::Rook,
                color: Color::White,
            }),
            Square::Full(Piece {
                role: Role::Knight,
                color: Color::White,
            }),
            Square::Full(Piece {
                role: Role::Bishop,
                color: Color::White,
            }),
            Square::Full(Piece {
                role: Role::Queen,
                color: Color::White,
            }),
            Square::Full(Piece {
                role: Role::King,
                color: Color::White,
            }),
            Square::Full(Piece {
                role: Role::Bishop,
                color: Color::White,
            }),
            Square::Full(Piece {
                role: Role::Knight,
                color: Color::White,
            }),
            Square::Full(Piece {
                role: Role::Rook,
                color: Color::White,
            }),
            // 2nd rank (White pawns)
            Square::Full(Piece {
                role: Role::Pawn,
                color: Color::White,
            }),
            Square::Full(Piece {
                role: Role::Pawn,
                color: Color::White,
            }),
            Square::Full(Piece {
                role: Role::Pawn,
                color: Color::White,
            }),
            Square::Full(Piece {
                role: Role::Pawn,
                color: Color::White,
            }),
            Square::Full(Piece {
                role: Role::Pawn,
                color: Color::White,
            }),
            Square::Full(Piece {
                role: Role::Pawn,
                color: Color::White,
            }),
            Square::Full(Piece {
                role: Role::Pawn,
                color: Color::White,
            }),
            Square::Full(Piece {
                role: Role::Pawn,
                color: Color::White,
            }),
            // 3rd rank
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            // 4th rank
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            // 5th rank
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            // 6th rank
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            Square::Empty,
            // 7th rank (Black pawns)
            Square::Full(Piece {
                role: Role::Pawn,
                color: Color::Black,
            }),
            Square::Full(Piece {
                role: Role::Pawn,
                color: Color::Black,
            }),
            Square::Full(Piece {
                role: Role::Pawn,
                color: Color::Black,
            }),
            Square::Full(Piece {
                role: Role::Pawn,
                color: Color::Black,
            }),
            Square::Full(Piece {
                role: Role::Pawn,
                color: Color::Black,
            }),
            Square::Full(Piece {
                role: Role::Pawn,
                color: Color::Black,
            }),
            Square::Full(Piece {
                role: Role::Pawn,
                color: Color::Black,
            }),
            Square::Full(Piece {
                role: Role::Pawn,
                color: Color::Black,
            }),
            // 8th rank (Black pieces)
            Square::Full(Piece {
                role: Role::Rook,
                color: Color::Black,
            }),
            Square::Full(Piece {
                role: Role::Knight,
                color: Color::Black,
            }),
            Square::Full(Piece {
                role: Role::Bishop,
                color: Color::Black,
            }),
            Square::Full(Piece {
                role: Role::Queen,
                color: Color::Black,
            }),
            Square::Full(Piece {
                role: Role::King,
                color: Color::Black,
            }),
            Square::Full(Piece {
                role: Role::Bishop,
                color: Color::Black,
            }),
            Square::Full(Piece {
                role: Role::Knight,
                color: Color::Black,
            }),
            Square::Full(Piece {
                role: Role::Rook,
                color: Color::Black,
            }),
        ],
    }
};
