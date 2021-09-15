use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::matrix::Matrix;
use crate::piece::Piece;

#[inline]
pub fn unwrap_abort<T>(o: Option<T>) -> T {
  use std::process;
  match o {
    Some(t) => t,
    None => process::abort(),
  }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Bag {
  pieces: Vec<Piece>,
}

fn shuffled_pieces() -> Vec<Piece> {
  #[rustfmt::skip]
  let mut matrices = vec![
      // I
      vec![
          vec![0, 0, 0, 0],
          vec![1, 1, 1, 1],
          vec![0, 0, 0, 0],
          vec![0, 0, 0, 0]
      ],
      // J
      vec![
          vec![2, 0, 0],
          vec![2, 2, 2],
          vec![0, 0, 0]
      ],
      // L
      vec![
          vec![0, 0, 3],
          vec![3, 3, 3],
          vec![0, 0, 0]
      ],
      // O
      vec![
          vec![4, 4],
          vec![4, 4],
      ],
      // S
      vec![
          vec![0, 5, 5],
          vec![5, 5, 0],
          vec![0, 0, 0]
      ],
      // T
      vec![
          vec![0, 6, 0],
          vec![6, 6, 6],
          vec![0, 0, 0]
      ],
      // Z
      vec![
          vec![7, 7, 0],
          vec![0, 7, 7],
          vec![0, 0, 0]
      ],
  ];

  // Randomize matrices
  matrices.shuffle(&mut thread_rng());

  matrices
    .into_iter()
    .map(|matrix| Piece::new(Matrix::from_vec(matrix)))
    .collect()
}

impl Bag {
  pub fn new() -> Bag {
    Bag {
      pieces: shuffled_pieces(),
    }
  }
  pub fn take_piece(&mut self) -> Piece {
    let piece = unwrap_abort(self.pieces.pop());
    // Refill bag if empty
    if self.pieces.is_empty() {
      self.pieces = shuffled_pieces();
    }
    piece
  }
}
