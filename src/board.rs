pub const BOARD_SIZE: usize = 12;
pub const BOARD_START: usize = 2;
pub const BOARD_END: usize = 10;
pub const BOARD_AREA: usize = BOARD_SIZE * BOARD_SIZE;

pub struct Board {
    pub squares: [Square; BOARD_AREA],
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Square {
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
pub struct Piece {
    role: Role,
    color: Color,
}

impl Board {
    // Helper to create a board with padding (borders) and initial chess setup
    pub const fn new() -> Self {
        let mut squares = [Square::Border; BOARD_AREA];

        // Place all inner squares as Empty first
        let mut rank = BOARD_START;
        while rank < BOARD_END {
            let mut file = BOARD_START;
            while file < BOARD_END {
                squares[rank * BOARD_SIZE + file] = Square::Empty;
                file += 1;
            }
            rank += 1;
        }

        // Place white pieces
        squares[BOARD_START * BOARD_SIZE + BOARD_START + 0] = Square::Full(Piece {
            role: Role::Rook,
            color: Color::White,
        });
        squares[BOARD_START * BOARD_SIZE + BOARD_START + 1] = Square::Full(Piece {
            role: Role::Knight,
            color: Color::White,
        });
        squares[BOARD_START * BOARD_SIZE + BOARD_START + 2] = Square::Full(Piece {
            role: Role::Bishop,
            color: Color::White,
        });
        squares[BOARD_START * BOARD_SIZE + BOARD_START + 3] = Square::Full(Piece {
            role: Role::Queen,
            color: Color::White,
        });
        squares[BOARD_START * BOARD_SIZE + BOARD_START + 4] = Square::Full(Piece {
            role: Role::King,
            color: Color::White,
        });
        squares[BOARD_START * BOARD_SIZE + BOARD_START + 5] = Square::Full(Piece {
            role: Role::Bishop,
            color: Color::White,
        });
        squares[BOARD_START * BOARD_SIZE + BOARD_START + 6] = Square::Full(Piece {
            role: Role::Knight,
            color: Color::White,
        });
        squares[BOARD_START * BOARD_SIZE + BOARD_START + 7] = Square::Full(Piece {
            role: Role::Rook,
            color: Color::White,
        });
        let mut file = 0;
        while file < 8 {
            squares[(BOARD_START + 1) * BOARD_SIZE + BOARD_START + file] = Square::Full(Piece {
                role: Role::Pawn,
                color: Color::White,
            });
            file += 1;
        }

        // Place black pieces
        squares[(BOARD_END - 1) * BOARD_SIZE + BOARD_START + 0] = Square::Full(Piece {
            role: Role::Rook,
            color: Color::Black,
        });
        squares[(BOARD_END - 1) * BOARD_SIZE + BOARD_START + 1] = Square::Full(Piece {
            role: Role::Knight,
            color: Color::Black,
        });
        squares[(BOARD_END - 1) * BOARD_SIZE + BOARD_START + 2] = Square::Full(Piece {
            role: Role::Bishop,
            color: Color::Black,
        });
        squares[(BOARD_END - 1) * BOARD_SIZE + BOARD_START + 3] = Square::Full(Piece {
            role: Role::Queen,
            color: Color::Black,
        });
        squares[(BOARD_END - 1) * BOARD_SIZE + BOARD_START + 4] = Square::Full(Piece {
            role: Role::King,
            color: Color::Black,
        });
        squares[(BOARD_END - 1) * BOARD_SIZE + BOARD_START + 5] = Square::Full(Piece {
            role: Role::Bishop,
            color: Color::Black,
        });
        squares[(BOARD_END - 1) * BOARD_SIZE + BOARD_START + 6] = Square::Full(Piece {
            role: Role::Knight,
            color: Color::Black,
        });
        squares[(BOARD_END - 1) * BOARD_SIZE + BOARD_START + 7] = Square::Full(Piece {
            role: Role::Rook,
            color: Color::Black,
        });
        let mut file = 0;
        while file < 8 {
            squares[(BOARD_END - 2) * BOARD_SIZE + BOARD_START + file] = Square::Full(Piece {
                role: Role::Pawn,
                color: Color::Black,
            });
            file += 1;
        }

        Board { squares }
    }
}
