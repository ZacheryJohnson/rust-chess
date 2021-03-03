use crate::piece::{*};

pub struct Knight {
  color: Color,
  position: Coordinate,
}

impl Piece for Knight {
  fn new(color: Color, position: Coordinate) -> Self {
    Knight { color, position }
  }

  fn get_color(&self) -> &Color { &self.color }

  fn get_position(&self) -> &Coordinate { &self.position }

  fn get_short_name(&self) -> &'static str { "N" }

  fn get_moves(&self, board: &Board) -> Vec<Coordinate> {
    let mut potential_moves: Vec<Coordinate> = vec!();

    for offset in &[(1, 2), (1, -2), (-1, 2), (-1, -2), (2, 1), (2, -1), (-2, 1), (-2, -1)] {
      match self.position + *offset {
        coord if !coord.is_valid() => {},
        coord if board.can_move(&coord) => { potential_moves.push(coord); },
        coord if board.can_capture(&coord, &self.color) => { potential_moves.push(coord); },
        _ => {},
      }
    }

    potential_moves
  }
}

#[cfg(test)]
mod tests {
  use crate::board::{Board, coord::Coordinate, file::File, rank::Rank};

  #[test]
  fn test_get_starting_moves_empty() {
    let board = Board::new();
    let coords = Coordinate { file: File::B, rank: Rank::One };
    let moves = board
      .get_square(coords).unwrap()
      .get_piece().as_ref().unwrap()
      .get_moves(&board);

    assert_eq!(moves.len(), 2);
    assert!(moves.contains(&Coordinate { file: File::A, rank: Rank::Three }));
    assert!(moves.contains(&Coordinate { file: File::C, rank: Rank::Three }));
  }

  #[test]
  fn test_get_moves_stop_after_first_capture() {
    let board = Board::from_fen_string("rnbqkbnr/ppp1pppp/8/3p4/8/2N5/PPPPPPPP/R1BQKBNR w KQkq d6 0 2").unwrap();
    let coords = Coordinate { file: File::C, rank: Rank::Three };
    let moves = board
      .get_square(coords).unwrap()
      .get_piece().as_ref().unwrap()
      .get_moves(&board);

    assert_eq!(moves.len(), 5);
    assert!(moves.contains(&Coordinate { file: File::A, rank: Rank::Four }));
    assert!(moves.contains(&Coordinate { file: File::B, rank: Rank::Five }));
    assert!(moves.contains(&Coordinate { file: File::B, rank: Rank::One }));
    assert!(moves.contains(&Coordinate { file: File::D, rank: Rank::Five }));
    assert!(moves.contains(&Coordinate { file: File::E, rank: Rank::Four }));
  }
}