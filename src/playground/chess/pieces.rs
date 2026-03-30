pub struct Piece {
    pub kind: PieceKind,
    pub color: PieceColor,
}

pub enum PieceKind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

pub enum PieceColor {
    Light,
    Dark,
}

pub enum PromotionKind {
    Queen,
    Knight,
    Rook,
    Bishop,
}

impl From<PromotionKind> for PieceKind {
    fn from(p: PromotionKind) -> Self {
        match p {
            PromotionKind::Queen => PieceKind::Queen,
            PromotionKind::Rook => PieceKind::Rook,
            PromotionKind::Bishop => PieceKind::Bishop,
            PromotionKind::Knight => PieceKind::Knight,
        }
    }
}
