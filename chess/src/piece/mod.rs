pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

use std::fmt::Display;
use std::fmt;
use crate::board::Coordinate;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
  White,
  Black,
}

impl Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.clone() {
      Color::White => write!(f, "W"),
      Color::Black => write!(f, "B"),
    }
  }
}

#[derive(PartialEq)]
pub enum MoveType {
  /// Linear pieces must stop upon encountering any piece.
  /// All standard chess pieces are linear except the knight.
  Linear,

  /// Jump pieces can "jump" over other pieces and aren't stopped
  /// if any pieces would be in their path if travelling linearly.
  /// The knight is the only jump piece.
  Jump,
}

pub trait Piece {
  fn get_color(&self) -> &Color;

  fn get_move_type(&self) -> MoveType;

  fn get_short_name(&self) -> &'static str;

  /// Returns the legal moves a piece can make in isolation.
  /// This does NOT consider any other pieces - imagine the piece alone on an empty board.
  /// This means that pawns can't capture (normally or en passant), pieces don't need to
  /// respect check/checkmate, etc.
  fn get_moves(&self) -> Vec<Coordinate>;
}