struct Board {
    board: [Square; 64],
}
impl Board {
    fn generate_board_from_array(board_string: &[i8; 64]) -> Board {
        //0 = empty square, 1 = pawn, 2 = knight, 3 = bishop, 4 = rook, 5 = queen, 6 = king, positive means white, negative means black
        let mut board_vec: Vec<Square> = vec![];
        let mut board_array: [Square; 64] = [Square::Empty; 64];
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
                Square::Empty => square_int = 0,
                Square::Piece { color: _, piece: Piece::Pawn } => square_int = 1,
                Square::Piece { color: _, piece: Piece::Knight } => square_int = 2,
                Square::Piece { color: _, piece: Piece::Bishop } => square_int = 3,
                Square::Piece { color: _, piece: Piece::Rook } => square_int = 4,
                Square::Piece { color: _, piece: Piece::Queen } => square_int = 5,
                Square::Piece { color: _, piece: Piece::King } => square_int = 6,
            }
            match square{
                Square::Piece { color: Color::Black, piece:_ } => square_int *= -1,
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
    fn move_piece(&mut self, locations: ((u8, u8), (u8, u8))){
        let move_from = (locations.0.0 * 8) + locations.0.1;
        let move_to = (locations.1.0 * 8) + locations.1.1;
        self.board[move_to as usize] = self.board[move_from as usize];
        self.board[move_from as usize] =  Square::Empty;
    }
    fn generate_board_from_fen(fen: &str) -> Board{
         let mut board = [Square::Empty; 64];
         let mut index_counter: usize = 0;
         for i in fen.chars(){
            match i{
                'p' => board[index_counter] = Square::Piece { color: Color::Black, piece: Piece::Pawn},
                'n' => board[index_counter] = Square::Piece { color: Color::Black, piece: Piece::Knight},
                'b' => board[index_counter] = Square::Piece { color: Color::Black, piece: Piece::Bishop},
                'r' => board[index_counter] = Square::Piece { color: Color::Black, piece: Piece::Rook},
                'q' => board[index_counter] = Square::Piece { color: Color::Black, piece: Piece::Queen},
                'k' => board[index_counter] = Square::Piece { color: Color::Black, piece: Piece::King},
                'P' => board[index_counter] = Square::Piece { color: Color::White, piece: Piece::Pawn},
                'N' => board[index_counter] = Square::Piece { color: Color::White, piece: Piece::Knight},
                'B' => board[index_counter] = Square::Piece { color: Color::White, piece: Piece::Bishop},
                'R' => board[index_counter] = Square::Piece { color: Color::White, piece: Piece::Rook},
                'Q' => board[index_counter] = Square::Piece { color: Color::White, piece: Piece::Queen},
                'K' => board[index_counter] = Square::Piece { color: Color::White, piece: Piece::King},
                '/' => index_counter -= 1,
                ' ' => panic!("your fen input is invalid. we only suppoert piece placements at the moment"),
                a => index_counter += (a.to_digit(10).unwrap() as usize) - 1
            }
            index_counter += 1
         }
         return  Board{board};
    }
}
#[derive (Clone, Copy)]
enum Color {
    White,
    Black,
}
#[derive (Clone, Copy)]
enum Piece {
    Pawn,
    Rook,
    Queen,
    Bishop,
    Knight,
    King,
}

#[derive (Clone, Copy)]
enum Square{
    Empty,
    Piece{color: Color, piece: Piece}
}
impl From<i8> for Square {
    fn from(piece_int: i8) -> Self {
        match piece_int {
            0 => return Square::Empty,
            1 => return Square::Piece { color: Color::White, piece: Piece::Pawn },
            2 => return Square::Piece { color: Color::White, piece: Piece::Knight },
            3 => return Square::Piece { color: Color::White, piece: Piece::Bishop },
            4 => return Square::Piece { color: Color::White, piece: Piece::Rook },
            5 => return Square::Piece { color: Color::White, piece: Piece::Queen },
            6 => return Square::Piece { color: Color::White, piece: Piece::King },
            -1 => return Square::Piece { color: Color::Black, piece: Piece::Pawn },
            -2 => return Square::Piece { color: Color::Black, piece: Piece::Knight },
            -3 => return Square::Piece { color: Color::Black, piece: Piece::Bishop },
            -4 => return Square::Piece { color: Color::Black, piece: Piece::Rook },
            -5 => return Square::Piece { color: Color::Black, piece: Piece::Queen },
            -6 => return Square::Piece { color: Color::Black, piece: Piece::King },
            _ => panic!("invalid item in board.board"),
        }
    }
}


fn main() {
    let board_array: [i8; 64] = [-4, -2, -3, -5, -6, -3, -2, -4,
                                 -1, -1, -1, -1, -1, -1, -1, -1, 
                                 0, 0, 0, 0, 0, 0, 0, 0, 
                                 0, 0, 0, 0, 0, 0, 0, 0, 
                                 0, 0, 0, 0, 0, 0, 0, 0, 
                                 0, 0, 0, 0, 0, 0, 0, 0, 
                                1, 1, 1, 1, 1, 1, 1, 1, 
                                4, 2, 3, 5, 6, 3, 2, 3];
    
    let board = Board::generate_board_from_array(&board_array);
    board.show();
    let fen_input = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    let test_board = Board::generate_board_from_fen(fen_input);
    test_board.show()

}
