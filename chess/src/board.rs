use std::{vec};
use std::fmt::Display;
use core::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
  InvalidPositionString
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SquareColor {
  Light,
  Dark,
}

impl Display for SquareColor {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.clone() {
      SquareColor::Light => write!(f, "L"),
      SquareColor::Dark => write!(f, "D"),
    }
  }}

/// Individual square on a [`Board`](`crate::board::Board`). Only has a color.
pub struct Square {
  color: SquareColor
}

impl Square {
  /// Creates a [`Square`](`crate::board::Square`)
  /// with a given [`SquareColor`](`crate::board::SquareColor`).
  pub fn new(color: SquareColor) -> Square {
    Square {
      color
    }
  }

  /// Returns the [`Square`](`crate::board::Square`)'s [`SquareColor`](`crate::board::SquareColor`).
  pub fn get_color(&self) -> &SquareColor {
    &self.color
  }
}

#[derive(Debug, PartialEq)]
pub enum Rank {
  One = 1,
  Two = 2,
  Three = 3,
  Four = 4,
  Five = 5,
  Six = 6,
  Seven = 7,
  Eight = 8,
}

impl Into<u8> for Rank {
  fn into(self) -> u8 {
    match self {
      Rank::One => 1,
      Rank::Two => 2,
      Rank::Three => 3,
      Rank::Four => 4,
      Rank::Five => 5,
      Rank::Six => 6,
      Rank::Seven => 7,
      Rank::Eight => 8,
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum File {
  A = 1,
  B = 2,
  C = 3,
  D = 4,
  E = 5,
  F = 6,
  G = 7,
  H = 8,
}

impl Into<u8> for File {
  fn into(self) -> u8 {
    match self {
      File::A => 1,
      File::B => 2,
      File::C => 3,
      File::D => 4,
      File::E => 5,
      File::F => 6,
      File::G => 7,
      File::H => 8,
    }
  }
}

/// Collection of [`Square`](`crate::board::Square`)s arranged in a rectangle
/// given a width and height.
pub struct Board {
  squares: Vec<Square>,
  width: u8,
  height: u8
}

impl Board {
  /// Creates a new chess board with the given dimensions.
  /// Board will always be rectangular (width * height).
  pub fn new(width: u8, height: u8) -> Board {
    let mut squares: Vec<Square> = vec![];
    let mut color = SquareColor::Dark;

    for _ in 0..width {
      for _ in 0..height {
        squares.push(Square::new(color));
        color = if color == SquareColor::Dark { SquareColor::Light } else { SquareColor::Dark }
      }

      // Chess boards flip colors every row, so repeat the last color we used by flipping again
      color = if color == SquareColor::Dark { SquareColor::Light } else { SquareColor::Dark }
    }

    Board {
      squares,
      width,
      height
    }
  }

  /// Returns a [`Square`](`crate::board::Square`) given the coordinates.
  /// The coordinates assume a chess player's perspective, and is not zero-indexed.
  /// For example, the coordinates (1, 8) would map to A8.
  /// This function handles the conversion from one-indexing to zero-indexing.
  fn get_square_by_coords(&self, x: u8, y: u8) -> &Square {
    &self.squares[((self.width * (y - 1)) + (x - 1)) as usize]
  }

  /// Returns a [`Square`](`crate::board::Square`) given a rank and file.
  pub fn get_square(&self, rank: Rank, file: File) -> &Square {
    self.get_square_by_coords(rank.into(), file.into())
  }

  /// Attempts to parse a board position `&str` into a
  /// ([`Rank`](`crate::board::Rank`), [`File`](`crate::board::File`)) tuple.
  /// A [`Error`](`crate::board::Error`) will be returned if the position fails
  /// to parse.
  pub fn get_rank_and_file(&self, position_str: &str) -> Result<(Rank, File), Error> {
    if position_str.len() != 2 {
      return Err(Error::InvalidPositionString);
    }

    let file = match position_str.chars().nth(0) {
      Some('a') | Some('A') => Ok(File::A),
      Some('b') | Some('B') => Ok(File::B),
      Some('c') | Some('C') => Ok(File::C),
      Some('d') | Some('D') => Ok(File::D),
      Some('e') | Some('E') => Ok(File::E),
      Some('f') | Some('F') => Ok(File::F),
      Some('g') | Some('G') => Ok(File::G),
      Some('h') | Some('H') => Ok(File::H),
      _ => Err(Error::InvalidPositionString)
    }?;

    let rank = match position_str.chars().nth(1) {
      Some('1') => Ok(Rank::One),
      Some('2') => Ok(Rank::Two),
      Some('3') => Ok(Rank::Three),
      Some('4') => Ok(Rank::Four),
      Some('5') => Ok(Rank::Five),
      Some('6') => Ok(Rank::Six),
      Some('7') => Ok(Rank::Seven),
      Some('8') => Ok(Rank::Eight),
      _ => Err(Error::InvalidPositionString)
    }?;

    Ok((rank, file))
  }
}

#[cfg(test)]
mod tests {
  use super::{*};

  fn make_standard_board() -> Board {
    Board::new(8, 8)
  }

  #[test]
  fn test_dark_square_returns_dark() {
    let dark_square = Square::new(SquareColor::Dark);
    assert_eq!(dark_square.get_color().clone(), SquareColor::Dark);
  }

  #[test]
  fn test_light_square_returns_light() {
    let light_square = Square::new(SquareColor::Light);
    assert_eq!(light_square.get_color().clone(), SquareColor::Light);
  }

  #[test]
  fn test_square_a1_is_dark() {
    let board = make_standard_board();
    let square_a1 = board.get_square(Rank::One, File::A);
    assert_eq!(square_a1.get_color().clone(), SquareColor::Dark);
  }

  #[test]
  fn test_square_c2_is_light() {
    let board = make_standard_board();
    let square_c2 = board.get_square(Rank::Two, File::C);
    assert_eq!(square_c2.get_color().clone(), SquareColor::Light);
  }

  #[test]
  fn test_get_rank_and_file_a1_success() {
    let board = make_standard_board();

    let (rank, file) = board.get_rank_and_file("A1").unwrap();
    assert_eq!(file, File::A);
    assert_eq!(rank, Rank::One);
  }

  #[test]
  fn test_get_rank_and_file_h8_success() {
    let board = make_standard_board();

    let (rank, file) = board.get_rank_and_file("H8").unwrap();
    assert_eq!(file, File::H);
    assert_eq!(rank, Rank::Eight);
  }

  #[test]
  fn test_get_rank_and_file_z5_invalid() {
    let board = make_standard_board();

    assert_eq!(board.get_rank_and_file("Z5").err().unwrap(), Error::InvalidPositionString);
  }

  #[test]
  fn test_get_rank_and_file_empty_string_invalid() {
    let board = make_standard_board();

    assert_eq!(board.get_rank_and_file("").err().unwrap(), Error::InvalidPositionString);
  }

  #[test]
  fn test_get_rank_and_file_a9_invalid() {
    let board = make_standard_board();

    assert_eq!(board.get_rank_and_file("a9").err().unwrap(), Error::InvalidPositionString);
  }

  #[test]
  fn test_get_rank_and_file_1010_invalid() {
    let board = make_standard_board();

    assert_eq!(board.get_rank_and_file("1010").err().unwrap(), Error::InvalidPositionString);
  }
}