pub type BoardRow = [Option<crate::PieceId>; 10];

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Board([BoardRow; 20]);

impl Board {
    pub fn place_at(
        &mut self,
        piece: &crate::Piece,
        position: &crate::Position,
    ) -> Result<(), String> {
        if !self.can_place_at(piece, position) {
            return Err(format!("Cannot place {piece:?} at {position:?}"));
        }

        // from now on, we assume that the check above did its job at verifying the position is correct
        for bit in piece.bits() {
            let p = bit + position;

            *self
                .0
                .get_mut(p.y() as usize)
                .and_then(|row| row.get_mut(p.x() as usize))
                .unwrap() = Some(piece.id());
        }

        Ok(())
    }
    pub fn can_place_at(&self, piece: &crate::Piece, position: &crate::Position) -> bool {
        for bit in piece.bits().iter().map(|bit| bit + position) {
            // -1 'cause indexes
            if bit.x() < 0 || bit.y() < 0 || bit.x() > 10 - 1 || bit.y() > 20 - 1 {
                return false;
            }

            let Some(row) = self.0.get(bit.y() as usize) else {
                return false;
            };

            let Some(spot) = row.get(bit.x() as usize) else {
                return false;
            };

            if spot.is_some() {
                return false;
            }
        }
        true
    }
}

pub struct BoardIterator<'a> {
    board: &'a Board,
    row: usize,
    col: usize,
}

impl<'a> Iterator for BoardIterator<'a> {
    type Item = (Option<crate::PieceId>, crate::Position);

    //   Some(Some(piece_id)) if the cell contains a piece.
    //   Some(None) if the cell is empty.
    //   None when there are no more cells to iterate over.

    fn next(&mut self) -> Option<Self::Item> {
        if self.row >= 20 {
            return None;
        }

        let id = self
            .board
            .0
            .get(self.row)
            .and_then(|row| row.get(self.col))
            .cloned()
            .unwrap();
        let pos = crate::Position::from((self.col as u8, self.row as u8));

        self.col += 1;

        if self.col >= 10 {
            self.col = 0;
            self.row += 1;
        }

        Some((id, pos))
    }
}

impl Board {
    pub fn iter(&self) -> BoardIterator {
        BoardIterator {
            board: self,
            row: 0,
            col: 0,
        }
    }
}
