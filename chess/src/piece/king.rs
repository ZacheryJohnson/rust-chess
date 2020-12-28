use crate::piece::{*};

pub struct King { color: Color }
impl King {
  pub fn new(color: Color) -> King {
    King { color }
  }
}

impl Piece for King {
  fn get_color(&self) -> &Color { &self.color }

  fn get_short_name(&self) -> &'static str { "K" }

  fn get_moves(&self, board: &Board, own_coords: &Coordinate) -> Vec<Coordinate> {
    unimplemented!()
  }
}