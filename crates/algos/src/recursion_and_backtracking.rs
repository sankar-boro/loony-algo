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
