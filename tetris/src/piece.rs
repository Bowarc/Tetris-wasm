use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum PieceId {
    I,
    O,
    T,
    S,
    Z,
    L,
    J,
}

impl PieceId {
    pub fn color(&self) -> crate::Color {
        use crate::Color;
        match self {
            Self::I => Color::from((0, 255, 255)),
            Self::O => Color::from((255, 255, 0)),
            Self::T => Color::from((128, 0, 128)),
            Self::S => Color::from((0, 255, 0)),
            Self::Z => Color::from((255, 0, 0)),
            Self::L => Color::from((255, 165, 0)),
            Self::J => Color::from((0, 0, 255)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Bit(i8, i8);

impl Bit {
    pub fn x(&self) -> i8 {
        self.0
    }
    pub fn y(&self) -> i8 {
        self.1
    }
}

impl std::ops::Add<&crate::Position> for &Bit {
    type Output = Bit;

    fn add(self, position: &crate::Position) -> Self::Output {
        Bit(self.0 + position.x() as i8, self.1 + position.y() as i8)
    }
}

#[derive(Debug)]
pub struct Piece {
    id: PieceId,
    bits: Vec<Bit>,
    color: crate::color::Color,
}

impl Piece {
    pub fn id(&self) -> PieceId {
        self.id
    }
    pub fn bits(&self) -> &[Bit] {
        &self.bits
    }
    pub fn color(&self) -> &super::Color {
        &self.color
    }
    pub fn rotate(&mut self) {
        if self.id == PieceId::O {
            return;
        }

        for bit in self.bits.iter_mut() {
            let old_x = bit.0;
            let old_y = bit.1;
            bit.0 = old_y;
            bit.1 = -old_x;
        }
    }
}

// Using the 'super rotation system' from https://strategywiki.org/wiki/Tetris/Rotation_systems
impl From<PieceId> for Piece {
    fn from(id: PieceId) -> Self {
        match id {
            PieceId::I => Piece {
                id,
                bits: vec![Bit(-2, 0), Bit(-1, 0), Bit(0, 0), Bit(1, 0)],
                color: id.color(),
            },
            PieceId::O => Piece {
                id,
                bits: vec![Bit(0, 0), Bit(0, 1), Bit(1, 0), Bit(1, 1)],
                color: id.color(),
            },
            PieceId::J => Piece {
                id,
                bits: vec![Bit(-1, -1), Bit(-1, 0), Bit(0, 0), Bit(1, 0)],
                color: id.color(),
            },
            PieceId::L => Piece {
                id,
                bits: vec![Bit(-1, 0), Bit(0, 0), Bit(1, -1), Bit(1, 0)],
                color: id.color(),
            },
            PieceId::T => Piece {
                id,
                bits: vec![Bit(-1, 0), Bit(0, 0), Bit(1, 0), Bit(0, 1)],
                color: id.color(),
            },
            PieceId::S => Piece {
                id,
                bits: vec![Bit(-1, 1), Bit(0, 0), Bit(0, 1), Bit(1, 0)],
                color: id.color(),
            },
            PieceId::Z => Piece {
                id,
                bits: vec![Bit(-1, 0), Bit(0, 0), Bit(0, 1), Bit(1, 1)],
                color: id.color(),
            },
        }
    }
}

