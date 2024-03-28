struct Board {
    board: Vec<Square>,
}
impl Board {
    fn generate_board_from_string(board_string: &str) -> Board {
        //0 = empty square
        // 1 = pawn
        // 2 = knight
        // 3 = bishop
        // 4 = rook
        // 5 = queen
        // 6 = king
        // positive means white
        // negative means black
        let mut board = Board { board: vec![] };
        for i in board_string.chars().enumerate() {
            let index: u32 = (i.0 + 1) as u32;
            let char = i.1;
            let piece_int: i32 = char.to_digit(10).unwrap() as i32;
            let x_cord: u32 = index / 8;
            let y_cord: u32 = index % 8;
            match piece_int {
                1 => board.board.push(Square::Pawn {
                    color: Color::White,
                    position: Position {
                        position: (x_cord, y_cord),
                    },
                }),
                2 => board.board.push(Square::Knight {
                    color: Color::White,
                    position: Position {
                        position: (x_cord, y_cord),
                    },
                }),
                3 => board.board.push(Square::Bishop {
                    color: Color::White,
                    position: Position {
                        position: (x_cord, y_cord),
                    },
                }),
                4 => board.board.push(Square::Rook {
                    color: Color::White,
                    position: Position {
                        position: (x_cord, y_cord),
                    },
                }),
                5 => board.board.push(Square::Queen {
                    color: Color::White,
                    position: Position {
                        position: (x_cord, y_cord),
                    },
                }),
                6 => board.board.push(Square::King {
                    color: Color::White,
                    position: Position {
                        position: (x_cord, y_cord),
                    },
                }),
                -1 => board.board.push(Square::Pawn {
                    color: Color::Black,
                    position: Position {
                        position: (x_cord, y_cord),
                    },
                }),
                -2 => board.board.push(Square::Knight {
                    color: Color::Black,
                    position: Position {
                        position: (x_cord, y_cord),
                    },
                }),
                -3 => board.board.push(Square::Bishop {
                    color: Color::Black,
                    position: Position {
                        position: (x_cord, y_cord),
                    },
                }),
                -4 => board.board.push(Square::Rook {
                    color: Color::Black,
                    position: Position {
                        position: (x_cord, y_cord),
                    },
                }),
                -5 => board.board.push(Square::Queen {
                    color: Color::Black,
                    position: Position {
                        position: (x_cord, y_cord),
                    },
                }),
                -6 => board.board.push(Square::King {
                    color: Color::Black,
                    position: Position {
                        position: (x_cord, y_cord),
                    },
                }),
                0 => {
                    if index % 2 == 0 {
                        board.board.push(Square::Empty {
                            color: Color::Black,
                        })
                    } else {
                        board.board.push(Square::Empty {
                            color: Color::White,
                        })
                    }
                }
                _ => panic!("invalid item in board.board"),
            }
        }
        return board;
    }
    fn generate_string_from_board(&self) -> String {
        let mut board_string = String::new();
        for i in self.board.iter() {
            match i {
                Square::Empty { .. } => board_string += "0",
                Square::Pawn {
                    color: Color::White,
                    ..
                } => board_string += "1",
                Square::Pawn {
                    color: Color::Black,
                    ..
                } => board_string += "-1",
                Square::Knight {
                    color: Color::White,
                    ..
                } => board_string += "2",
                Square::Knight {
                    color: Color::Black,
                    ..
                } => board_string += "-2",
                Square::Bishop {
                    color: Color::White,
                    ..
                } => board_string += "3",
                Square::Bishop {
                    color: Color::Black,
                    ..
                } => board_string += "-3",
                Square::Rook {
                    color: Color::White,
                    ..
                } => board_string += "4",
                Square::Rook {
                    color: Color::Black,
                    ..
                } => board_string += "-4",
                Square::Queen {
                    color: Color::White,
                    ..
                } => board_string += "5",
                Square::Queen {
                    color: Color::Black,
                    ..
                } => board_string += "-5",
                Square::King {
                    color: Color::White,
                    ..
                } => board_string += "6",
                Square::King {
                    color: Color::Black,
                    ..
                } => board_string += "-6",
            }
        }
        return board_string;
    }
}
enum Color {
    White,
    Black,
}
enum Square {
    Pawn { color: Color, position: Position },
    Rook { color: Color, position: Position },
    Queen { color: Color, position: Position },
    Bishop { color: Color, position: Position },
    Knight { color: Color, position: Position },
    King { color: Color, position: Position },
    Empty { color: Color },
}
struct Position {
    position: (u32, u32),
}

fn main() {
    let board_string = "4235632411111111000000000000000000000000000000001111111142356324";
    let board = Board::generate_board_from_string(board_string);
    let test_board_string = board.generate_string_from_board();
    if board_string == test_board_string {
        println!("the board_test_string matches the board_string. this means the generate_string_from_board and the generate_board_from_string functions are working")
    } else {
        println!("the test_board_string does not match the board_string. this means that there is a bug in either the generate_board_from_string function or the generate_string_from_board_function")
    }
}
