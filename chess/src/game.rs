// use crate::piece;
// use crate::board;
// use std::collections::HashMap;
// use std::hash::Hash;
// use std::fmt::Display;
// use std::fmt;
//
// #[cfg(test)]
// mod tests {
//   use crate::game::{*};
//
//   #[test]
//   fn test_print_game_state() {
//     let game_state = GameState::new();
//     game_state.console_print();
//   }
//
//   fn validate_legal_moves(expected: Vec<Coordinates>, actual: Vec<Coordinates>) {
//     assert_eq!(expected.len(), actual.len());
//     for potential_move in actual.iter() {
//       assert!(expected.contains(potential_move));
//     }
//   }
//
//   #[test]
//   fn test_get_valid_moves_pawn_d2_at_game_start() {
//     let game_state = GameState::new();
//     let coords = Coordinates(board::File::D, board::Rank::Two);
//     let legal_moves = game_state.get_legal_moves(&coords);
//
//     let expected_legal_moves = vec!(
//       Coordinates(board::File::D, board::Rank::Three),
//     );
//
//     validate_legal_moves(expected_legal_moves, legal_moves);
//   }
//
//   #[test]
//   fn test_get_valid_moves_bishop_c1_at_game_start() {
//     let game_state = GameState::new();
//     let coords = Coordinates(board::File::C, board::Rank::One);
//     let legal_moves = game_state.get_legal_moves(&coords);
//
//     let expected_legal_moves = vec!();
//     validate_legal_moves(expected_legal_moves, legal_moves);
//   }
//
//   #[test]
//   fn test_get_valid_moves_rook_a1_at_game_start() {
//     let game_state = GameState::new();
//     let coords = Coordinates(board::File::A, board::Rank::One);
//     let legal_moves = game_state.get_legal_moves(&coords);
//
//     let expected_legal_moves = vec!();
//     validate_legal_moves(expected_legal_moves, legal_moves);
//   }
//
//   #[test]
//   fn test_get_valid_moves_knight_b1_at_game_start() {
//     let game_state = GameState::new();
//     let coords = Coordinates(board::File::B, board::Rank::One);
//     let legal_moves = game_state.get_legal_moves(&coords);
//
//     let expected_legal_moves = vec!(
//       Coordinates(board::File::A, board::Rank::Three),
//       Coordinates(board::File::C, board::Rank::Three)
//     );
//
//     validate_legal_moves(expected_legal_moves, legal_moves);
//   }
// }