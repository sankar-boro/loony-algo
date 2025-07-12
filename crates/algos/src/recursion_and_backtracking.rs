// ### **Recursion**  
// Recursion is a technique where a function calls itself to break a complex problem into smaller subproblems. Each recursive call works on a reduced version of the original problem until it reaches a **base case**, which stops further recursion.  

// 📌 **Key Points:**  
// - Uses a base case to stop recursion.  
// - Can be inefficient if not optimized (e.g., Fibonacci without memoization).  
// - Used in problems like tree traversal, sorting (QuickSort, MergeSort), etc.

// ---

// ### **Backtracking**  
// Backtracking is a problem-solving technique that **tries all possible solutions** recursively. If a solution is invalid or does not lead to the desired result, it **backtracks** (undoes the last step) and tries another option.  

// 📌 **Key Points:**  
// - Explores all possibilities but **prunes invalid paths** to optimize performance.  
// - Used in **constraint-based problems** like N-Queens, Sudoku Solver, and Maze solving.  
// - Works well when a brute-force approach is too slow.  

// ---

// ### **Recursion vs Backtracking**
// | Feature       | Recursion | Backtracking |
// |--------------|----------|-------------|
// | Definition   | A function calls itself to solve a problem | A trial-and-error method that explores all possibilities and backtracks when needed |
// | Approach     | Solves smaller subproblems recursively | Tries a solution, backtracks if invalid, and explores alternatives |
// | Base Case    | Necessary to stop infinite recursion | No fixed base case; stops when a solution is found or all options fail |
// | Examples     | Fibonacci, Factorial, Merge Sort | Sudoku, N-Queens, Subset Generation |
// | Efficiency   | Can lead to redundant calls (optimized using memoization) | Eliminates unnecessary calculations using pruning |
pub fn generate_parentheses(n: usize, open: usize, close: usize, current: String, result: &mut Vec<String>) {
    if current.len() == n * 2 {
        result.push(current);
        return;
    }

    // Add '(' if we have remaining open brackets to use
    if open < n {
        generate_parentheses(n, open + 1, close, format!("{}(", current), result);
    }

    // Add ')' only if open > close to maintain validity
    if close < open {
        generate_parentheses(n, open, close + 1, format!("{})", current), result);
    }
}



// ### **N-Queens Problem**
// The **N-Queens problem** is a classic combinatorial problem in which we must place **N queens on an N×N chessboard** such that:
// 1. No two queens share the same **row**.
// 2. No two queens share the same **column**.
// 3. No two queens share the same **diagonal**.

// The challenge is to find all possible solutions or just one valid configuration.
// ### **Explanation of the Code**
// 1. `solve_n_queens(n)`:  
//    - Initializes an empty board.
//    - Calls the `backtrack` function.

// 2. `backtrack(row, n, board, solutions)`:  
//    - If `row == n`, a valid solution is found, so we store it.
//    - Otherwise, we try placing a queen in each column and recursively move to the next row.

// 3. `is_valid(row, col, board)`:  
//    - Checks if placing a queen at `(row, col)` is safe by ensuring:
//      - No other queen is in the same column.
//      - No other queen is on the same diagonals.

// 4. `print_solutions(solutions, n)`:  
//    - Prints the board configurations with 'Q' for queens and '.' for empty spaces.
// ### **Time Complexity**
// - **O(N!)** in the worst case since we try every possible queen placement.
// - **Optimized with pruning** (validity checks) to avoid unnecessary recursion.
pub fn solve_n_queens(n: usize) -> Vec<Vec<usize>> {
    let mut solutions = Vec::new();
    let mut board = vec![0; n]; // Board represented as an array where index = row, value = column
    backtrack(0, n, &mut board, &mut solutions);
    solutions
}

fn backtrack(row: usize, n: usize, board: &mut Vec<usize>, solutions: &mut Vec<Vec<usize>>) {
    if row == n {
        solutions.push(board.clone());
        return;
    }
    
    for col in 0..n {
        if is_valid(row, col, board) {
            board[row] = col;
            backtrack(row + 1, n, board, solutions);
        }
    }
}

fn is_valid(row: usize, col: usize, board: &Vec<usize>) -> bool {
    for prev_row in 0..row {
        let row_col_value = board[prev_row];
        if row_col_value == col {
            return  false;
        } else if prev_row as isize - row_col_value as isize == row as isize - col as isize {
            return false;
        } else if prev_row as isize + row_col_value as isize == row as isize + col as isize {
            return false;
        }

        // else if row_col_value as isize - col as isize == row as isize - prev_row as isize{
        //     return false
        // } else if col as isize - row_col_value as isize == row as isize - prev_row as isize {
        //     return false
        // }
        
    }
    true
}

pub fn print_solutions(solutions: &Vec<Vec<usize>>, n: usize) {
    for solution in solutions {
        for &col in solution {
            let mut row = vec!['.'; n];
            row[col] = 'Q';
        }
    }
}
