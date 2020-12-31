use crate::board::file::File;
use crate::board::rank::Rank;
use std::fmt;
use std::ops::Add;

use crate::errors::Error;

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
pub struct Coordinate {
  pub file: File,
  pub rank: Rank,
}

impl fmt::Display for Coordinate {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", Into::<&str>::into(self.file), Into::<&str>::into(self.rank))
  }
}

impl Into<String> for Coordinate {
  fn into(self) -> String {
    let file_str: &str = self.file.into();
    let rank_str: &str = self.rank.into();

    String::from(file_str) + rank_str
  }
}

impl Add<(i8, i8)> for Coordinate {
  type Output = Self;

  fn add(self, rhs: (i8, i8)) -> Self::Output {
    Coordinate {
      file: File::from(Into::<i8>::into(self.file) + rhs.0),
      rank: Rank::from(Into::<i8>::into(self.rank) + rhs.1),
    }
  }
}

impl Coordinate {
  pub fn is_valid(&self) -> bool {
    self.file != File::Invalid && self.rank != Rank::Invalid
  }

  pub fn make_coordinate(x: i8, y: i8) -> Coordinate {
    Coordinate { file: File::from(x + 1), rank: Rank::from(y + 1) }
  }

  /// Attempts to parse a board position `&str` into a
  /// ([`Rank`](`crate::board::Rank`), [`File`](`crate::board::File`)) tuple.
  /// A [`Error`](`crate::board::Error`) will be returned if the position fails
  /// to parse.
  pub fn get_coordinate(position_str: &str) -> Result<Coordinate, Error> {
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

    Ok(Coordinate { file, rank })
  }
}

#[cfg(test)]
mod tests {
  use super::{*};

  #[test]
  fn test_get_coordinate_a1_success() {
    let coord = Coordinate::get_coordinate("A1").unwrap();
    assert_eq!(coord.file, File::A);
    assert_eq!(coord.rank, Rank::One);
  }

  #[test]
  fn test_get_coordinate_h8_success() {
    let coord = Coordinate::get_coordinate("H8").unwrap();
    assert_eq!(coord.file, File::H);
    assert_eq!(coord.rank, Rank::Eight);
  }

  #[test]
  fn test_get_coordinate_z5_invalid() {
    assert_eq!(Coordinate::get_coordinate("Z5").err().unwrap(), Error::InvalidPositionString);
  }

  #[test]
  fn test_get_coordinate_empty_string_invalid() {
    assert_eq!(Coordinate::get_coordinate("").err().unwrap(), Error::InvalidPositionString);
  }

  #[test]
  fn test_get_coordinate_a9_invalid() {
    assert_eq!(Coordinate::get_coordinate("a9").err().unwrap(), Error::InvalidPositionString);
  }

  #[test]
  fn test_get_coordinate_1010_invalid() {
    assert_eq!(Coordinate::get_coordinate("1010").err().unwrap(), Error::InvalidPositionString);
  }
}