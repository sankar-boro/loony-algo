use algos::recursion_and_backtracking::generate_parentheses;

fn main() -> std::result::Result<(), ()> {
    let n = 3;  // Number of pairs
    let mut result = Vec::new();
    generate_parentheses(n, 0, 0, String::new(), &mut result);
    println!("res: {:?}", result);
    Ok(())
}