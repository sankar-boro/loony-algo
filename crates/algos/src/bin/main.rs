use algos::n_queens;
use algos::recursion_and_backtracking;
use algos::recursion_and_backtracking::print_solutions;

fn main() {
    // n_queens::run(4);
    let n = 5;
    let res = recursion_and_backtracking::solve_n_queens(n);
    print_solutions(&res, n);  
    println!("{res:?}")  
}