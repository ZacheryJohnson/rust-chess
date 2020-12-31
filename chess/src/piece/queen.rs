use crate::piece::{*};

pub struct Queen {
  color: Color,
  position: Coordinate,
}

impl Piece for Queen {
  fn new(color: Color, position: Coordinate) -> Self {
    Queen { color, position }
  }

  fn get_color(&self) -> &Color { &self.color }

  fn get_position(&self) -> &Coordinate { &self.position }

  fn get_short_name(&self) -> &'static str { "Q" }

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
    let coords = Coordinate { file: File::D, rank: Rank::One };
    let moves = board
      .get_square(coords.clone()).unwrap()
      .get_piece().as_ref().unwrap()
      .get_moves(&board, &coords);

    assert_eq!(moves.len(), 0);
  }

  #[test]
  fn test_get_moves_stop_after_first_capture() {
    let board = Board::from_fen_string("rnbqkbnr/ppp2ppp/8/3pp2Q/4P3/8/PPPP1PPP/RNB1KBNR w KQkq d6 0 3").unwrap();
    let coords = Coordinate { file: File::H, rank: Rank::Five };
    let moves = board
      .get_square(coords.clone()).unwrap()
      .get_piece().as_ref().unwrap()
      .get_moves(&board, &coords);

    assert_eq!(moves.len(), 13);
    assert!(moves.contains(&Coordinate { file: File::D, rank: Rank::One }));
    assert!(moves.contains(&Coordinate { file: File::E, rank: Rank::Two }));
    assert!(moves.contains(&Coordinate { file: File::F, rank: Rank::Three }));
    assert!(moves.contains(&Coordinate { file: File::G, rank: Rank::Four }));
    assert!(moves.contains(&Coordinate { file: File::E, rank: Rank::Five }));
    assert!(moves.contains(&Coordinate { file: File::F, rank: Rank::Five }));
    assert!(moves.contains(&Coordinate { file: File::G, rank: Rank::Five }));
    assert!(moves.contains(&Coordinate { file: File::G, rank: Rank::Six }));
    assert!(moves.contains(&Coordinate { file: File::F, rank: Rank::Seven }));
    assert!(moves.contains(&Coordinate { file: File::H, rank: Rank::Seven }));
    assert!(moves.contains(&Coordinate { file: File::H, rank: Rank::Six }));
    assert!(moves.contains(&Coordinate { file: File::H, rank: Rank::Four }));
    assert!(moves.contains(&Coordinate { file: File::H, rank: Rank::Three }));
  }
}