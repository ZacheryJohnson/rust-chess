use std::fmt;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, PartialOrd)]
pub enum File {
  A = 1,
  B = 2,
  C = 3,
  D = 4,
  E = 5,
  F = 6,
  G = 7,
  H = 8,
  Invalid = std::isize::MAX,
}

impl fmt::Display for File {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self.clone() {
      File::A => write!(f, "A"),
      File::B => write!(f, "B"),
      File::C => write!(f, "C"),
      File::D => write!(f, "D"),
      File::E => write!(f, "E"),
      File::F => write!(f, "F"),
      File::G => write!(f, "G"),
      File::H => write!(f, "H"),
      File::Invalid => write!(f, "!"),
    }
  }
}

impl Into<i8> for File {
  fn into(self) -> i8 {
    match self {
      File::A => 1,
      File::B => 2,
      File::C => 3,
      File::D => 4,
      File::E => 5,
      File::F => 6,
      File::G => 7,
      File::H => 8,
      File::Invalid => panic!("Trying to put invalid file into i8"),
    }
  }
}

impl From<i8> for File {
  fn from(x: i8) -> Self {
    match x {
      1 => File::A,
      2 => File::B,
      3 => File::C,
      4 => File::D,
      5 => File::E,
      6 => File::F,
      7 => File::G,
      8 => File::H,
      _ => File::Invalid,
    }
  }
}

impl Into<&str> for File {
  fn into(self) -> &'static str {
    match self {
      File::A => "A",
      File::B => "B",
      File::C => "C",
      File::D => "D",
      File::E => "E",
      File::F => "F",
      File::G => "G",
      File::H => "H",
      _ => "INVALID"
    }
  }
}