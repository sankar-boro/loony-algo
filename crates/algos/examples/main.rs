use algos::{
    search::exponential_search,
    sorting::insertion_sort
};


fn test_sorting() {
    let mut data = vec![5, 9, 4, 7, 6, 3, 2, 1];
    insertion_sort(&mut data);
    println!("Sorted array: {:?}", data);
}

fn test_exponential_search() {
    let data = vec![1, 2, 3, 4, 5, 6, 7];
    let res = exponential_search(&data, 6);
    println!("Index: {:?}", res);
}

fn main() -> std::result::Result<(), ()> {
    test_exponential_search();
    Ok(())
}