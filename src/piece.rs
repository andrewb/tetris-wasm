use crate::matrix::Matrix;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Piece {
  pub matrix: Matrix,
  pub at_rest: bool,
}

impl Piece {
  pub fn new(matrix: Matrix) -> Piece {
    Piece {
      matrix,
      at_rest: false,
    }
  }
}
