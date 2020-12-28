use crate::piece::{*};

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

  fn get_moves(&self) -> Vec<Coordinate> {
    unimplemented!()
  }
}
