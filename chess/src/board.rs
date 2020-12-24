use std::{vec};
use std::fmt;
use std::fmt::{Display, Formatter};
use crate::piece::{*};

const BOARD_WIDTH: i8 = 8;
const BOARD_HEIGHT: i8 = 8;

#[derive(Debug, PartialEq)]
pub enum Error {
  InvalidPositionString,
  InvalidRawCoordinatePair,
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
  }
}

/// Individual square on a [`Board`](`crate::board::Board`). Only has a color.
pub struct Square {
  color: SquareColor,
  piece: Option<Box<dyn Piece>>
}

impl Square {
  /// Creates a [`Square`](`crate::board::Square`)
  /// with a given [`SquareColor`](`crate::board::SquareColor`).
  pub fn new(color: SquareColor, piece: Option<Box<dyn Piece>>) -> Square {
    Square {
      color,
      piece
    }
  }

  /// Returns the [`Square`](`crate::board::Square`)'s [`SquareColor`](`crate::board::SquareColor`).
  pub fn get_color(&self) -> &SquareColor {
    &self.color
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum Rank {
  One = 1,
  Two = 2,
  Three = 3,
  Four = 4,
  Five = 5,
  Six = 6,
  Seven = 7,
  Eight = 8,
  Invalid = std::isize::MAX,
}

impl Display for Rank {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.clone() {
      Rank::One => write!(f, "1"),
      Rank::Two => write!(f, "2"),
      Rank::Three => write!(f, "3"),
      Rank::Four => write!(f, "4"),
      Rank::Five => write!(f, "5"),
      Rank::Six => write!(f, "6"),
      Rank::Seven => write!(f, "7"),
      Rank::Eight => write!(f, "8"),
      Rank::Invalid => write!(f, "!"),
    }
  }
}

impl Into<i8> for Rank {
  fn into(self) -> i8 {
    match self {
      Rank::One => 1,
      Rank::Two => 2,
      Rank::Three => 3,
      Rank::Four => 4,
      Rank::Five => 5,
      Rank::Six => 6,
      Rank::Seven => 7,
      Rank::Eight => 8,
      Rank::Invalid => panic!("Trying to put invalid rank into i8"),
    }
  }
}

impl From<i8> for Rank {
  fn from(x: i8) -> Self {
    match x {
      1 => Rank::One,
      2 => Rank::Two,
      3 => Rank::Three,
      4 => Rank::Four,
      5 => Rank::Five,
      6 => Rank::Six,
      7 => Rank::Seven,
      8 => Rank::Eight,
      _ => Rank::Invalid,
    }
  }
}

impl Into<&str> for Rank {
  fn into(self) -> &'static str {
    match self {
      Rank::One => "1",
      Rank::Two => "2",
      Rank::Three => "3",
      Rank::Four => "4",
      Rank::Five => "5",
      Rank::Six => "6",
      Rank::Seven => "7",
      Rank::Eight => "8",
      _ => "INVALID"
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum File {
  A = 1,
  B = 2,
  C = 3,
  D = 4,
  E = 5,
  F = 6,
  G = 7,
  H = 8,
  Invalid = std::isize::MAX,
}

impl Display for File {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.clone() {
      File::A => write!(f, "A"),
      File::B => write!(f, "B"),
      File::C => write!(f, "C"),
      File::D => write!(f, "D"),
      File::E => write!(f, "E"),
      File::F => write!(f, "F"),
      File::G => write!(f, "G"),
      File::H => write!(f, "H"),
      File::Invalid => write!(f, "!"),
    }
  }
}

impl Into<i8> for File {
  fn into(self) -> i8 {
    match self {
      File::A => 1,
      File::B => 2,
      File::C => 3,
      File::D => 4,
      File::E => 5,
      File::F => 6,
      File::G => 7,
      File::H => 8,
      File::Invalid => panic!("Trying to put invalid file into i8"),
    }
  }
}

impl From<i8> for File {
  fn from(x: i8) -> Self {
    match x {
      1 => File::A,
      2 => File::B,
      3 => File::C,
      4 => File::D,
      5 => File::E,
      6 => File::F,
      7 => File::G,
      8 => File::H,
      _ => File::Invalid,
    }
  }
}

impl Into<&str> for File {
  fn into(self) -> &'static str {
    match self {
      File::A => "A",
      File::B => "B",
      File::C => "C",
      File::D => "D",
      File::E => "E",
      File::F => "F",
      File::G => "G",
      File::H => "H",
      _ => "INVALID"
    }
  }
}

pub struct Coordinate {
  pub file: File,
  pub rank: Rank
}

impl Display for Coordinate {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}{}", Into::<&str>::into(self.file), Into::<&str>::into(self.rank))
  }
}

fn get_piece_at_start_coord(coord: Coordinate) -> Option<Box<dyn Piece>>{
  match (coord.file, coord.rank) {
    (File::A, Rank::One) => Some(Box::new(Rook::new(Color::White))),
    (File::B, Rank::One) => Some(Box::new(Knight::new(Color::White))),
    (File::C, Rank::One) => Some(Box::new(Bishop::new(Color::White))),
    (File::D, Rank::One) => Some(Box::new(Queen::new(Color::White))),
    (File::E, Rank::One) => Some(Box::new(King::new(Color::White))),
    (File::F, Rank::One) => Some(Box::new(Bishop::new(Color::White))),
    (File::G, Rank::One) => Some(Box::new(Knight::new(Color::White))),
    (File::H, Rank::One) => Some(Box::new(Rook::new(Color::White))),
    (File::A, Rank::Two) => Some(Box::new(Pawn::new(Color::White))),
    (File::B, Rank::Two) => Some(Box::new(Pawn::new(Color::White))),
    (File::C, Rank::Two) => Some(Box::new(Pawn::new(Color::White))),
    (File::D, Rank::Two) => Some(Box::new(Pawn::new(Color::White))),
    (File::E, Rank::Two) => Some(Box::new(Pawn::new(Color::White))),
    (File::F, Rank::Two) => Some(Box::new(Pawn::new(Color::White))),
    (File::G, Rank::Two) => Some(Box::new(Pawn::new(Color::White))),
    (File::H, Rank::Two) => Some(Box::new(Pawn::new(Color::White))),

    (File::A, Rank::Eight) => Some(Box::new(Rook::new(Color::Black))),
    (File::B, Rank::Eight) => Some(Box::new(Knight::new(Color::Black))),
    (File::C, Rank::Eight) => Some(Box::new(Bishop::new(Color::Black))),
    (File::D, Rank::Eight) => Some(Box::new(Queen::new(Color::Black))),
    (File::E, Rank::Eight) => Some(Box::new(King::new(Color::Black))),
    (File::F, Rank::Eight) => Some(Box::new(Bishop::new(Color::Black))),
    (File::G, Rank::Eight) => Some(Box::new(Knight::new(Color::Black))),
    (File::H, Rank::Eight) => Some(Box::new(Rook::new(Color::Black))),
    (File::A, Rank::Seven) => Some(Box::new(Pawn::new(Color::Black))),
    (File::B, Rank::Seven) => Some(Box::new(Pawn::new(Color::Black))),
    (File::C, Rank::Seven) => Some(Box::new(Pawn::new(Color::Black))),
    (File::D, Rank::Seven) => Some(Box::new(Pawn::new(Color::Black))),
    (File::E, Rank::Seven) => Some(Box::new(Pawn::new(Color::Black))),
    (File::F, Rank::Seven) => Some(Box::new(Pawn::new(Color::Black))),
    (File::G, Rank::Seven) => Some(Box::new(Pawn::new(Color::Black))),
    (File::H, Rank::Seven) => Some(Box::new(Pawn::new(Color::Black))),
    _ => None
  }
}

fn make_coord(x: i8, y: i8) -> Coordinate {
  Coordinate { file: File::from(x + 1), rank: Rank::from(y + 1) }
}


/// Collection of [`Square`](`crate::board::Square`)s, 8x8.
pub struct Board {
  squares: Vec<Square>
}

impl Display for Board {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    for x in (0..BOARD_WIDTH).rev() {
      for y in 0..BOARD_HEIGHT {
        let square = self.get_square_by_coords(x, y).unwrap();
        match &square.piece {
          Some(piece) => {write!(f, "|{}{}", piece.get_color(), piece.get_short_name());},
          None => {
            match square.get_color() {
              SquareColor::Dark => {write!(f, "|::");},
              SquareColor::Light => {write!(f, "|  ");},
            }
          },
        };
      }
      write!(f, "\n");
    }

    write!(f, "")
  }
}

impl Board {
  /// Creates a new chess board with the given dimensions.
  /// Board will always be rectangular (width * height).
  pub fn new() -> Board {
    let mut squares: Vec<Square> = vec![];
    let mut color = SquareColor::Dark;

    for x in 0..BOARD_WIDTH {
      for y in 0..BOARD_HEIGHT {
        let piece = get_piece_at_start_coord(make_coord(x, y));
        squares.push(Square::new(color, piece));
        color = if color == SquareColor::Dark { SquareColor::Light } else { SquareColor::Dark }
      }

      // Chess boards flip colors every row, so repeat the last color we used by flipping again
      color = if color == SquareColor::Dark { SquareColor::Light } else { SquareColor::Dark }
    }

    Board {
      squares
    }
  }

  /// Returns a [`Square`](`crate::board::Square`) given the coordinates.
  /// The coordinates assume a chess player's perspective, and is not zero-indexed.
  /// For example, the coordinates (1, 8) would map to A8.
  /// This function handles the conversion from one-indexing to zero-indexing.
  fn get_square_by_coords(&self, x: i8, y: i8) -> Result<&Square, Error> {
    if x < 0 || y < 0 {
      return Err(Error::InvalidRawCoordinatePair);
    }

    match self.squares.get((BOARD_WIDTH * y + x) as usize) {
      Some(square) => Ok(square),
      None => Err(Error::InvalidRawCoordinatePair)
    }
  }

  /// Returns a [`Square`](`crate::board::Square`) given a rank and file.
  pub fn get_square(&self, coord: Coordinate) -> Result<&Square, Error> {
    self.get_square_by_coords(Into::<i8>::into(coord.rank) - 1, Into::<i8>::into(coord.file) - 1)
  }

  /// Attempts to parse a board position `&str` into a
  /// ([`Rank`](`crate::board::Rank`), [`File`](`crate::board::File`)) tuple.
  /// A [`Error`](`crate::board::Error`) will be returned if the position fails
  /// to parse.
  pub fn get_coordinate(&self, position_str: &str) -> Result<Coordinate, Error> {
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

    Ok(Coordinate{file, rank})
  }
}

#[cfg(test)]
mod tests {
  use super::{*};

  fn make_standard_board() -> Board {
    Board::new()
  }

  #[test]
  fn test_draw_board() {
    let board = make_standard_board();
    println!("{}", board);
  }

  #[test]
  fn test_dark_square_returns_dark() {
    let dark_square = Square::new(SquareColor::Dark, None);
    assert_eq!(*dark_square.get_color(), SquareColor::Dark);
  }

  #[test]
  fn test_light_square_returns_light() {
    let light_square = Square::new(SquareColor::Light, None);
    assert_eq!(*light_square.get_color(), SquareColor::Light);
  }

  #[test]
  fn test_square_a1_is_dark() {
    let board = make_standard_board();
    let square_a1 = board.get_square(Coordinate { file: File::A, rank: Rank::One }).unwrap();
    assert_eq!(*square_a1.get_color(), SquareColor::Dark);
  }

  #[test]
  fn test_square_c2_is_light() {
    let board = make_standard_board();
    let square_c2 = board.get_square(Coordinate { file: File::C, rank: Rank::Two }).unwrap();
    assert_eq!(*square_c2.get_color(), SquareColor::Light);
  }

  #[test]
  fn test_get_coordinate_a1_success() {
    let board = make_standard_board();

    let coord = board.get_coordinate("A1").unwrap();
    assert_eq!(coord.file, File::A);
    assert_eq!(coord.rank, Rank::One);
  }

  #[test]
  fn test_get_coordinate_h8_success() {
    let board = make_standard_board();

    let coord = board.get_coordinate("H8").unwrap();
    assert_eq!(coord.file, File::H);
    assert_eq!(coord.rank, Rank::Eight);
  }

  #[test]
  fn test_get_coordinate_z5_invalid() {
    let board = make_standard_board();

    assert_eq!(board.get_coordinate("Z5").err().unwrap(), Error::InvalidPositionString);
  }

  #[test]
  fn test_get_coordinate_empty_string_invalid() {
    let board = make_standard_board();

    assert_eq!(board.get_coordinate("").err().unwrap(), Error::InvalidPositionString);
  }

  #[test]
  fn test_get_coordinate_a9_invalid() {
    let board = make_standard_board();

    assert_eq!(board.get_coordinate("a9").err().unwrap(), Error::InvalidPositionString);
  }

  #[test]
  fn test_get_coordinate_1010_invalid() {
    let board = make_standard_board();

    assert_eq!(board.get_coordinate("1010").err().unwrap(), Error::InvalidPositionString);
  }

  #[test]
  fn test_get_piece_at_c1_white_bishop() {
    let board = make_standard_board();
    let square = board.get_square(Coordinate { file: File::C, rank:Rank::One }).unwrap();

    let piece = square.piece.as_ref().unwrap();
    assert_eq!(*piece.get_color(), Color::White);
    assert_eq!(piece.get_short_name(), "B");
  }

  #[test]
  fn test_get_piece_at_g7_black_pawn() {
    let board = make_standard_board();
    let square = board.get_square(Coordinate { file: File::G, rank:Rank::Seven }).unwrap();

    let piece = square.piece.as_ref().unwrap();
    assert_eq!(*piece.get_color(), Color::Black);
    assert_eq!(piece.get_short_name(), "P");
  }
}