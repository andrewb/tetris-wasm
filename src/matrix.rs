use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Matrix {
  cells: Vec<u8>,
  pub row_count: u8,
  pub col_count: u8,
  pub origin: (i8, i8),
}

// Private
impl Matrix {
  fn get_index(&self, row: i32, column: i32) -> usize {
    (row * self.col_count as i32 + column) as usize
  }
  fn blocks(&self) -> HashSet<(i8, i8)> {
    let mut blocks = HashSet::new();
    let (x, y) = self.origin;
    let chunks = self.cells.as_slice().chunks(self.col_count as usize);
    for (i, row) in chunks.enumerate() {
      for (j, &cell) in row.iter().enumerate() {
        if cell != 0 {
          blocks.insert((
            // Row
            i as i8 + y,
            // Col
            j as i8 + x,
          ));
        }
      }
    }
    blocks
  }
  fn in_bounds(&self, row: i32, col: i32) -> bool {
    (row >= 0 && row < self.row_count as i32) && (col >= 0 && col < self.col_count as i32)
  }
  fn block_value(&self, block: (i8, i8)) -> u8 {
    let (x, y) = self.origin;
    self.cells[self.get_index((block.0 - y) as i32, (block.1 - x) as i32)]
  }
}

// Public
impl Matrix {
  // Convert 2d vector to matrix
  pub fn from_vec(vec: Vec<Vec<u8>>) -> Matrix {
    let row_count = vec.len() as u8;
    let col_count = vec.first().unwrap().len() as u8;
    let cells = vec.iter().flatten().cloned().collect();
    Matrix {
      row_count,
      col_count,
      origin: (0, 0),
      cells,
    }
  }
  pub fn transpose(matrix: &Matrix) -> Matrix {
    let mut transposed = Vec::new();
    let vec = matrix.to_vec();
    for (i, row) in vec.iter().enumerate() {
      for (j, _) in row.iter().enumerate() {
        if i == 0 {
          transposed.push(vec![0; vec.len()]);
        }
        transposed[j][i] = vec[i][j];
      }
    }
    Matrix::from_vec(transposed)
  }
  pub fn rotate_right(matrix: &Matrix) -> Matrix {
    let mut transposed = Matrix::transpose(matrix).to_vec();
    for row in transposed.iter_mut() {
      row.reverse();
    }
    let mut rotated = Matrix::from_vec(transposed);
    rotated.origin = matrix.origin;
    rotated
  }
  pub fn min_y(&self) -> Option<i8> {
    self.blocks().iter().map(|b| b.0).min()
  }
  pub fn max_y(&self) -> Option<i8> {
    self.blocks().iter().map(|b| b.0).max()
  }
  pub fn min_x(&self) -> Option<i8> {
    self.blocks().iter().map(|b| b.1).min()
  }
  pub fn max_x(&self) -> Option<i8> {
    self.blocks().iter().map(|b| b.1).max()
  }
  pub fn collides(&self, other: &Matrix) -> bool {
    self.blocks().intersection(&other.blocks()).count() > 0
  }
  pub fn resolution_x(&self, other: &Matrix) -> i8 {
    let intersection = self
      .blocks()
      .intersection(&other.blocks())
      .cloned()
      .collect::<HashSet<(i8, i8)>>();
    if intersection.is_empty() {
      return 0;
    }

    let min_self_x = self.min_x().unwrap();
    let max_self_x = self.max_x().unwrap();
    let min_other_x = intersection.iter().map(|b| b.1).min().unwrap();
    let mid_point = min_self_x + ((max_self_x - min_self_x) / 2);

    let mut next_matrix = self.clone();
    let mut collides = true;

    while collides {
      // Move out of intersection
      if min_other_x > mid_point {
        // Move left
        next_matrix.origin.0 -= 1;
      } else {
        // Move right
        next_matrix.origin.0 += 1;
      }
      collides = next_matrix.blocks().intersection(&intersection).count() > 0
    }

    next_matrix.origin.0 - self.origin.0
  }
  pub fn add(&mut self, other: &Matrix) {
    // Add to matrix
    for block in other.blocks() {
      let world_row = block.0 as i32 - self.origin.1 as i32;
      let world_col = block.1 as i32 - self.origin.0 as i32;

      if self.in_bounds(world_row, world_col) {
        let self_idx = self.get_index(world_row, world_col);
        self.cells[self_idx] = other.block_value(block);
      }
    }
  }
  pub fn to_vec(&self) -> Vec<Vec<u8>> {
    let mut vec = Vec::new();
    let chunks = self.cells.as_slice().chunks(self.col_count as usize);
    for row in chunks {
      vec.push(row.to_vec());
    }
    vec
  }
  pub fn cells_ptr(&self) -> *const u8 {
    self.cells.as_ptr()
  }
}

#[cfg(test)]
mod test {
  use super::Matrix;
  #[test]
  fn test_from_vec() {
    let matrix = Matrix::from_vec(vec![vec![1, 1, 1], vec![1, 1, 1]]);
    assert_eq!(matrix.row_count, 2);
    assert_eq!(matrix.col_count, 3);
  }

  #[test]
  fn test_to_vec() {
    let vec = vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]];
    let expected = vec.clone();
    let matrix = Matrix::from_vec(vec);
    assert_eq!(matrix.to_vec(), expected);
  }

  #[test]
  fn test_transpose_1() {
    #[rustfmt::skip]
  let matrix = Matrix::from_vec(vec![
      vec![1, 2],
      vec![3, 4],
      vec![5, 6]
  ]);
    #[rustfmt::skip]
  let expected = Matrix::from_vec(vec![
      vec![1, 3, 5],
      vec![2, 4, 6]
  ]);
    assert_eq!(Matrix::transpose(&matrix), expected);
  }

  #[test]
  fn test_transpose_2() {
    #[rustfmt::skip]
  let matrix = Matrix::from_vec(vec![
      vec![1, 3, 5],
      vec![2, 4, 6]
  ]);
    #[rustfmt::skip]
  let expected = Matrix::from_vec(vec![
      vec![1, 2],
      vec![3, 4],
      vec![5, 6]
  ]);
    assert_eq!(Matrix::transpose(&matrix), expected);
  }

  #[test]
  fn test_rotate_right() {
    #[rustfmt::skip]
  let matrix = Matrix::from_vec(vec![
      vec![0, 1, 0],
      vec![1, 1, 1],
  ]);
    #[rustfmt::skip]
  let expected = Matrix::from_vec(vec![
      vec![1, 0],
      vec![1, 1],
      vec![1, 0],
  ]);
    assert_eq!(Matrix::rotate_right(&matrix), expected);
  }

  #[test]
  fn test_collides() {
    #[rustfmt::skip]
  let mut a = Matrix::from_vec(vec![
    vec![0, 0, 0],
    vec![0, 1, 0],
    vec![1, 1, 1]
  ]);
    #[rustfmt::skip]
  let b = Matrix::from_vec(vec![
    vec![0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0],
    vec![0, 0, 1, 1, 0, 0],
    vec![0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0],
  ]);
    assert_eq!(a.collides(&b), false);
    a.origin = (1, 1);
    assert_eq!(a.collides(&b), true);
    a.origin = (1, 3);
    assert_eq!(a.collides(&b), false);
  }

  #[test]
  fn test_resolution_x_left() {
    let a = Matrix::from_vec(vec![vec![1, 1, 1, 1]]);
    let b = Matrix::from_vec(vec![vec![0, 1, 0, 0, 0, 0]]);
    assert_eq!(a.resolution_x(&b), 2);
  }

  #[test]
  fn test_resolution_x_right() {
    let a = Matrix::from_vec(vec![vec![1, 1, 1, 1]]);
    let b = Matrix::from_vec(vec![vec![0, 0, 1, 1, 0, 0]]);
    assert_eq!(a.resolution_x(&b), -2);
  }

  #[test]
  fn test_resolution_x_no_overlap() {
    let a = Matrix::from_vec(vec![vec![1, 1, 1, 1]]);
    let b = Matrix::from_vec(vec![vec![0, 0, 0, 0, 0, 0]]);
    assert_eq!(a.resolution_x(&b), 0);
  }

  #[test]
  fn test_resolution_x_multiple_overlap() {
    #[rustfmt::skip]
  let mut a = Matrix::from_vec(vec![
    vec![1, 0],
    vec![0, 1]
  ]);
    #[rustfmt::skip]
  let b = Matrix::from_vec(vec![
    vec![0, 0, 0, 0],
    vec![0, 0, 1, 0],
    vec![0, 0, 0, 1],
    vec![0, 0, 0, 0],
  ]);
    a.origin = (2, 1);
    assert_eq!(a.resolution_x(&b), 1);
  }

  #[test]
  fn test_resolution_x_bounds_collision() {
    let mut a = Matrix::from_vec(vec![
      vec![0, 0, 0, 0],
      vec![0, 0, 0, 0],
      vec![1, 1, 1, 1],
      vec![0, 0, 0, 0],
    ]);
    let b = Matrix::from_vec(vec![
      vec![0, 0, 0, 0, 0, 0],
      vec![0, 0, 0, 0, 0, 0],
      vec![0, 1, 0, 0, 0, 0],
      vec![0, 1, 0, 0, 0, 0],
      vec![0, 1, 0, 0, 0, 0],
      vec![0, 1, 0, 0, 0, 0],
    ]);
    // Out of bounds (max)
    a.origin = (0, 2);
    assert_eq!(a.resolution_x(&b), 2);
  }

  #[test]
  fn test_min_y() {
    let a = Matrix::from_vec(vec![vec![1, 1, 1, 1]]);
    let b = Matrix::from_vec(vec![vec![0, 1, 0], vec![1, 1, 1]]);
    let c = Matrix::from_vec(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]);
    assert_eq!(a.min_y(), Some(0));
    assert_eq!(b.min_y(), Some(0));
    assert_eq!(c.min_y(), Some(1));
  }

  #[test]
  fn test_max_y() {
    let a = Matrix::from_vec(vec![vec![1, 1, 1, 1]]);
    let b = Matrix::from_vec(vec![vec![0, 1, 0], vec![1, 1, 1]]);
    let c = Matrix::from_vec(vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]]);
    assert_eq!(a.max_y(), Some(0));
    assert_eq!(b.max_y(), Some(1));
    assert_eq!(c.max_y(), Some(2));
  }

  #[test]
  fn add_in_bounds() {
    #[rustfmt::skip]
  let mut a = Matrix::from_vec(vec![
    vec![0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0],
  ]);
    #[rustfmt::skip]
  let mut b = Matrix::from_vec(vec![
    vec![1, 1, 1],
    vec![1, 1, 1]
  ]);
    #[rustfmt::skip]
  let expected = Matrix::from_vec(vec![
    vec![0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0],
    vec![0, 0, 1, 1, 1, 0],
    vec![0, 0, 1, 1, 1, 0],
    vec![0, 0, 0, 0, 0, 0],
  ]);
    b.origin = (2, 3);
    a.add(&b);
    assert_eq!(a.cells, expected.cells);
  }

  #[test]
  fn add_in_bounds_2() {
    #[rustfmt::skip]
  let mut a = Matrix::from_vec(vec![
    vec![0, 0, 0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  ]);
    #[rustfmt::skip]
  let mut b = Matrix::from_vec(vec![
      vec![0, 0, 0, 0],
      vec![1, 1, 1, 1],
      vec![0, 0, 0, 0],
      vec![0, 0, 0, 0]
  ]);
    #[rustfmt::skip]
  let expected = Matrix::from_vec(vec![
    vec![0, 0, 0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
    vec![1, 1, 1, 1, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0, 0, 0],
  ]);
    a.origin = (2, 0);
    b.origin = (2, 1);
    a.add(&b);
    assert_eq!(a.cells, expected.cells);
  }

  #[test]
  fn add_test_bounds() {
    #[rustfmt::skip]
  let mut a = Matrix::from_vec(vec![
    vec![0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0],
  ]);
    #[rustfmt::skip]
  let mut b = Matrix::from_vec(vec![
    vec![1, 1, 1],
    vec![1, 1, 1]
  ]);
    #[rustfmt::skip]
  let expected = Matrix::from_vec(vec![
    vec![0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 0, 0],
    vec![0, 0, 0, 0, 1, 1],
    vec![0, 0, 0, 0, 1, 1],
  ]);
    b.origin = (4, 4);
    a.add(&b);
    assert_eq!(a.cells, expected.cells);
  }
}
