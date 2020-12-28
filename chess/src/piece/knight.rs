use crate::piece::{*};

pub struct Knight { color: Color }
impl Knight {
  pub fn new(color: Color) -> Knight {
    Knight { color }
  }
}

impl Piece for Knight {
  fn get_color(&self) -> &Color { &self.color }

  fn get_short_name(&self) -> &'static str { "N" }

  fn get_moves(&self, board: &Board, own_coords: &Coordinate) -> Vec<Coordinate> {
    unimplemented!()
  }
}