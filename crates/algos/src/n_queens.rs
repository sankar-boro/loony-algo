pub fn run(n: usize) {
    let mut solutions: Vec<Vec<usize>> = Vec::new();
    let mut board = vec![0; n]; // Board represented as an array where index = row, value = column
    backtrack(0, n, &mut board, &mut solutions);
}

fn backtrack(row: usize, n: usize, board: &mut Vec<usize>, solutions: &mut Vec<Vec<usize>>) {
    println!("row: {row}, board: {board:?}");
    if row == n {
        solutions.push(board.clone());
        return;
    }

    for col in 0..n {
        if is_valid(row, col, board, solutions) {
            board[row] = col;
            backtrack(row + 1, n, board, solutions);
        }        
    }
}

fn is_valid(row: usize, col: usize, board: &Vec<usize>, solutions: &Vec<Vec<usize>>) -> bool {

    for cur_row in 0..row {
        let x = board[cur_row];
        if x == col {
            return false;
        } else if x + 1 == 1 {

        }
    }

    return true;
}

// [
//     [1, 0, 0, 0],
//     [0, 0, 1, 0],
//     [0, 0, 0, 0],
//     [0, 0, 0, 0]
// ]
