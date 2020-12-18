type CoordinateDelta = (i8, i8);

enum Color {
  White,
  Black,
}

enum MoveType {
  /// Linear pieces must stop upon encountering any piece.
  /// All standard chess pieces are linear except the knight.
  Linear,

  /// Jump pieces can "jump" over other pieces and aren't stopped
  /// if any pieces would be in their path if travelling linearly.
  /// The knight is the only jump piece.
  Jump,
}

trait Piece {
  fn get_color(&self) -> Color;

  fn get_move_type(&self) -> MoveType;

  /// Returns the legal moves a piece can make in isolation.
  /// This does NOT consider any other pieces - imagine the piece alone on an empty board.
  /// This means that pawns can't capture (normally or en passant), pieces don't need to
  /// respect check/checkmate, etc.
  fn get_moves(&self) -> Vec<CoordinateDelta>;
}

struct Pawn { color: Color}
impl Piece for Pawn {
  fn get_color(&self) -> &Color { &self.color }

  fn get_move_type(&self) -> MoveType { MoveType::Linear }

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

struct Knight { color: Color }
impl Piece for Knight {
  fn get_color(&self) -> &Color { &self.color }

  fn get_move_type(&self) -> MoveType { MoveType::Jump }

  fn get_moves(&self) -> Vec<CoordinateDelta> {
    vec!(
      ( 1,  2), ( 2,  1),
      (-1,  2), (-2,  1),
      (-1, -2), (-2, -1),
      ( 1, -2), ( 2, -1),
    )
  }
}

struct Bishop { color: Color }
impl Piece for Bishop {
  fn get_color(&self) -> &Color { &self.color }

  fn get_move_type(&self) -> MoveType { MoveType::Linear }

  fn get_moves(&self) -> Vec<CoordinateDelta> {
    vec!(
      (1,1), (2,2), (3,3), (4,4), (5,5), (6,6), (7,7),
      (-1,1), (-2,2), (-3,3), (-4,4), (-5,5), (-6,6), (-7,7),
      (1,-1), (2,-2), (3,-3), (4,-4), (5,-5), (6,-6), (7,-7),
      (-1,-1), (-2,-2), (-3,-3), (-4,-4), (-5,-5), (-6,-6), (-7,-7),
    )
  }
}

struct Rook { color: Color }
impl Piece for Rook {
  fn get_color(&self) -> &Color { &self.color }

  fn get_move_type(&self) -> MoveType { MoveType::Linear }

  fn get_moves(&self) -> Vec<CoordinateDelta> {
    vec!(
      (0,1), (0,2), (0,3), (0,4), (0,5), (0,6), (0,7),
      (0,-1), (0,-2), (0,-3), (0,-4), (0,-5), (0,-6), (0,-7),
      (1,0), (2,0), (3,0), (4,0), (5,0), (6,0), (7,0),
      (-1,0), (-2,0), (-3,0), (-4,0), (-5,0), (-6,0), (-7,0),
    )
  }
}

struct Queen { color: Color }
impl Piece for Queen {
  fn get_color(&self) -> &Color { &self.color }

  fn get_move_type(&self) -> MoveType { MoveType::Linear }

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

struct King { color: Color }
impl Piece for King {
  fn get_color(&self) -> &Color { &self.color }

  fn get_move_type(&self) -> MoveType { MoveType::Linear }

  fn get_moves(&self) -> Vec<CoordinateDelta> {
    vec!(
      (-1,1), (0,1), (1,1),
      (-1,0),         (1,0),
      (-1,-1), (0,-1), (1,-1),
    )
  }
}