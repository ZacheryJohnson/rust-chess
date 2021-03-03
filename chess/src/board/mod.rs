use std::collections::HashSet;

pub mod square;
use crate::board::square::{Square, SquareColor};

pub mod file;
use crate::board::file::File;

pub mod rank;
use crate::board::rank::Rank;

pub mod coord;
use crate::board::coord::Coordinate;

use crate::errors::Error;

use crate::piece::{Color, Piece};
use crate::piece::bishop::Bishop;
use crate::piece::king::King;
use crate::piece::knight::Knight;
use crate::piece::pawn::Pawn;
use crate::piece::queen::Queen;
use crate::piece::rook::Rook;

const BOARD_WIDTH: i8 = 8;
const BOARD_HEIGHT: i8 = 8;

fn make_piece_at_coord(coord: Coordinate) -> Option<Box<dyn Piece>> {
  match (coord.file, coord.rank) {
    (File::A, Rank::One) => Some(Box::new(Rook::new(Color::White, coord))),
    (File::B, Rank::One) => Some(Box::new(Knight::new(Color::White, coord))),
    (File::C, Rank::One) => Some(Box::new(Bishop::new(Color::White, coord))),
    (File::D, Rank::One) => Some(Box::new(Queen::new(Color::White, coord))),
    (File::E, Rank::One) => Some(Box::new(King::new(Color::White, coord))),
    (File::F, Rank::One) => Some(Box::new(Bishop::new(Color::White, coord))),
    (File::G, Rank::One) => Some(Box::new(Knight::new(Color::White, coord))),
    (File::H, Rank::One) => Some(Box::new(Rook::new(Color::White, coord))),
    (File::A, Rank::Two) => Some(Box::new(Pawn::new(Color::White, coord))),
    (File::B, Rank::Two) => Some(Box::new(Pawn::new(Color::White, coord))),
    (File::C, Rank::Two) => Some(Box::new(Pawn::new(Color::White, coord))),
    (File::D, Rank::Two) => Some(Box::new(Pawn::new(Color::White, coord))),
    (File::E, Rank::Two) => Some(Box::new(Pawn::new(Color::White, coord))),
    (File::F, Rank::Two) => Some(Box::new(Pawn::new(Color::White, coord))),
    (File::G, Rank::Two) => Some(Box::new(Pawn::new(Color::White, coord))),
    (File::H, Rank::Two) => Some(Box::new(Pawn::new(Color::White, coord))),

    (File::A, Rank::Eight) => Some(Box::new(Rook::new(Color::Black, coord))),
    (File::B, Rank::Eight) => Some(Box::new(Knight::new(Color::Black, coord))),
    (File::C, Rank::Eight) => Some(Box::new(Bishop::new(Color::Black, coord))),
    (File::D, Rank::Eight) => Some(Box::new(Queen::new(Color::Black, coord))),
    (File::E, Rank::Eight) => Some(Box::new(King::new(Color::Black, coord))),
    (File::F, Rank::Eight) => Some(Box::new(Bishop::new(Color::Black, coord))),
    (File::G, Rank::Eight) => Some(Box::new(Knight::new(Color::Black, coord))),
    (File::H, Rank::Eight) => Some(Box::new(Rook::new(Color::Black, coord))),
    (File::A, Rank::Seven) => Some(Box::new(Pawn::new(Color::Black, coord))),
    (File::B, Rank::Seven) => Some(Box::new(Pawn::new(Color::Black, coord))),
    (File::C, Rank::Seven) => Some(Box::new(Pawn::new(Color::Black, coord))),
    (File::D, Rank::Seven) => Some(Box::new(Pawn::new(Color::Black, coord))),
    (File::E, Rank::Seven) => Some(Box::new(Pawn::new(Color::Black, coord))),
    (File::F, Rank::Seven) => Some(Box::new(Pawn::new(Color::Black, coord))),
    (File::G, Rank::Seven) => Some(Box::new(Pawn::new(Color::Black, coord))),
    (File::H, Rank::Seven) => Some(Box::new(Pawn::new(Color::Black, coord))),
    _ => None
  }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CastleAvailability {
  WhiteKingside,
  WhiteQueenside,
  BlackKingside,
  BlackQueenside,
}

fn get_castle_availability_str(avail: &HashSet<CastleAvailability>) -> String {
  let mut _str = String::new();
  if avail.contains(&CastleAvailability::WhiteKingside) { _str += "K"; }
  if avail.contains(&CastleAvailability::WhiteQueenside) { _str += "Q"; }
  if avail.contains(&CastleAvailability::BlackKingside) { _str += "k"; }
  if avail.contains(&CastleAvailability::BlackQueenside) { _str += "q"; }

  if _str.is_empty() { _str = String::from("-"); }

  _str
}

fn get_default_castling_availability() -> HashSet<CastleAvailability> {
  let mut castling_availability = HashSet::new();
  castling_availability.insert(CastleAvailability::WhiteKingside);
  castling_availability.insert(CastleAvailability::WhiteQueenside);
  castling_availability.insert(CastleAvailability::BlackKingside);
  castling_availability.insert(CastleAvailability::BlackQueenside);

  castling_availability
}

/// Collection of [`Square`](`crate::board::Square`)s, 8x8.
pub struct Board {
  squares: Vec<Square>,
  active_color: Color,
  castling_availability: HashSet<CastleAvailability>,
  en_passant_target: Option<Coordinate>,
  half_move_clock: i32,
  full_move: i32,
}

impl Board {
  /// Creates a new chess board with the given dimensions.
  /// Board will always be rectangular (width * height).
  pub fn new() -> Board {
    let mut squares: Vec<Square> = vec![];
    let mut color = SquareColor::Dark;

    for y in 0..BOARD_HEIGHT {
      for x in 0..BOARD_WIDTH {
        let coord = Coordinate::make_coordinate(x, y);
        let mut square = Square::new(color, coord);
        square.set_piece(make_piece_at_coord(coord));
        squares.push(square);
        color = if color == SquareColor::Dark { SquareColor::Light } else { SquareColor::Dark };
      }

      // Chess boards flip colors every row, so repeat the last color we used by flipping again
      color = if color == SquareColor::Dark { SquareColor::Light } else { SquareColor::Dark };
    }

    Board {
      squares,
      active_color: Color::White,
      castling_availability: get_default_castling_availability(),
      en_passant_target: None,
      half_move_clock: 0,
      full_move: 1,
    }
  }

  /// Creates a board from a given FEN string.
  pub fn from_fen_string(fen_string: &'static str) -> Result<Board, Error> {
    let fields: Vec<&str> = fen_string.split(" ").collect();

    if fields.len() != 6 {
      return Err(Error::InvalidFENString);
    }

    let pieces_str = fields[0];
    // In FEN, black rows are listed first and white rows last, so we'll reverse it first
    let mut ranks: Vec<&str> = pieces_str.split("/").collect();
    ranks.reverse();

    let mut squares: Vec<Square> = vec![];
    let mut square_idx = 0;
    let mut color = SquareColor::Dark;
    for rank in ranks {
      let mut expected_pieces_remaining: i8 = 8;
      for piece_char in rank.chars() {
        if expected_pieces_remaining <= 0 {
          println!("Found more pieces in a rank that was expecting!");
          return Err(Error::InvalidFENString);
        }

        let mut additional_empty_squares: i8 = 0;

        let coord = Coordinate::make_coordinate(square_idx % BOARD_WIDTH, square_idx / BOARD_WIDTH);
        let mut square = Square::new(color, coord);
        let piece: Result<Option<Box<dyn Piece>>, Error> = match piece_char {
          'p' => Ok(Some(Box::new(Pawn::new(Color::Black, coord)))),
          'r' => Ok(Some(Box::new(Rook::new(Color::Black, coord)))),
          'n' => Ok(Some(Box::new(Knight::new(Color::Black, coord)))),
          'b' => Ok(Some(Box::new(Bishop::new(Color::Black, coord)))),
          'q' => Ok(Some(Box::new(Queen::new(Color::Black, coord)))),
          'k' => Ok(Some(Box::new(King::new(Color::Black, coord)))),
          'P' => Ok(Some(Box::new(Pawn::new(Color::White, coord)))),
          'R' => Ok(Some(Box::new(Rook::new(Color::White, coord)))),
          'N' => Ok(Some(Box::new(Knight::new(Color::White, coord)))),
          'B' => Ok(Some(Box::new(Bishop::new(Color::White, coord)))),
          'Q' => Ok(Some(Box::new(Queen::new(Color::White, coord)))),
          'K' => Ok(Some(Box::new(King::new(Color::White, coord)))),
          '1' => { Ok(None) }
          '2' => {
            additional_empty_squares = 1;
            Ok(None)
          }
          '3' => {
            additional_empty_squares = 2;
            Ok(None)
          }
          '4' => {
            additional_empty_squares = 3;
            Ok(None)
          }
          '5' => {
            additional_empty_squares = 4;
            Ok(None)
          }
          '6' => {
            additional_empty_squares = 5;
            Ok(None)
          }
          '7' => {
            additional_empty_squares = 6;
            Ok(None)
          }
          '8' => {
            additional_empty_squares = 7;
            Ok(None)
          }
          _ => Err(Error::InvalidFENString),
        };
        square.set_piece(piece?);
        squares.push(square);
        color = if color == SquareColor::Dark { SquareColor::Light } else { SquareColor::Dark };
        expected_pieces_remaining -= 1;
        square_idx += 1;

        for _ in 0..additional_empty_squares {
          if expected_pieces_remaining <= 0 {
            println!("Found more pieces in a rank that was expecting!");
            return Err(Error::InvalidFENString);
          }
          let coord = Coordinate::make_coordinate(square_idx % BOARD_WIDTH, square_idx / BOARD_WIDTH);
          let square = Square::new(color, coord);
          squares.push(square);
          color = if color == SquareColor::Dark { SquareColor::Light } else { SquareColor::Dark };
          expected_pieces_remaining -= 1;
          square_idx += 1;
        }
      }

      // Chess boards flip colors every row, so repeat the last color we used by flipping again
      color = if color == SquareColor::Dark { SquareColor::Light } else { SquareColor::Dark };
    }

    let active_color_str = fields[1];
    let active_color: Color = {
      if active_color_str == "w" || active_color_str == "W" {
        Ok(Color::White)
      } else if active_color_str == "b" || active_color_str == "B" {
        Ok(Color::Black)
      } else {
        Err(Error::InvalidFENString)
      }
    }?;

    let castling_avail_str = fields[2];
    let mut castling_availability = HashSet::new();
    for castle_char in castling_avail_str.chars() {
      let _ = match castle_char {
        'K' => {
          castling_availability.insert(CastleAvailability::WhiteKingside);
          Ok(())
        }
        'Q' => {
          castling_availability.insert(CastleAvailability::WhiteQueenside);
          Ok(())
        }
        'k' => {
          castling_availability.insert(CastleAvailability::BlackKingside);
          Ok(())
        }
        'q' => {
          castling_availability.insert(CastleAvailability::BlackQueenside);
          Ok(())
        }
        '-' => { Ok(()) }
        _ => Err(Error::InvalidFENString),
      }?;
    }

    let en_passant_str = fields[3];
    let en_passant_target: Option<Coordinate> = {
      if en_passant_str == "-" {
        Ok(None)
      } else {
        Ok(Some(Coordinate::get_coordinate(en_passant_str)?))
      }
    }?;

    let half_move_str = fields[4];
    let half_move_clock = match half_move_str.parse() {
      Ok(val) => Ok(val),
      Err(_) => Err(Error::InvalidFENString),
    }?;

    let full_move_str = fields[5];
    let full_move = match full_move_str.parse() {
      Ok(val) => Ok(val),
      Err(_) => Err(Error::InvalidFENString),
    }?;


    let board = Board {
      squares,
      active_color,
      castling_availability,
      en_passant_target,
      half_move_clock,
      full_move,
    };

    Ok(board)
  }

  pub fn to_fen_string(&self) -> String {
    // Pieces
    let mut pieces_str = String::new();
    for y in (0..BOARD_HEIGHT).rev() {
      let mut consecutive_empty_squares: u8 = 0;
      for x in 0..BOARD_WIDTH {
        let square = self.get_square_by_coords(x, y).unwrap();
        match &square.get_piece() {
          Some(piece) => {
            if consecutive_empty_squares > 0 {
              pieces_str += consecutive_empty_squares.to_string().as_str();
            }
            consecutive_empty_squares = 0;

            pieces_str += &match piece.get_color() {
              Color::White => piece.get_short_name().to_uppercase(),
              Color::Black => piece.get_short_name().to_lowercase(),
            };
          }
          None => {
            consecutive_empty_squares += 1;
          }
        };
      }
      if consecutive_empty_squares > 0 {
        pieces_str += consecutive_empty_squares.to_string().as_str();
      }

      pieces_str.push('/');
    }

    // Remove extra / at end
    pieces_str.pop();

    let active_color_str = if self.get_active_color() == Color::White { "w" } else { "b" };

    let castle_avail_str = get_castle_availability_str(&self.get_castling_availability());

    let en_passant_str = match self.get_en_passant_target() {
      Some(coord) => Into::<String>::into(coord),
      None => String::from("-"),
    }.to_lowercase();

    let half_move_str = self.get_half_move_clock().to_string();

    let full_move_str = self.get_full_move().to_string();

    format!("{} {} {} {} {} {}",
      pieces_str,
      active_color_str,
      castle_avail_str,
      en_passant_str,
      half_move_str,
      full_move_str)
  }

  pub fn get_active_color(&self) -> Color {
    self.active_color.clone()
  }

  pub fn get_castling_availability(&self) -> HashSet<CastleAvailability> {
    self.castling_availability.clone()
  }

  pub fn get_en_passant_target(&self) -> Option<Coordinate> {
    self.en_passant_target.clone()
  }

  pub fn get_half_move_clock(&self) -> i32 {
    self.half_move_clock
  }

  pub fn get_full_move(&self) -> i32 {
    self.full_move
  }

  /// Returns true if a move or capture a piece at the target coordinate given it's color
  pub fn can_capture(&self, target_coord: &Coordinate, mover_color: &Color) -> bool {
    match self.get_square(*target_coord) {
      Ok(square) if square.get_piece().is_some() && *square.get_piece().as_ref().unwrap().get_color() != *mover_color => true,
      _ => false,
    }
  }

  /// Returns true if a piece can move to a target coordinate given it's color
  pub fn can_move(&self, target_coord: &Coordinate) -> bool {
    match self.get_square(*target_coord) {
      Ok(square) if square.get_piece().is_none() => true,
      _ => false,
    }
  }

  pub fn is_in_check(&self, king_color: &Color) -> bool {
    let king_pos: &Square = self.squares.iter()
      .filter(|sq| sq.get_piece().is_some())
      .filter(|sq| *sq.get_piece().as_ref().unwrap().get_short_name().to_lowercase() == *"k")
      .filter(|sq| *sq.get_piece().as_ref().unwrap().get_color() == *king_color)
      .collect::<Vec<&Square>>()[0];

    let attacker_color = if *king_color == Color::White { Color::Black } else { Color::White };
    let attacker_squares: Vec<&Square> = self.squares.iter()
      .filter(|sq| sq.get_piece().is_some())
      .filter(|sq| *sq.get_piece().as_ref().unwrap().get_color() == attacker_color)
      .collect();

    attacker_squares.iter()
      .flat_map(|sq| sq.get_piece().as_ref().unwrap().get_moves(&self))
      .any(|move_coord| move_coord == *king_pos.get_coord())
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
    self.get_square_by_coords(Into::<i8>::into(coord.file) - 1, Into::<i8>::into(coord.rank) - 1)
  }
}

#[cfg(test)]
mod tests {
  use super::{*};

  #[test]
  fn test_square_a1_is_dark() {
    let board = Board::new();
    let square_a1 = board.get_square(Coordinate { file: File::A, rank: Rank::One }).unwrap();
    assert_eq!(*square_a1.get_color(), SquareColor::Dark);
  }

  #[test]
  fn test_square_c2_is_light() {
    let board = Board::new();
    let square_c2 = board.get_square(Coordinate { file: File::C, rank: Rank::Two }).unwrap();
    assert_eq!(*square_c2.get_color(), SquareColor::Light);
  }

  #[test]
  fn test_fen_string_starting_position_success() {
    let board = Board::from_fen_string("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap();
    assert_eq!(board.get_active_color(), Color::White);
    assert_eq!(board.get_castling_availability(), get_default_castling_availability());
    assert!(board.get_en_passant_target().is_none());
    assert_eq!(board.get_half_move_clock(), 0);
    assert_eq!(board.get_full_move(), 1);
  }

  #[test]
  fn test_to_fen_string_starting_position_success() {
    let board = Board::new();
    assert_eq!(board.to_fen_string(), String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"));
  }

  #[test]
  fn test_fen_string_starting_position_then_e4_success() {
    let board = Board::from_fen_string("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap();
    assert_eq!(board.get_active_color(), Color::Black);
    assert_eq!(board.get_castling_availability(), get_default_castling_availability());
    assert_eq!(board.get_en_passant_target().unwrap(), Coordinate { file: File::E, rank: Rank::Three });
    assert_eq!(board.get_half_move_clock(), 0);
    assert_eq!(board.get_full_move(), 1);
  }

  #[test]
  fn test_to_fen_string_starting_position_then_e4_success() {
    let board = Board::from_fen_string("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap();
    assert_eq!(board.to_fen_string(), String::from("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"));
  }

  #[test]
  fn test_fen_string_lategame_success() {
    let board = Board::from_fen_string("8/1k4R1/1r6/6p1/8/3r2p1/3P2P1/2B3K1 b - - 1 37").unwrap();
    assert_eq!(board.get_active_color(), Color::Black);
    assert_eq!(board.get_castling_availability(), HashSet::new());
    assert!(board.get_en_passant_target().is_none());
    assert_eq!(board.get_half_move_clock(), 1);
    assert_eq!(board.get_full_move(), 37);
  }

  #[test]
  fn test_get_piece_at_c1_white_bishop() {
    let board = Board::new();
    let square = board.get_square(Coordinate { file: File::C, rank: Rank::One }).unwrap();

    let piece = square.get_piece().as_ref().unwrap();
    assert_eq!(*piece.get_color(), Color::White);
    assert_eq!(piece.get_short_name(), "B");
  }

  #[test]
  fn test_get_piece_at_g7_black_pawn() {
    let board = Board::new();
    let square = board.get_square(Coordinate { file: File::G, rank: Rank::Seven }).unwrap();

    let piece = square.get_piece().as_ref().unwrap();
    assert_eq!(*piece.get_color(), Color::Black);
    assert_eq!(piece.get_short_name(), "P");
  }

  #[test]
  fn test_neither_in_check_at_game_start() {
    let board = Board::new();

    assert!(!board.is_in_check(&Color::White));
    assert!(!board.is_in_check(&Color::Black));
  }

  #[test]
  fn test_white_is_in_check() {
    let board = Board::from_fen_string("rnbqk1nr/pppp1ppp/8/4P3/1b6/8/PPP1PPPP/RNBQKBNR w KQkq - 1 3").unwrap();

    assert!(board.is_in_check(&Color::White));
    assert!(!board.is_in_check(&Color::Black));
  }
}