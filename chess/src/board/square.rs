use std::fmt;

use crate::board::Coordinate;
use crate::piece::Piece;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SquareColor {
  Light,
  Dark,
}

impl fmt::Display for SquareColor {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.clone() {
      SquareColor::Light => write!(f, "L"),
      SquareColor::Dark => write!(f, "D"),
    }
  }
}

/// Individual square on a [`Board`](`crate::board::Board`).
pub struct Square {
  color: SquareColor,
  coord: Coordinate,
  piece: Option<Box<dyn Piece>>,
}

impl Square {
  /// Creates a [`Square`](`crate::board::Square`)
  /// with a given [`SquareColor`](`crate::board::SquareColor`).
  pub fn new(color: SquareColor, coord: Coordinate) -> Square {
    Square {
      color,
      coord,
      piece: None,
    }
  }

  /// Returns the [`Square`](`crate::board::Square`)'s [`SquareColor`](`crate::board::SquareColor`).
  pub fn get_color(&self) -> &SquareColor { &self.color }

  pub fn get_coord(&self) -> &Coordinate { &self.coord }

  pub fn get_piece(&self) -> &Option<Box<dyn Piece>> { &self.piece }

  pub fn set_piece(&mut self, piece: Option<Box<dyn Piece>>) { self.piece = piece; }
}