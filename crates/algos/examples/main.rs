use algos::recursion_and_backtracking::{solve_n_queens, print_solutions};

// col=0 if new backtrack is called.
// fn is_valid(colsize_of_a_row: usize, col: usize, col_value: char, board: &mut Vec<char>) -> bool {
//     for curr_col_in_a_row in 0..colsize_of_a_row {
//         let curr_col_value_of_a_row = board[curr_col_in_a_row];
//         let prev_row_value = board[index];

//         if prev_row_value == col {
//             return false;
//         }
//     }
//     return true;
// }

// fn backtrack(row_index: usize, size: usize, board: &mut Vec<char>, solutions: &mut Vec<Vec<char>>) {
//     if row_index == size {
//         solutions.push(board.clone());
//         return;
//     }
//     // row=1, col=0
//     for col in 0..size {
//         if is_valid(row_index, col, board[col], board) {
//             board[row_index] = '*';
//             backtrack(row_index + 1, size, board, solutions);
//         }
//     }
// }

// fn solve_n_queens(size: usize) -> Vec<Vec<char>> {
//     let mut solutions = Vec::new();
//     let mut board = vec!['_'; size];
//    backtrack(0, size, &mut board, &mut solutions);
//    return solutions;
// }

// fn print_solutions(solutions: &Vec<Vec<usize>>, n: usize) {
//     for solution in solutions {
//         for &col in solution {
//             let mut row = vec!['.'; n];
//             row[col] = 'Q';
//             println!("{}", row.iter().collect::<String>());
//         }
//         println!("\n---\n");
//     }
// }


fn main() -> std::result::Result<(), ()> {
    let size =  std::env::var("size").unwrap().parse::<usize>().unwrap();
    let x = solve_n_queens(size);
    // println!("{:?}", x);
    print_solutions(&x, size);
    Ok(())
}