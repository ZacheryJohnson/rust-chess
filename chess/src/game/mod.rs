
use crate::board::Board;
use crate::board::coord::Coordinate;
use crate::piece::Color;

struct Game {
  board: Board,
  current_player: Color
}

impl Game {
  pub fn new() -> Game {
    Game {
      board: Board::new(),
      current_player: Color::White,
    }
  }

  pub fn current_player(&self) -> Color {
    self.board.get_active_color()
  }
}

#[cfg(test)]
mod tests {
  use super::{*};
  #[test]
  fn test_create_new_game() {
    let game = Game::new();
    assert_eq!(game.current_player(), Color::White);
  }
}