use crate::piece::{*};

pub struct King {
  color: Color,
  position: Coordinate,
}

impl Piece for King {
  fn new(color: Color, position: Coordinate) -> Self {
    King { color, position }
  }

  fn get_color(&self) -> &Color { &self.color }

  fn get_position(&self) -> &Coordinate { &self.position }

  fn get_short_name(&self) -> &'static str { "K" }

  fn get_moves(&self, board: &Board) -> Vec<Coordinate> {
    let mut potential_moves: Vec<Coordinate> = vec!();

    match self.position + (0, 1) {
      coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
      coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord); },
      _ => {},
    }

    match self.position + (0, -1) {
      coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
      coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord); },
      _ => {},
    }

    match self.position + (1, 0) {
      coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
      coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord); },
      _ => {},
    }

    match self.position + (-1, 0) {
      coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
      coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord); },
      _ => {},
    }

    match self.position + (1, 1) {
      coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
      coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord); },
      _ => {},
    }

    match self.position + (1, -1) {
      coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
      coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord); },
      _ => {},
    }

    match self.position + (-1, 1) {
      coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
      coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord); },
      _ => {},
    }

    match self.position + (-1, -1) {
      coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
      coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord); },
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
    let coords = Coordinate { file: File::E, rank: Rank::One };
    let moves = board
      .get_square(coords).unwrap()
      .get_piece().as_ref().unwrap()
      .get_moves(&board);

    assert_eq!(moves.len(), 0);
  }

  #[test]
  fn test_get_moves_stop_after_first_capture() {
    let board = Board::from_fen_string("rnbqkbnr/pp2pppp/8/3p4/2pKP3/8/PPPP1PPP/RNBQ1BNR b kq - 1 5").unwrap();
    let coords = Coordinate { file: File::D, rank: Rank::Four };
    let moves = board
      .get_square(coords).unwrap()
      .get_piece().as_ref().unwrap()
      .get_moves(&board);

    assert_eq!(moves.len(), 7);
    assert!(moves.contains(&Coordinate { file: File::C, rank: Rank::Three }));
    assert!(moves.contains(&Coordinate { file: File::C, rank: Rank::Four })); // This one should be illegal and fail tests in the future
    assert!(moves.contains(&Coordinate { file: File::C, rank: Rank::Five }));
    assert!(moves.contains(&Coordinate { file: File::D, rank: Rank::Three })); // This one should also be illegal
    assert!(moves.contains(&Coordinate { file: File::D, rank: Rank::Five })); // This one should also be illegal
    assert!(moves.contains(&Coordinate { file: File::E, rank: Rank::Three }));
    assert!(moves.contains(&Coordinate { file: File::E, rank: Rank::Five }));
  }
}