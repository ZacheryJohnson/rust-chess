use crate::piece::{*};

pub struct Rook { color: Color }
impl Rook {
  pub fn new(color: Color) -> Rook {
    Rook { color }
  }
}

impl Piece for Rook {
  fn get_color(&self) -> &Color { &self.color }

  fn get_short_name(&self) -> &'static str { "R" }

  fn get_moves(&self, board: &Board, own_coords: &Coordinate) -> Vec<Coordinate> {
    unimplemented!()
  }
}