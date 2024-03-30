struct Board {
    board: [Square; 64],
}
impl Board {
    fn generate_board_from_array(board_string: &[i8; 64]) -> Board {
        //0 = empty square, 1 = pawn, 2 = knight, 3 = bishop, 4 = rook, 5 = queen, 6 = king, positive means white, negative means black
        let mut board_vec: Vec<Square> = vec![];
        let mut board_array: [Square; 64] = [Square{square: (Piece::Empty, Color::Empty)}; 64];
        for i in board_string.iter() {
            let piece_int = i.clone();
            board_vec.push(Square::from(piece_int));
        }
        for i in board_vec.iter().enumerate(){
            board_array[i.0] = *i.1
        }
        return Board{board: board_array};
    }
}
#[derive (Clone, Copy)]
enum Color {
    White,
    Black,
    Empty
}
#[derive (Clone, Copy)]
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
            0 => return Square{square: (Piece::Empty, Color::Empty)},
            1 => return Square{square: (Piece::Pawn, Color::White)},
            2 => return Square{square: (Piece::Knight, Color::White)},
            3 => return Square{square: (Piece::Bishop, Color::White)},
            4 => return Square{square: (Piece::Rook, Color::White)},
            5 => return Square{square: (Piece::Queen, Color::White)},
            6 => return Square{square: (Piece::King, Color::White)},
            -1 => return Square{square: (Piece::Pawn, Color::Black)},
            -2 => return Square{square: (Piece::Knight, Color::Black)},
            -3 => return Square{square: (Piece::Bishop, Color::Black)},
            -4 => return Square{square: (Piece::Rook, Color::Black)},
            -5 => return Square{square: (Piece::Queen, Color::Black)},
            -6 => return Square{square: (Piece::King, Color::Black)},
            _ => panic!("invalid item in board.board"),
        }
    }
}
#[derive (Clone, Copy)]
struct Square{
    square: (Piece, Color)
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
    
    let _board = Board::generate_board_from_array(&board_array);
}
