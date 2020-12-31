use crate::piece::{*};

pub struct Bishop { color: Color }
impl Bishop {
  pub fn new(color: Color) -> Bishop {
    Bishop { color }
  }
}

impl Piece for Bishop {
  fn get_color(&self) -> &Color { &self.color }

  fn get_short_name(&self) -> &'static str { "B" }

  fn get_moves(&self, board: &Board, own_coords: &Coordinate) -> Vec<Coordinate> {
    let mut potential_moves: Vec<Coordinate> = vec!();
    for i in 1..8 {
      match *own_coords + (i, i) {
        coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
        coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord); break; },
        _ => { break; },
      }
    }

    for i in 1..8 {
      match *own_coords + (i, -i) {
        coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
        coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord); break; },
        _ => { break; },
      }
    }

    for i in 1..8 {
      match *own_coords + (-i, i) {
        coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
        coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord); break; },
        _ => { break; },
      }
    }

    for i in 1..8 {
      match *own_coords + (-i, -i) {
        coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
        coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord); break; },
        _ => {break;},
      }
    }

    potential_moves
  }
}

#[cfg(test)]
mod tests {
  use crate::board::{*};

  #[test]
  fn test_get_starting_moves_empty() {
    let board = Board::new();
    let coords = Coordinate { file: File::F, rank: Rank::One };
    let moves = board
      .get_square(coords.clone()).unwrap()
      .get_piece().as_ref().unwrap()
      .get_moves(&board, &coords);

    assert_eq!(moves.len(), 0);
  }

  #[test]
  fn test_get_moves_after_1e4() {
    let board = Board::from_fen_string("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq e6 0 2").unwrap();
    let coords = Coordinate { file: File::F, rank: Rank::One };
    let moves = board
      .get_square(coords.clone()).unwrap()
      .get_piece().as_ref().unwrap()
      .get_moves(&board, &coords);

    assert_eq!(moves.len(), 5);
    assert!(moves.contains(&Coordinate { file: File::E, rank: Rank::Two }));
    assert!(moves.contains(&Coordinate { file: File::D, rank: Rank::Three }));
    assert!(moves.contains(&Coordinate { file: File::C, rank: Rank::Four }));
    assert!(moves.contains(&Coordinate { file: File::B, rank: Rank::Five }));
    assert!(moves.contains(&Coordinate { file: File::A, rank: Rank::Six }));
  }

  #[test]
  fn test_get_moves_stop_after_first_capture() {
    let board = Board::from_fen_string("rnbqkbnr/ppp2ppp/3p4/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 3").unwrap();
    let coords = Coordinate { file: File::C, rank: Rank::Four };
    let moves = board
      .get_square(coords.clone()).unwrap()
      .get_piece().as_ref().unwrap()
      .get_moves(&board, &coords);

    assert_eq!(moves.len(), 9);
    assert!(moves.contains(&Coordinate { file: File::B, rank: Rank::Five }));
    assert!(moves.contains(&Coordinate { file: File::A, rank: Rank::Six }));
    assert!(moves.contains(&Coordinate { file: File::D, rank: Rank::Five }));
    assert!(moves.contains(&Coordinate { file: File::E, rank: Rank::Six }));
    assert!(moves.contains(&Coordinate { file: File::F, rank: Rank::Seven }));
    assert!(moves.contains(&Coordinate { file: File::B, rank: Rank::Three }));
    assert!(moves.contains(&Coordinate { file: File::D, rank: Rank::Three }));
    assert!(moves.contains(&Coordinate { file: File::E, rank: Rank::Two }));
    assert!(moves.contains(&Coordinate { file: File::F, rank: Rank::One }));
  }
}