use rand::{prelude::StdRng, Rng, SeedableRng};

use crate::{square::File, Colour, Piece, Square};

#[derive(Clone)]
pub struct Zobrist {
    piece: [[[u64; 64]; 6]; 2],
    side: u64,
    ep: [u64; 8],
    castling: [u64; 4],
}

impl Zobrist {
    #[must_use]
    pub fn new() -> Self {
        let mut rng = StdRng::seed_from_u64(1);

        let mut piece = [[[0_u64; 64]; 6]; 2];
        let mut ep = [0; 8];
        let mut castling = [0; 4];

        for side in &mut piece {
            for piece_kind in side.iter_mut() {
                for square in piece_kind.iter_mut() {
                    *square = rng.gen();
                }
            }
        }

        let side = rng.gen();

        for file in &mut ep {
            *file = rng.gen();
        }

        for castle_flag in &mut castling {
            *castle_flag = rng.gen();
        }

        Self { piece, side, ep, castling }
    }

    pub fn add_piece(&self, colour: Colour, piece: Piece, square: Square, hash: &mut u64) {
        *hash ^= self.piece[colour as usize][piece as usize][square.into_inner() as usize];
    }

    pub fn remove_piece(&self, colour: Colour, piece: Piece, square: Square, hash: &mut u64) {
        *hash ^= self.piece[colour as usize][piece as usize][square.into_inner() as usize];
    }

    pub fn move_piece(&self, colour: Colour, piece: Piece, from_square: Square, to_square: Square, hash: &mut u64) {
        *hash ^= self.piece[colour as usize][piece as usize][from_square.into_inner() as usize]
            ^ self.piece[colour as usize][piece as usize][to_square.into_inner() as usize];
    }

    pub fn set_ep(&self, old: Option<Square>, new: Option<Square>, hash: &mut u64) {
        if let Some(ep) = old {
            *hash ^= self.ep[File::from(ep) as usize];
        }
        if let Some(ep) = new {
            *hash ^= self.ep[File::from(ep) as usize];
        }
    }

    pub fn add_castling(&self, kind: usize, hash: &mut u64) {
        *hash ^= self.castling[kind];
    }

    pub fn remove_castling(&self, kind: usize, hash: &mut u64) {
        *hash ^= self.castling[kind];
    }

    pub fn toggle_side(&self, hash: &mut u64) {
        *hash ^= self.side;
    }
}

impl Default for Zobrist {
    fn default() -> Self {
        Self::new()
    }
}
