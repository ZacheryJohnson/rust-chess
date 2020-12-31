pub mod bishop;
pub mod king;
pub mod knight;
pub mod pawn;
pub mod queen;
pub mod rook;

use std::fmt::Display;
use std::fmt;
use crate::board::{Board, coord::Coordinate};

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

pub trait Piece {
  fn new(color: Color, position: Coordinate) -> Self where Self: Sized;

  fn get_color(&self) -> &Color;

  fn get_position(&self) -> &Coordinate;

  fn get_short_name(&self) -> &'static str;

  /// Returns the legal moves a piece can make given the board state and it's own coordinates.
  fn get_moves(&self, board: &Board) -> Vec<Coordinate>;
}