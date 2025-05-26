#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position(u8, u8);

impl Position{
    pub fn x(&self) -> u8 {
        self.0
    }
    pub fn y(&self) -> u8 {
        self.1
    }
}

impl From<(u8, u8)> for Position{
    fn from(value: (u8, u8)) -> Self {
        Position(value.0, value.1)
    }
}
