use crate::piece::{*};

pub struct Pawn {
  color: Color,
  has_moved: bool,
}

impl Pawn {
  pub fn new(color: Color) -> Pawn {
    Pawn { color, has_moved: false }
  }
}

impl Piece for Pawn {
  fn get_color(&self) -> &Color { &self.color }

  fn get_short_name(&self) -> &'static str { "P" }

  fn get_moves(&self, board: &Board, own_coords: &Coordinate) -> Vec<Coordinate> {
    let forward_move: (i8, i8) = if self.color == Color::White { (0, 1) } else { (0, -1) };

    let mut potential_moves: Vec<Coordinate> = vec![*own_coords + forward_move];

    // Pawns that haven't moved yet can move two squares
    if !self.has_moved {
      potential_moves.push(*own_coords + (forward_move.0, forward_move.1 * 2))
    }

    potential_moves
  }
}

#[cfg(test)]
mod tests {
  use super::{*};
  use crate::board::{*};

  #[test]
  fn get_starting_moves_c2() {
    let board = Board::new();
    let coords = Coordinate { file: File::C, rank: Rank::Two };
    let moves = Board::new()
      .get_square(coords.clone()).unwrap()
      .get_piece().as_ref().unwrap()
      .get_moves(&board, &coords);

    assert!(moves.contains(&Coordinate { file: File::C, rank: Rank::Three }));
    assert!(moves.contains(&Coordinate { file: File::C, rank: Rank::Four }));
  }


  #[test]
  fn get_starting_moves_f7() {
    let board = Board::new();
    let coords = Coordinate { file: File::F, rank: Rank::Seven };
    let moves = Board::new()
      .get_square(coords.clone()).unwrap()
      .get_piece().as_ref().unwrap()
      .get_moves(&board, &coords);

    assert!(moves.contains(&Coordinate { file: File::F, rank: Rank::Six }));
    assert!(moves.contains(&Coordinate { file: File::F, rank: Rank::Five }));
  }
}