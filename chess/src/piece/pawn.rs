use crate::piece::{*};

pub struct Pawn { color: Color }
impl Pawn {
  pub fn new(color: Color) -> Pawn {
    Pawn { color }
  }
}

impl Piece for Pawn {
  fn get_color(&self) -> &Color { &self.color }

  fn get_move_type(&self) -> MoveType { MoveType::Linear }

  fn get_short_name(&self) -> &'static str { "P" }

  fn get_moves(&self) -> Vec<Coordinate> {
    unimplemented!()
  }
}