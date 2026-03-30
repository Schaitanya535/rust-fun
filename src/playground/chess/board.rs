use super::pieces::{Piece, PieceColor, PromotionKind};

enum SquareColor {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rank {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Square {
    file: File,
    rank: Rank,
}

impl Square {
    fn color(&self) -> SquareColor {
        let f = self.file as u8;
        let r = self.rank as u8;
        if (f + r) % 2 == 0 {
            SquareColor::Dark
        } else {
            SquareColor::Light
        }
    }

    fn notation(&self) -> String {
        let file_char = (b'a' + self.file as u8) as char;
        format!("{}{}", file_char, self.rank as u8 + 1)
    }
}

struct Board {
    squares: [[Option<Piece>; 8]; 8],
}

pub enum Move {
    Normal {
        from: (File, Rank),
        to: (File, Rank),
    },
    Promotion {
        from: (File, Rank),
        to: (File, Rank),
        kind: PromotionKind, // ← can never be King or Pawn
    },
    CastleKingside {
        color: PieceColor,
    },
    CastleQueenside {
        color: PieceColor,
    },
    EnPassant {
        from: (File, Rank),
        to: (File, Rank),
    },
}

impl Board {
    // Basic access
    fn get(&self, file: File, rank: Rank) -> Option<Piece> {
        todo!()
    }
    fn set(&mut self, file: File, rank: Rank, piece: Option<Piece>) {
        todo!()
    }
    fn is_empty(&self, file: File, rank: Rank) -> bool {
        todo!()
    }

    // Square info — derived on the fly from coordinates
    fn square_color(file: File, rank: Rank) -> SquareColor {
        todo!()
    }

    // Piece queries
    fn find_king(&self, color: PieceColor) -> (File, Rank) {
        todo!()
    }
    fn pieces_of(&self, color: PieceColor) -> Vec<(File, Rank, Piece)> {
        todo!()
    }

    // Move logic
    fn apply_move(&mut self, mv: Move) {
        todo!()
    }
    fn legal_moves(&self, color: PieceColor) -> Vec<Move> {
        todo!()
    }
    fn is_in_check(&self, color: PieceColor) -> bool {
        todo!()
    }
    fn is_checkmate(&self, color: PieceColor) -> bool {
        todo!()
    }

    // Setup
    fn starting_position() -> Self {
        todo!()
    }
    fn empty() -> Self {
        todo!()
    }
}
