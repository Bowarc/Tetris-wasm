// Function to draw the piece in a box
#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use tetris::*;
    fn draw_piece(piece: &Piece) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "{:?}\n\n",
            piece.bits().iter().map(|bit| format!("({}, {}), ", bit.x(), bit.y())).collect::<String>()
        ));

        // Create a grid to represent the piece
        let mut grid = vec![vec![' '; 7]; 5]; // 5 rows and 7 columns

        for bit in piece.bits() {
            // Shift  to center in the grid
            let x = bit.x() + 3;
            let y = bit.y() + 2;
            if (0..5).contains(&y) && (0..7).contains(&x) {
                grid[y as usize][x as usize] = '#';
            }
        }

        out.push_str("+-------+\n");

        for row in grid {
            out.push_str(&format!("|{}|\n", row.iter().collect::<String>()));
        }

        out.push_str("+-------+\n");
        out
    }

    fn test_piece(mut piece: Piece, rotations: usize) {
        assert_debug_snapshot!(draw_piece(&piece));

        for _ in 0..rotations {
            piece.rotate();

            assert_debug_snapshot!(draw_piece(&piece));
        }
    }

    #[test]
    fn test() {
        // Theses take snapshots of results, I havn't taken the time to verify them all atm
        test_piece(Piece::from(PieceId::I), 4);
        test_piece(Piece::from(PieceId::O), 4);
        test_piece(Piece::from(PieceId::T), 4);
        test_piece(Piece::from(PieceId::S), 4);
        test_piece(Piece::from(PieceId::Z), 4);
        test_piece(Piece::from(PieceId::L), 4);
        test_piece(Piece::from(PieceId::J), 4);
    }
}
