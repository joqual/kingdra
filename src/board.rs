pub const BOARD_SIZE: usize = 12;
pub const BOARD_START: usize = 2;
pub const BOARD_END: usize = 10;
pub const BOARD_AREA: usize = BOARD_SIZE * BOARD_SIZE;
pub const NUM_RANKS: usize = BOARD_END - BOARD_START;

#[derive(Copy, Clone, Debug)]
pub enum Square {
    // Square is empty
    Empty,

    // Square is occupied
    Full(Piece),

    // Makes piece movement calculation easier by having non-movable borders outside of the board
    Border,
}

#[derive(Copy, Clone, Debug)]
enum Role {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Copy, Clone, Debug)]
enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    role: Role,
    color: Color,
}

impl Piece {
    pub fn as_char(&self) -> &'static str {
        match self.role {
            Role::Pawn => "p",
            Role::Rook => "R",
            Role::Knight => "N",
            Role::Bishop => "B",
            Role::Queen => "Q",
            Role::King => "K",
        }
    }

    pub fn as_fancy(&self) -> &'static str {
        match self.role {
            Role::Pawn => "♙",
            Role::Rook => "♖",
            Role::Knight => "♘",
            Role::Bishop => "♗",
            Role::Queen => "♕",
            Role::King => "♔",
        }
    }

    pub fn valid_moves(self, board_state: &Board) -> Vec<&str>{
        return vec!["TODO!"]
    }

}

pub struct Board {
    pub squares: [[Square; BOARD_SIZE]; BOARD_SIZE],
}

impl Board {
    // Helper to create a board with padding (borders) and initial chess setup
    // It's really ugly
    pub fn new() -> Self {
        let mut squares = [[Square::Border; BOARD_SIZE]; BOARD_SIZE];

        // Place all inner squares as Empty first
        for rank in BOARD_START..BOARD_END {
            for file in BOARD_START..BOARD_END {
                squares[rank][file] = Square::Empty;
            }
        }

        // Place pawns
        for file in 0..NUM_RANKS {
            // White pawns
            squares[BOARD_START + 1][BOARD_START + file] = Square::Full(Piece {
                role: Role::Pawn,
                color: Color::White,
            });
            // Black pawns
            squares[BOARD_END - 2][BOARD_START + file] = Square::Full(Piece {
                role: Role::Pawn,
                color: Color::Black,
            });
        }

        // Place white pieces
        squares[BOARD_START][BOARD_START + 0] = Square::Full(Piece { role: Role::Rook, color: Color::White });
        squares[BOARD_START][BOARD_START + 1] = Square::Full(Piece { role: Role::Knight, color: Color::White });
        squares[BOARD_START][BOARD_START + 2] = Square::Full(Piece { role: Role::Bishop, color: Color::White });
        squares[BOARD_START][BOARD_START + 3] = Square::Full(Piece { role: Role::Queen, color: Color::White });
        squares[BOARD_START][BOARD_START + 4] = Square::Full(Piece { role: Role::King, color: Color::White });
        squares[BOARD_START][BOARD_START + 5] = Square::Full(Piece { role: Role::Bishop, color: Color::White });
        squares[BOARD_START][BOARD_START + 6] = Square::Full(Piece { role: Role::Knight, color: Color::White });
        squares[BOARD_START][BOARD_START + 7] = Square::Full(Piece { role: Role::Rook, color: Color::White });

        // Place black pieces
        squares[BOARD_END - 1][BOARD_START + 0] = Square::Full(Piece { role: Role::Rook, color: Color::Black });
        squares[BOARD_END - 1][BOARD_START + 1] = Square::Full(Piece { role: Role::Knight, color: Color::Black });
        squares[BOARD_END - 1][BOARD_START + 2] = Square::Full(Piece { role: Role::Bishop, color: Color::Black });
        squares[BOARD_END - 1][BOARD_START + 3] = Square::Full(Piece { role: Role::Queen, color: Color::Black });
        squares[BOARD_END - 1][BOARD_START + 4] = Square::Full(Piece { role: Role::King, color: Color::Black });
        squares[BOARD_END - 1][BOARD_START + 5] = Square::Full(Piece { role: Role::Bishop, color: Color::Black });
        squares[BOARD_END - 1][BOARD_START + 6] = Square::Full(Piece { role: Role::Knight, color: Color::Black });
        squares[BOARD_END - 1][BOARD_START + 7] = Square::Full(Piece { role: Role::Rook, color: Color::Black });

        Board { squares }
    }

    pub fn print_state(self) {
        for row in self.squares.iter() {
            for square in row.iter() {
                match square {
                    Square::Border => (),
                    Square::Empty => print!(" ■ "),
                    Square::Full(p) => print!(" {:} ", p.as_fancy()),
                }
            }
            println!();
        }
    }
}
