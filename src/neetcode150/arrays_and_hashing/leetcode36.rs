#![allow(dead_code)]
struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut seen: HashSet<String> = HashSet::new();

        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != '.' {
                    let row_hash = format!("Row {} has value {}", i, board[i][j]);
                    let col_hash = format!("Col {} has value {}", j, board[i][j]);
                    let box_hash = format!("Box ({}, {}) has value {}", i / 3, j / 3, board[i][j]);

                    if seen.contains(&row_hash)
                        || seen.contains(&col_hash)
                        || seen.contains(&box_hash)
                    {
                        return false;
                    }

                    seen.insert(row_hash);
                    seen.insert(col_hash);
                    seen.insert(box_hash);
                }
            }
        }

        true
    }
}
