use crate::piece::{*};

pub struct Bishop { color: Color }
impl Bishop {
  pub fn new(color: Color) -> Bishop {
    Bishop { color }
  }
}

impl Piece for Bishop {
  fn get_color(&self) -> &Color { &self.color }

  fn get_short_name(&self) -> &'static str { "B" }

  fn get_moves(&self, board: &Board, own_coords: &Coordinate) -> Vec<Coordinate> {
    unimplemented!()
  }
}