use algos::quick_sort;

fn main() -> std::result::Result<(), ()> {
    let mut data = vec![5, 2, 4, 7, 6, 3, 2, 1];
    quick_sort(&mut data);
    println!("Sorted array: {:?}", data);
    Ok(())
}