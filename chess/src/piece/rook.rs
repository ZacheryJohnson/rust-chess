use crate::piece::{*};

pub struct Rook {
  color: Color,
  position: Coordinate,
}

impl Piece for Rook {
  fn new(color: Color, position: Coordinate) -> Self {
    Rook { color, position }
  }

  fn get_color(&self) -> &Color { &self.color }

  fn get_position(&self) -> &Coordinate { &self.position }

  fn get_short_name(&self) -> &'static str { "R" }

  fn get_moves(&self, board: &Board) -> Vec<Coordinate> {
    let mut potential_moves: Vec<Coordinate> = vec!();
    for offset in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
      for i in 1..8 {
        match self.position + (i * offset.0, i * offset.1) {
          coord if !coord.is_valid() => { break; },
          coord if board.can_move(&coord) => { potential_moves.push(coord); },
          coord if board.can_capture(&coord, &self.color) => { potential_moves.push(coord); break; },
          _ => { break; },
        }
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
    let coords = Coordinate { file: File::F, rank: Rank::One };
    let moves = board
      .get_square(coords).unwrap()
      .get_piece().as_ref().unwrap()
      .get_moves(&board);

    assert_eq!(moves.len(), 0);
  }

  #[test]
  fn test_get_moves_stop_after_first_capture() {
    let board = Board::from_fen_string("rnbqkbnr/pp3ppp/4p3/2pp4/P7/3R4/1PPPPPPP/1NBQKBNR w Kkq - 0 4").unwrap();
    let coords = Coordinate { file: File::D, rank: Rank::Three };
    let moves = board
      .get_square(coords).unwrap()
      .get_piece().as_ref().unwrap()
      .get_moves(&board);

    assert_eq!(moves.len(), 9);
    assert!(moves.contains(&Coordinate { file: File::A, rank: Rank::Three }));
    assert!(moves.contains(&Coordinate { file: File::B, rank: Rank::Three }));
    assert!(moves.contains(&Coordinate { file: File::C, rank: Rank::Three }));
    assert!(moves.contains(&Coordinate { file: File::E, rank: Rank::Three }));
    assert!(moves.contains(&Coordinate { file: File::F, rank: Rank::Three }));
    assert!(moves.contains(&Coordinate { file: File::G, rank: Rank::Three }));
    assert!(moves.contains(&Coordinate { file: File::H, rank: Rank::Three }));
    assert!(moves.contains(&Coordinate { file: File::D, rank: Rank::Four }));
    assert!(moves.contains(&Coordinate { file: File::D, rank: Rank::Five }));
  }
}