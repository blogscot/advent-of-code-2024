use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Tile {
    Wall,
    Floor(Option<u32>),
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tile::Wall => write!(f, "# "),
            Tile::Floor(value) => {
              if let Some(steps) = value {
                write!(f, "{steps:<2} ")
              } else {
                write!(f, ". ")
              }
            },
        }
    }
}