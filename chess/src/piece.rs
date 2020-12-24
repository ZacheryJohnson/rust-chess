use std::fmt::Display;
use core::fmt;

type CoordinateDelta = (i8, i8);

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
  fn get_moves(&self) -> Vec<CoordinateDelta>;
}

pub struct Pawn { color: Color}
impl Pawn {
  pub fn new(color: Color) -> Pawn {
    Pawn { color }
  }
}

impl Piece for Pawn {
  fn get_color(&self) -> &Color { &self.color }

  fn get_move_type(&self) -> MoveType { MoveType::Linear }

  fn get_short_name(&self) -> &'static str { "P" }

  /// Pawns are the only unit where color matters.
  /// Every other piece can move in any direction
  /// regardless of color.
  fn get_moves(&self) -> Vec<CoordinateDelta> {
    match self.color {
      Color::White => vec!((0, 1)),
      Color::Black => vec!((0, -1)),
    }
  }
}

pub struct Knight { color: Color }
impl Knight {
  pub fn new(color: Color) -> Knight {
    Knight { color }
  }
}

impl Piece for Knight {
  fn get_color(&self) -> &Color { &self.color }

  fn get_move_type(&self) -> MoveType { MoveType::Jump }

  fn get_short_name(&self) -> &'static str { "N" }

  fn get_moves(&self) -> Vec<CoordinateDelta> {
    vec!(
      ( 1,  2), ( 2,  1),
      (-1,  2), (-2,  1),
      (-1, -2), (-2, -1),
      ( 1, -2), ( 2, -1),
    )
  }
}

pub struct Bishop { color: Color }
impl Bishop {
  pub fn new(color: Color) -> Bishop {
    Bishop { color }
  }
}

impl Piece for Bishop {
  fn get_color(&self) -> &Color { &self.color }

  fn get_move_type(&self) -> MoveType { MoveType::Linear }

  fn get_short_name(&self) -> &'static str { "B" }

  fn get_moves(&self) -> Vec<CoordinateDelta> {
    vec!(
      (1,1), (2,2), (3,3), (4,4), (5,5), (6,6), (7,7),
      (-1,1), (-2,2), (-3,3), (-4,4), (-5,5), (-6,6), (-7,7),
      (1,-1), (2,-2), (3,-3), (4,-4), (5,-5), (6,-6), (7,-7),
      (-1,-1), (-2,-2), (-3,-3), (-4,-4), (-5,-5), (-6,-6), (-7,-7),
    )
  }
}

pub struct Rook { color: Color }
impl Rook {
  pub fn new(color: Color) -> Rook {
    Rook { color }
  }
}

impl Piece for Rook {
  fn get_color(&self) -> &Color { &self.color }

  fn get_move_type(&self) -> MoveType { MoveType::Linear }

  fn get_short_name(&self) -> &'static str { "R" }

  fn get_moves(&self) -> Vec<CoordinateDelta> {
    vec!(
      (0,1), (0,2), (0,3), (0,4), (0,5), (0,6), (0,7),
      (0,-1), (0,-2), (0,-3), (0,-4), (0,-5), (0,-6), (0,-7),
      (1,0), (2,0), (3,0), (4,0), (5,0), (6,0), (7,0),
      (-1,0), (-2,0), (-3,0), (-4,0), (-5,0), (-6,0), (-7,0),
    )
  }
}

pub struct Queen { color: Color }
impl Queen {
  pub fn new(color: Color) -> Queen {
    Queen { color }
  }
}

impl Piece for Queen {
  fn get_color(&self) -> &Color { &self.color }

  fn get_move_type(&self) -> MoveType { MoveType::Linear }

  fn get_short_name(&self) -> &'static str { "Q" }

  fn get_moves(&self) -> Vec<CoordinateDelta> {
    vec!(
      // Bishop style moves
      (1,1), (2,2), (3,3), (4,4), (5,5), (6,6), (7,7),
      (-1,1), (-2,2), (-3,3), (-4,4), (-5,5), (-6,6), (-7,7),
      (1,-1), (2,-2), (3,-3), (4,-4), (5,-5), (6,-6), (7,-7),
      (-1,-1), (-2,-2), (-3,-3), (-4,-4), (-5,-5), (-6,-6), (-7,-7),
      // Rook style moves
      (0,1), (0,2), (0,3), (0,4), (0,5), (0,6), (0,7),
      (0,-1), (0,-2), (0,-3), (0,-4), (0,-5), (0,-6), (0,-7),
      (1,0), (2,0), (3,0), (4,0), (5,0), (6,0), (7,0),
      (-1,0), (-2,0), (-3,0), (-4,0), (-5,0), (-6,0), (-7,0),
    )
  }
}

pub struct King { color: Color }
impl King {
  pub fn new(color: Color) -> King {
    King { color }
  }
}

impl Piece for King {
  fn get_color(&self) -> &Color { &self.color }

  fn get_move_type(&self) -> MoveType { MoveType::Linear }

  fn get_short_name(&self) -> &'static str { "K" }

  fn get_moves(&self) -> Vec<CoordinateDelta> {
    vec!(
      (-1,1), (0,1), (1,1),
      (-1,0),         (1,0),
      (-1,-1), (0,-1), (1,-1),
    )
  }
}