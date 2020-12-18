use crate::piece;
use crate::board;
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Display;
use std::fmt;

// TODO: move this to board to standardize (File, Rank)
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Coordinates(board::File, board::Rank);

impl Display for Coordinates {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", self.0, self.1)
  }
}

pub struct GameState {
  board: board::Board,
  pieces: HashMap<Coordinates, Box<dyn piece::Piece>>
}

const BOARD_WIDTH: i8 = 8;
const BOARD_HEIGHT: i8 = 8;

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
      board: board::Board::new(BOARD_WIDTH as u8, BOARD_HEIGHT as u8),
      pieces
    }
  }

  pub fn get_legal_moves(
    &self,
    coords: &Coordinates
  ) -> Vec<Coordinates> {
    match self.pieces.get(coords) {
      Some(piece) => {
        let mut potential_moves: Vec<Coordinates> = vec!();
        let mut illegal_moves: Vec<Coordinates> = vec!();
        for potential_move in &piece.get_moves() {
          let potential_file = board::File::from(&coords.0.into() + &potential_move.0);
          let potential_rank = board::Rank::from(&coords.1.into() + &potential_move.1);

          // Verify the potential coordinate is valid (on the board)
          if potential_rank == board::Rank::Invalid || potential_file == board::File::Invalid {
            continue;
          }

          // Verify the potential coordinate is unoccupied by an ally
          let potential_coord = Coordinates(potential_file, potential_rank);
          let potential_piece = self.pieces.get(&potential_coord);
          if potential_piece.is_some() && *potential_piece.unwrap().get_color() == *piece.get_color() {
            illegal_moves.push(Coordinates(potential_file, potential_rank));
            continue;
          }

          potential_moves.push(Coordinates(potential_file, potential_rank));
        }

        // Prune illegal moves if the piece moves linearly
        let mut legal_moves: Vec<Coordinates> = vec!();
        if piece.get_move_type() == piece::MoveType::Linear {
          for potential_move in potential_moves {
            if is_linear_move_illegal(coords, &potential_move, &illegal_moves) {
              continue;
            }

            legal_moves.push(potential_move);
          }
        }
        else {
          legal_moves = potential_moves;
        }

        legal_moves
      },
      None => {
        vec!()
      },
    }
  }

  pub fn console_print(&self) {
    for y in (1..=BOARD_HEIGHT).rev() {
      for x in 1..=BOARD_WIDTH {
        let coords = Coordinates(board::File::from(x), board::Rank::from(y));
        match self.pieces.get(&coords) {
          Some(piece) => print!("|{}{}", piece.get_color(), piece.get_short_name()),
          None => {
            let square = self.board.get_square(coords.1, coords.0);
            match square.get_color() {
              board::SquareColor::Dark  => print!("|::"),
              board::SquareColor::Light => print!("|  "),
            }
          },
        }
      }

      println!("|");
    }
  }
}

fn is_linear_move_illegal(
  piece: &Coordinates,
  dest: &Coordinates,
  potential_blockers: &Vec<Coordinates>
) -> bool {
  let file_movement: i8 = if piece.0 < dest.0 { 1 } else if piece.0 > dest.0 { -1 } else { 0 };
  let rank_movement: i8 = if piece.1 < dest.1 { 1 } else if piece.1 > dest.1 { -1 } else { 0 };

  let mut moving_coord = Coordinates(piece.0, piece.1);

  let mut illegal = false;
  while moving_coord.0 != dest.0 || moving_coord.1 != dest.1 {
    if potential_blockers.contains(&moving_coord) {
      illegal = true;
      break;
    }

    moving_coord = Coordinates(
      board::File::from(moving_coord.0 as i8 + file_movement),
      board::Rank::from(moving_coord.1 as i8 + rank_movement)
    );
  };

  illegal
}

#[cfg(test)]
mod tests {
  use crate::game::{*};

  #[test]
  fn test_print_game_state() {
    let game_state = GameState::new();
    game_state.console_print();
  }

  fn validate_legal_moves(expected: Vec<Coordinates>, actual: Vec<Coordinates>) {
    assert_eq!(expected.len(), actual.len());
    for potential_move in actual.iter() {
      assert!(expected.contains(potential_move));
    }
  }

  #[test]
  fn test_get_valid_moves_pawn_d2_at_game_start() {
    let game_state = GameState::new();
    let coords = Coordinates(board::File::D, board::Rank::Two);
    let legal_moves = game_state.get_legal_moves(&coords);

    let expected_legal_moves = vec!(
      Coordinates(board::File::D, board::Rank::Three),
    );

    validate_legal_moves(expected_legal_moves, legal_moves);
  }

  #[test]
  fn test_get_valid_moves_bishop_c1_at_game_start() {
    let game_state = GameState::new();
    let coords = Coordinates(board::File::C, board::Rank::One);
    let legal_moves = game_state.get_legal_moves(&coords);

    let expected_legal_moves = vec!();
    validate_legal_moves(expected_legal_moves, legal_moves);
  }

  #[test]
  fn test_get_valid_moves_rook_a1_at_game_start() {
    let game_state = GameState::new();
    let coords = Coordinates(board::File::A, board::Rank::One);
    let legal_moves = game_state.get_legal_moves(&coords);

    let expected_legal_moves = vec!();
    validate_legal_moves(expected_legal_moves, legal_moves);
  }

  #[test]
  fn test_get_valid_moves_knight_b1_at_game_start() {
    let game_state = GameState::new();
    let coords = Coordinates(board::File::B, board::Rank::One);
    let legal_moves = game_state.get_legal_moves(&coords);

    let expected_legal_moves = vec!(
      Coordinates(board::File::A, board::Rank::Three),
      Coordinates(board::File::C, board::Rank::Three)
    );

    validate_legal_moves(expected_legal_moves, legal_moves);
  }
}