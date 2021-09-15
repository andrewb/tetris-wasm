use crate::bag::Bag;
use crate::board::Board;
use crate::matrix::Matrix;
use crate::piece::Piece;
// use crate::utils::set_panic_hook;

use std::collections::HashSet;
use std::fmt;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmd {
  Left,
  Right,
  Rotate,
  Drop,
}

#[wasm_bindgen]
pub struct Game {
  pub score: u32,
  pub elapsed_time: f32,
  #[wasm_bindgen(js_name = gameOver)]
  pub game_over: bool,
  board: Board,
  bag: Bag,
  piece: Piece,
  commands: HashSet<Cmd>,
  drop_speed: f32,
  input_rate: f32,
  last_input_time: f32,
  last_drop_time: f32,
}

fn next_piece(bag: &mut Bag, board: &Board) -> Piece {
  let mut piece = bag.take_piece();
  let origin_x = board.matrix.col_count as f32 / 2.0 - piece.matrix.col_count as f32 / 2.0;
  let mut origin_y = 0;
  // Find the piece's max y position
  let max_block_y = piece.matrix.max_y().unwrap_or(0);
  // Find the first occupied row on the board
  let min_board_y = match board.matrix.min_y() {
    Some(min_y) => min_y,
    None => (board.matrix.row_count - 1) as i8,
  };
  if max_block_y >= min_board_y {
    origin_y = max_block_y - min_board_y - 1;
  }
  piece.matrix.origin = (origin_x.floor() as i8, origin_y);
  piece
}

#[wasm_bindgen]
impl Game {
  pub fn new(row_count: u8, col_count: u8) -> Game {
    // set_panic_hook();
    let mut bag = Bag::new();
    let board = Board::new(row_count, col_count);
    let piece = next_piece(&mut bag, &board);
    Game {
      score: 0,
      bag,
      board,
      piece,
      commands: HashSet::new(),
      elapsed_time: 0.0,
      input_rate: 1.0 / 60.0,
      last_input_time: 0.0,
      drop_speed: 0.5,
      last_drop_time: 0.0,
      game_over: false,
    }
  }
  #[wasm_bindgen(js_name = pushCommand)]
  pub fn push_command(&mut self, cmd: Cmd) {
    self.commands.insert(cmd);
  }
  pub fn update(&mut self, dt: f32) {
    if self.game_over {
      return;
    }
    self.elapsed_time += dt;
    let row_count = self.board.matrix.row_count as i8;
    let col_count = self.board.matrix.col_count as i8;
    let row_max = row_count - 1;
    let col_max = col_count - 1;
    let input_timeout = (self.elapsed_time - self.last_input_time) > self.input_rate;
    let drop_timeout = (self.elapsed_time - self.last_drop_time) > self.drop_speed;

    // User input
    if input_timeout {
      for cmd in self.commands.iter() {
        let mut next_matrix = self.piece.matrix.clone();
        let piece_min_x = self.piece.matrix.min_x().unwrap();
        let piece_max_x = self.piece.matrix.max_x().unwrap();
        match cmd {
          Cmd::Rotate => {
            next_matrix = Matrix::rotate_right(&self.piece.matrix);
            // Check if the rotated piece in in bounds
            let rot_piece_min_x = next_matrix.min_x().unwrap();
            let rot_piece_max_x = next_matrix.max_x().unwrap();
            let mut next_x = if rot_piece_min_x < 0 {
              // Move to right by offset
              next_matrix.origin.0 + rot_piece_min_x.abs()
            } else if rot_piece_max_x > col_max {
              // Move to left by offset
              next_matrix.origin.0 - (rot_piece_max_x - col_max)
            } else {
              // No change necessary
              next_matrix.origin.0
            };
            // Apply horizontal resolution
            next_x += next_matrix.resolution_x(&self.board.matrix);
            next_matrix.origin = (next_x, next_matrix.origin.1);
          }
          Cmd::Left => {
            if piece_min_x > 0 {
              next_matrix.origin.0 -= 1;
            }
          }
          Cmd::Right => {
            if piece_max_x < col_max {
              next_matrix.origin.0 += 1;
            }
          }
          Cmd::Drop => {
            self.piece.matrix.origin = self.drop_pos();
            self.last_drop_time = self.elapsed_time;
            self.piece.at_rest = true;
          }
        }

        if self.piece.at_rest {
          break;
        }

        if !(next_matrix.collides(&self.board.matrix)) {
          // Update piece
          self.piece.matrix = next_matrix;
        }
      }
      self.commands.clear();
      self.last_input_time = self.elapsed_time;
    }

    // Vertical movement
    if drop_timeout {
      let mut v_next_matrix = self.piece.matrix.clone();
      v_next_matrix.origin.1 += 1;

      let v_next_max_y = v_next_matrix.max_y().unwrap_or(0);

      if v_next_max_y > row_max || v_next_matrix.collides(&self.board.matrix) {
        // Piece is at rest
        self.piece.at_rest = true;
      } else {
        self.piece.matrix = v_next_matrix;
      }
      self.last_drop_time = self.elapsed_time;
    }

    if self.piece.at_rest {
      // Add to board
      self.board.matrix.add(&self.piece.matrix);
      // Check if piece is at top
      self.game_over = match self.piece.matrix.min_y() {
        Some(min_y) => min_y <= 0,
        None => false,
      };
      // 1 point for each block added to the board
      self.score += 1;
      if !self.game_over {
        self.piece = next_piece(&mut self.bag, &self.board);
      }
    }

    // Update board
    let mut board_vec = self.board.matrix.to_vec();
    // Remove full rows from the board
    board_vec.retain(|row| row.iter().any(|&cell| cell == 0));

    let new_row_count = row_count as usize - board_vec.len();
    for _ in 0..new_row_count {
      board_vec.insert(0, vec![0; col_count as usize]);
    }
    self.board.matrix = Matrix::from_vec(board_vec);
    // 10 points for each row
    self.score += (new_row_count * 10) as u32;
  }
  fn drop_pos(&self) -> (i8, i8) {
    let mut matrix = self.piece.matrix.clone();
    let mut collision = false;
    let row_max = self.board.matrix.row_count as i8 - 1;
    while !collision {
      matrix.origin.1 += 1;
      collision = matrix.max_y().unwrap_or(0) > row_max || matrix.collides(&self.board.matrix);
    }
    (matrix.origin.0, matrix.origin.1 - 1)
  }

  #[wasm_bindgen(js_name = boardCellsPtr)]
  pub fn js_board_cells_ptr(&self) -> *const u8 {
    self.board.matrix.cells_ptr()
  }

  #[wasm_bindgen(js_name = pieceCellsPtr)]
  pub fn js_piece_cells_ptr(&self) -> *const u8 {
    self.piece.matrix.cells_ptr()
  }

  #[wasm_bindgen(js_name = pieceSize)]
  pub fn js_piece_size(&self) -> Vec<u8> {
    vec![self.piece.matrix.col_count, self.piece.matrix.row_count]
  }

  #[wasm_bindgen(js_name = pieceCoord)]
  pub fn js_piece_coord(&self) -> Vec<i8> {
    vec![self.piece.matrix.origin.0, self.piece.matrix.origin.1]
  }

  #[wasm_bindgen(js_name = dropCoord)]
  pub fn js_drop_coord(&self) -> Vec<i8> {
    let drop_pos = self.drop_pos();
    vec![drop_pos.0, drop_pos.1]
  }
}

impl fmt::Display for Game {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut state = Matrix::from_vec(self.board.matrix.to_vec());
    let mut output = String::new();

    if !self.game_over {
      state.add(&self.piece.matrix);
    }

    for row in state.to_vec().iter() {
      let as_string: String = row
        .iter()
        .map(|i| match i {
          0 => '□',
          1..=7 => '■',
          _ => '?',
        })
        .collect();
      output.push_str(&as_string);
      output.push('\n');
    }
    write!(f, "{}", output)
  }
}
