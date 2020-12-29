use crate::piece::{*};

pub struct Knight { color: Color }
impl Knight {
  pub fn new(color: Color) -> Knight {
    Knight { color }
  }
}

impl Piece for Knight {
  fn get_color(&self) -> &Color { &self.color }

  fn get_short_name(&self) -> &'static str { "N" }

  fn get_moves(&self, board: &Board, own_coords: &Coordinate) -> Vec<Coordinate> {
    let mut potential_moves: Vec<Coordinate> = vec!();

    match *own_coords + (1, 2) {
      coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
      coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord);},
      _ => {},
    }
    match *own_coords + (-1, 2) {
      coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
      coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord);},
      _ => {},
    }
    match *own_coords + (1, -2) {
      coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
      coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord);},
      _ => {},
    }
    match *own_coords + (-1, -2) {
      coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
      coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord);},
      _ => {},
    }
    match *own_coords + (2, 1) {
      coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
      coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord);},
      _ => {},
    }
    match *own_coords + (-2, 1) {
      coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
      coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord);},
      _ => {},
    }
    match *own_coords + (2, -1) {
      coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
      coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord);},
      _ => {},
    }
    match *own_coords + (-2, -1) {
      coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
      coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord);},
      _ => {},
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
    let coords = Coordinate { file: File::B, rank: Rank::One };
    let moves = board
      .get_square(coords.clone()).unwrap()
      .get_piece().as_ref().unwrap()
      .get_moves(&board, &coords);

    assert_eq!(moves.len(), 2);
    assert!(moves.contains(&Coordinate { file: File::A, rank: Rank::Three }));
    assert!(moves.contains(&Coordinate { file: File::C, rank: Rank::Three }));
  }

  #[test]
  fn test_get_moves_stop_after_first_capture() {
    let board = Board::from_fen_string("rnbqkbnr/ppp1pppp/8/3p4/8/2N5/PPPPPPPP/R1BQKBNR w KQkq d6 0 2").unwrap();
    let coords = Coordinate { file: File::C, rank: Rank::Three };
    let moves = board
      .get_square(coords.clone()).unwrap()
      .get_piece().as_ref().unwrap()
      .get_moves(&board, &coords);

    assert_eq!(moves.len(), 5);
    assert!(moves.contains(&Coordinate { file: File::A, rank: Rank::Four }));
    assert!(moves.contains(&Coordinate { file: File::B, rank: Rank::Five }));
    assert!(moves.contains(&Coordinate { file: File::B, rank: Rank::One }));
    assert!(moves.contains(&Coordinate { file: File::D, rank: Rank::Five }));
    assert!(moves.contains(&Coordinate { file: File::E, rank: Rank::Four }));
  }
}