use std::time::Instant;

use rayon::prelude::*;
use tinyvec::ArrayVec;
use yukari_movegen::{perft, Board, Move, Zobrist};

#[must_use]
pub fn divide(board: &Board, zobrist: &Zobrist, depth: u32) -> u64 {
    if depth == 0 {
        1
    } else {
        let moves: [Move; 256] = [Move::default(); 256];
        let mut moves = ArrayVec::from(moves);
        moves.set_len(0);
        board.generate(&mut moves);

        moves
            .par_iter()
            .map(|m| {
                let board = board.make(*m, zobrist);
                let nodes = perft(&board, zobrist, depth - 1);
                println!("{} {}", m, nodes);
                nodes
            })
            .sum()
    }
}

fn main() {
    let fen = std::env::args()
        .nth(1)
        .expect("Please provide a FEN string wrapped in quotes or the string 'bench' as argument");
    let depth = std::env::args()
        .nth(2)
        .expect("Please provide a FEN string wrapped in quotes or the string 'bench' as argument")
        .parse::<u32>()
        .expect("Please provide a FEN string wrapped in quotes or the string 'bench' as argument");
    let zobrist = Zobrist::new();
    let board = Board::from_fen(
        if fen == "startpos" {
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
        } else if fen == "kiwipete" {
            "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
        } else {
            &fen
        },
        &zobrist,
    )
    .unwrap();
    //let nodes = divide(&startpos, &zobrist, depth);
    let start = Instant::now();
    let nodes = perft(&board, &zobrist, depth);
    println!("Perft {}: {}", depth, nodes);
    println!("time: {:.3}s", Instant::now().duration_since(start).as_secs_f32());
}
