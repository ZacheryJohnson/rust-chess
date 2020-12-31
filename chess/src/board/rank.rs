use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum Rank {
  One = 1,
  Two = 2,
  Three = 3,
  Four = 4,
  Five = 5,
  Six = 6,
  Seven = 7,
  Eight = 8,
  Invalid = std::isize::MAX,
}

impl fmt::Display for Rank {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.clone() {
      Rank::One => write!(f, "1"),
      Rank::Two => write!(f, "2"),
      Rank::Three => write!(f, "3"),
      Rank::Four => write!(f, "4"),
      Rank::Five => write!(f, "5"),
      Rank::Six => write!(f, "6"),
      Rank::Seven => write!(f, "7"),
      Rank::Eight => write!(f, "8"),
      Rank::Invalid => write!(f, "!"),
    }
  }
}

impl Into<i8> for Rank {
  fn into(self) -> i8 {
    match self {
      Rank::One => 1,
      Rank::Two => 2,
      Rank::Three => 3,
      Rank::Four => 4,
      Rank::Five => 5,
      Rank::Six => 6,
      Rank::Seven => 7,
      Rank::Eight => 8,
      Rank::Invalid => panic!("Trying to put invalid rank into i8"),
    }
  }
}

impl From<i8> for Rank {
  fn from(x: i8) -> Self {
    match x {
      1 => Rank::One,
      2 => Rank::Two,
      3 => Rank::Three,
      4 => Rank::Four,
      5 => Rank::Five,
      6 => Rank::Six,
      7 => Rank::Seven,
      8 => Rank::Eight,
      _ => Rank::Invalid,
    }
  }
}

impl Into<&str> for Rank {
  fn into(self) -> &'static str {
    match self {
      Rank::One => "1",
      Rank::Two => "2",
      Rank::Three => "3",
      Rank::Four => "4",
      Rank::Five => "5",
      Rank::Six => "6",
      Rank::Seven => "7",
      Rank::Eight => "8",
      _ => "INVALID"
    }
  }
}
