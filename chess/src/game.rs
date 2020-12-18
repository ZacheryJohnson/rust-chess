use crate::piece;
use crate::board;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Coordinates(board::File, board::Rank);

pub struct GameState {
  board: board::Board,
  pieces: HashMap<Coordinates, Box<dyn piece::Piece>>
}

const BOARD_WIDTH: u8 = 8;
const BOARD_HEIGHT: u8 = 8;

impl GameState {
  /// Creates a new chess game with a standard board size.
  pub fn new() -> GameState {
    // White piece setup
    // White pawns
    let mut pieces: HashMap<Coordinates, Box<dyn piece::Piece>> = HashMap::new();
    pieces.insert(
      Coordinates(board::File::A, board::Rank::Two),
      Box::new(piece::Pawn::new(piece::Color::White))
    );
    pieces.insert(
      Coordinates(board::File::B, board::Rank::Two),
      Box::new(piece::Pawn::new(piece::Color::White))
    );
    pieces.insert(
      Coordinates(board::File::C, board::Rank::Two),
      Box::new(piece::Pawn::new(piece::Color::White))
    );
    pieces.insert(
      Coordinates(board::File::D, board::Rank::Two),
      Box::new(piece::Pawn::new(piece::Color::White)),
    );
    pieces.insert(
      Coordinates(board::File::E, board::Rank::Two),
     Box::new(piece::Pawn::new(piece::Color::White))
    );
    pieces.insert(
      Coordinates(board::File::F, board::Rank::Two),
      Box::new(piece::Pawn::new(piece::Color::White))
    );
    pieces.insert(
      Coordinates(board::File::G, board::Rank::Two),
      Box::new(piece::Pawn::new(piece::Color::White))
    );
    pieces.insert(
      Coordinates(board::File::H, board::Rank::Two),
      Box::new(piece::Pawn::new(piece::Color::White))
    );
    // White rooks
    pieces.insert(
      Coordinates(board::File::A, board::Rank::One),
      Box::new(piece::Rook::new(piece::Color::White))
    );
    pieces.insert(
      Coordinates(board::File::H, board::Rank::One),
      Box::new(piece::Rook::new(piece::Color::White))
    );
    // White knights
    pieces.insert(
      Coordinates(board::File::B, board::Rank::One),
      Box::new(piece::Knight::new(piece::Color::White))
    );
    pieces.insert(
      Coordinates(board::File::G, board::Rank::One),
      Box::new(piece::Knight::new(piece::Color::White))
    );
    // White bishops
    pieces.insert(
      Coordinates(board::File::C, board::Rank::One),
      Box::new(piece::Bishop::new(piece::Color::White))
    );
    pieces.insert(
      Coordinates(board::File::F, board::Rank::One),
      Box::new(piece::Bishop::new(piece::Color::White))
    );
    // White Queen
    pieces.insert(
      Coordinates(board::File::D, board::Rank::One),
      Box::new(piece::Queen::new(piece::Color::White))
    );
    // White King
    pieces.insert(
      Coordinates(board::File::E, board::Rank::One),
      Box::new(piece::King::new(piece::Color::White))
    );

    // Black piece setup
    // Black pawns
    pieces.insert(
      Coordinates(board::File::A, board::Rank::Seven),
      Box::new(piece::Pawn::new(piece::Color::Black))
    );
    pieces.insert(
      Coordinates(board::File::B, board::Rank::Seven),
      Box::new(piece::Pawn::new(piece::Color::Black))
    );
    pieces.insert(
      Coordinates(board::File::C, board::Rank::Seven),
      Box::new(piece::Pawn::new(piece::Color::Black))
    );
    pieces.insert(
      Coordinates(board::File::D, board::Rank::Seven),
      Box::new(piece::Pawn::new(piece::Color::Black))
    );
    pieces.insert(
      Coordinates(board::File::E, board::Rank::Seven),
      Box::new(piece::Pawn::new(piece::Color::Black))
    );
    pieces.insert(
      Coordinates(board::File::F, board::Rank::Seven),
      Box::new(piece::Pawn::new(piece::Color::Black))
    );
    pieces.insert(
      Coordinates(board::File::G, board::Rank::Seven),
      Box::new(piece::Pawn::new(piece::Color::Black))
    );
    pieces.insert(
      Coordinates(board::File::H, board::Rank::Seven),
      Box::new(piece::Pawn::new(piece::Color::Black))
    );
    // Black rooks
    pieces.insert(
      Coordinates(board::File::A, board::Rank::Eight),
      Box::new(piece::Rook::new(piece::Color::Black))
    );
    pieces.insert(
      Coordinates(board::File::H, board::Rank::Eight),
      Box::new(piece::Rook::new(piece::Color::Black))
    );
    // Black knights
    pieces.insert(
      Coordinates(board::File::B, board::Rank::Eight),
      Box::new(piece::Knight::new(piece::Color::Black))
    );
    pieces.insert(
      Coordinates(board::File::G, board::Rank::Eight),
      Box::new(piece::Knight::new(piece::Color::Black))
    );
    // Black bishops
    pieces.insert(
      Coordinates(board::File::C, board::Rank::Eight),
      Box::new(piece::Bishop::new(piece::Color::Black))
    );
    pieces.insert(
      Coordinates(board::File::F, board::Rank::Eight),
      Box::new(piece::Bishop::new(piece::Color::Black))
    );
    // Black Queen
    pieces.insert(
      Coordinates(board::File::D, board::Rank::Eight),
      Box::new(piece::Queen::new(piece::Color::Black))
    );
    // Black King
    pieces.insert(
      Coordinates(board::File::E, board::Rank::Eight),
      Box::new(piece::King::new(piece::Color::Black))
    );

    GameState {
      board: board::Board::new(BOARD_WIDTH, BOARD_HEIGHT),
      pieces
    }
  }

  pub fn console_print(&self) {
    for y in (1..=BOARD_HEIGHT).rev() {
      for x in 1..=BOARD_WIDTH {
        let coords = Coordinates(board::File::from(x), board::Rank::from(y));
        match self.pieces.get(&coords) {
          Some(piece) => print!(" {}{} ", piece.get_color(), piece.get_short_name()),
          None => print!(" .. "),
        }
      }

      println!();
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::game::GameState;

  #[test]
  fn print_game_state() {
    let game = GameState::new();
    game.console_print();
  }
}