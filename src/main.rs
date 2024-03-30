struct Board {
    board: Vec<Square>,
}
impl Board {
    fn generate_board_from_array(board_string: &[i8; 64]) -> Board {
        //0 = empty square, 1 = pawn, 2 = knight, 3 = bishop, 4 = rook, 5 = queen, 6 = king, positive means white, negative means black
        let mut board = Board { board: vec![] };
        for i in board_string.iter() {
            let piece_int = i.clone();
            board.board.push(Square::from(piece_int));
        }
        return board;
    }
}
enum Color {
    White,
    Black,
    Empty
}
enum Piece {
    Pawn,
    Rook,
    Queen,
    Bishop,
    Knight,
    King,
    Empty,
}
impl From<i8> for Square {
    fn from(piece_int: i8) -> Self {
        match piece_int {
            0 => return Square{Square: (Piece::Empty, Color::Empty)},
            1 => return Square{Square: (Piece::Pawn, Color::White)},
            2 => return Square{Square: (Piece::Knight, Color::White)},
            3 => return Square{Square: (Piece::Bishop, Color::White)},
            4 => return Square{Square: (Piece::Rook, Color::White)},
            5 => return Square{Square: (Piece::Queen, Color::White)},
            6 => return Square{Square: (Piece::King, Color::White)},
            -1 => return Square{Square: (Piece::Pawn, Color::Black)},
            -2 => return Square{Square: (Piece::Knight, Color::Black)},
            -3 => return Square{Square: (Piece::Bishop, Color::Black)},
            -4 => return Square{Square: (Piece::Rook, Color::Black)},
            -5 => return Square{Square: (Piece::Queen, Color::Black)},
            -6 => return Square{Square: (Piece::King, Color::Black)},
            _ => panic!("invalid item in board.board"),
        }
    }
}
struct Square{
    Square: (Piece, Color)
}
struct Position {
    position: (u32, u32),
}

fn main() {
    let board_array: [i8; 64] = [4, 2, 3, 5, 6, 3, 2, 4,
                                 1, 1, 1, 1, 1, 1, 1, 1, 
                                 0, 0, 0, 0, 0, 0, 0, 0, 
                                 0, 0, 0, 0, 0, 0, 0, 0, 
                                 0, 0, 0, 0, 0, 0, 0, 0, 
                                 0, 0, 0, 0, 0, 0, 0, 0, 
                                -1, -1, -1, -1, -1, -1, -1, -1, 
                                -4, -2, -3, -5, -6, -3, -2, -3];
    
    let mut board = Board::generate_board_from_array(&board_array);
}
