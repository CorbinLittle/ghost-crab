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
    fn generate_array_from_board(&self) -> [i8; 64]{
        let mut board_array: [i8; 64] = [0; 64];
        for i in self.board.iter().enumerate(){ 
            let index = i.0;
            let square = i.1;
            let mut square_int: i8;
            match square{
                Square{square: (Piece::Empty, ..)} => square_int = 0,
                Square{square: (Piece::Pawn, ..)} => square_int = 1,
                Square{square: (Piece::Knight, ..)} => square_int = 2,
                Square{square: (Piece::Bishop, ..)} => square_int = 3,
                Square{square: (Piece::Rook, ..)} => square_int = 4,
                Square{square: (Piece::Queen, ..)} => square_int = 5,
                Square{square: (Piece::King, ..)} => square_int = 6,
            }
            match square{
                Square { square: (.., Color::Black) } => square_int *= -1,
                _ => {}
            }
            board_array[index] = square_int

        }
        return board_array;
    }
    fn show(&self){
        let array = self.generate_array_from_board();
        for i in array.iter().enumerate(){
            let index = i.0 + 1;
            let square = i.1;
            print!("{}", square);
            if index % 8 == 0{
                print!("\n")
            }
        }
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
    
    let board = Board::generate_board_from_array(&board_array);
    board.show()

}
