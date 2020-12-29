use crate::piece::{*};

pub struct Rook { color: Color }
impl Rook {
  pub fn new(color: Color) -> Rook {
    Rook { color }
  }
}

impl Piece for Rook {
  fn get_color(&self) -> &Color { &self.color }

  fn get_short_name(&self) -> &'static str { "R" }

  fn get_moves(&self, board: &Board, own_coords: &Coordinate) -> Vec<Coordinate> {
    let mut potential_moves: Vec<Coordinate> = vec!();
    for i in 1..8 {
      match *own_coords + (0, i) {
        coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
        coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord); break; },
        _ => { break; },
      }
    }

    for i in 1..8 {
      match *own_coords + (0, -i) {
        coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
        coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord); break; },
        _ => { break; },
      }
    }

    for i in 1..8 {
      match *own_coords + (i, 0) {
        coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
        coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord); break; },
        _ => { break; },
      }
    }

    for i in 1..8 {
      match *own_coords + (-i, 0) {
        coord if coord.is_valid() && board.can_move(&coord, &self.color) => { potential_moves.push(coord); },
        coord if coord.is_valid() && board.can_capture(&coord, &self.color) => { potential_moves.push(coord); break; },
        _ => {break;},
      }
    }

    println!("{:?}", potential_moves);

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
  fn test_get_moves_stop_after_first_capture() {
    let board = Board::from_fen_string("rnbqkbnr/pp3ppp/4p3/2pp4/P7/3R4/1PPPPPPP/1NBQKBNR w Kkq - 0 4").unwrap();
    let coords = Coordinate { file: File::D, rank: Rank::Three };
    let moves = board
      .get_square(coords.clone()).unwrap()
      .get_piece().as_ref().unwrap()
      .get_moves(&board, &coords);

    println!("{:?}", moves);

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