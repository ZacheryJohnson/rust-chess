use crate::piece::{*};
use crate::board::Rank;

pub struct Pawn {
  color: Color
}

impl Pawn {
  pub fn new(color: Color) -> Pawn {
    Pawn { color }
  }
}

impl Piece for Pawn {
  fn get_color(&self) -> &Color { &self.color }

  fn get_short_name(&self) -> &'static str { "P" }

  fn get_moves(&self, board: &Board, own_coords: &Coordinate) -> Vec<Coordinate> {
    let forward: i8 = if self.color == Color::White { 1 } else { -1 };
    let forward_move: (i8, i8) = (0, forward);

    let mut potential_moves: Vec<Coordinate> = vec!();

    let one_square_move = *own_coords + forward_move;
    match board.get_square(one_square_move) {
      Ok(square) if square.get_piece().as_ref().is_none() => { potential_moves.push(one_square_move); },
      _ => {}
    }

    // Pawns that haven't moved yet can move two squares
    if *self.get_color() == Color::White && own_coords.rank == Rank::Two || *self.get_color() == Color::Black && own_coords.rank == Rank::Seven {
      let two_square_move = *own_coords + (0, forward * 2);
      match board.get_square(two_square_move) {
        Ok(square) if square.get_piece().as_ref().is_none() => { potential_moves.push(two_square_move); },
        _ => {}
      }
    }

    let capture_squares: Vec<Coordinate> = vec![*own_coords + (1, forward), *own_coords + (-1, forward)];
    for capture in capture_squares {
      match board.get_square(capture) {
        Ok(_) if board.can_capture(&capture, &self.color) => { potential_moves.push(capture); },
        Ok(_) if board.get_en_passant_target().as_ref().is_some() && board.get_en_passant_target().as_ref().unwrap() == &capture => { potential_moves.push(capture); }
        _ => {},
      }
    }

    potential_moves
  }
}

#[cfg(test)]
mod tests {
  use crate::board::{*};

  #[test]
  fn test_get_starting_moves_c2() {
    let board = Board::new();
    let coords = Coordinate { file: File::C, rank: Rank::Two };
    let moves = board
      .get_square(coords.clone()).unwrap()
      .get_piece().as_ref().unwrap()
      .get_moves(&board, &coords);

    assert_eq!(moves.len(), 2);
    assert!(moves.contains(&Coordinate { file: File::C, rank: Rank::Three }));
    assert!(moves.contains(&Coordinate { file: File::C, rank: Rank::Four }));
  }

  #[test]
  fn test_get_starting_moves_f7() {
    let board = Board::new();
    let coords = Coordinate { file: File::F, rank: Rank::Seven };
    let moves = board
      .get_square(coords.clone()).unwrap()
      .get_piece().as_ref().unwrap()
      .get_moves(&board, &coords);

    assert_eq!(moves.len(), 2);
    assert!(moves.contains(&Coordinate { file: File::F, rank: Rank::Six }));
    assert!(moves.contains(&Coordinate { file: File::F, rank: Rank::Five }));
  }

  #[test]
  fn test_capture_d4_e5() {
    let board = Board::from_fen_string("rnbqkbnr/pppp1ppp/8/4p3/3P4/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 2").unwrap();
    let coords = Coordinate { file: File::D, rank: Rank::Four };
    let moves = board
      .get_square(coords.clone()).unwrap()
      .get_piece().as_ref().unwrap()
      .get_moves(&board, &coords);

    assert_eq!(moves.len(), 2);
    assert!(moves.contains(&Coordinate { file: File::D, rank: Rank::Five }));
    assert!(moves.contains(&Coordinate { file: File::E, rank: Rank::Five }));
  }

  #[test]
  fn test_cannot_capture_ally() {
    let board = Board::from_fen_string("rnbqkbnr/1ppppppp/8/4P3/p2P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 3").unwrap();
    let coords = Coordinate { file: File::D, rank: Rank::Four };
    let moves = board
      .get_square(coords.clone()).unwrap()
      .get_piece().as_ref().unwrap()
      .get_moves(&board, &coords);

    assert_eq!(moves.len(), 1);
    assert!(moves.contains(&Coordinate { file: File::D, rank: Rank::Five }));
    assert!(!moves.contains(&Coordinate { file: File::E, rank: Rank::Five })); // This is an ally on E5 - assert that it cannot capture that piece
  }

  #[test]
  fn test_capture_en_passant() {
    let board = Board::from_fen_string("rnbqkbnr/ppp2ppp/3p4/3Pp3/8/8/PPP1PPPP/RNBQKBNR w KQkq e6 0 3").unwrap();
    let coords = Coordinate { file: File::D, rank: Rank::Five };
    let moves = board
      .get_square(coords.clone()).unwrap()
      .get_piece().as_ref().unwrap()
      .get_moves(&board, &coords);

    assert_eq!(moves.len(), 1);
    assert!(moves.contains(&Coordinate { file: File::E, rank: Rank::Six })); // En passant capture
  }
}