use crate::matrix::Matrix;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Board {
  pub matrix: Matrix,
}

impl Board {
  pub fn new(row_count: u8, col_count: u8) -> Board {
    Board {
      matrix: Matrix::from_vec(vec![vec![0; col_count as usize]; row_count as usize]),
    }
  }
}
