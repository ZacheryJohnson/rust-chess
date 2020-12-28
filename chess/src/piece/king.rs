use crate::piece::{*};

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

  fn get_moves(&self) -> Vec<Coordinate> {
    unimplemented!()
  }
}