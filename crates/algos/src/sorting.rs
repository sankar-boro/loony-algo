/// Bubble Sort Algorithm

/// Bubble Sort is a simple sorting algorithm that repeatedly steps through the list, 
/// compares adjacent elements, and swaps them if they are in the wrong order. 
/// The process continues until the list is sorted.

/// - **Time Complexity**:
///   - Best Case: O(n) (Already sorted)
///   - Worst Case: O(n²)
///   - Average Case: O(n²)
/// - **Space Complexity**: O(1) (In-place sorting)
/// - **Stable Sorting**: Yes (Preserves the order of equal elements)

/// ## Algorithm Steps
/// 1. Iterate through the array from the first element to the last.
/// 2. Compare adjacent elements and swap them if they are in the wrong order.
/// 3. Repeat the process for all elements until no swaps are needed.
pub fn bubble_sort(arr: &mut Vec<i32>) {
    let n = arr.len(); // 8
    let n = n - 1; // 7
    for i in 0..=n {
        let mut swapped = false;
        for j in 1..=(n - i) {
            if arr[j - 1] > arr[j] {
                arr.swap(j - 1, j);
                swapped = true;
            }
        }
        if !swapped {
            break; // Optimization: Stop if already sorted
        }
    }
}

/// ### **Selection Sort in Rust**
/// **Selection Sort** is a simple **comparison-based sorting algorithm** that repeatedly 
/// selects the smallest (or largest) element from an unsorted portion and moves it to the 
/// sorted portion.

/// **Time Complexity**:  
///   - Best Case: O(n) (Aldready sorted)  
///   - Worst Case: O(n²)  
///   - Average Case: O(n²)  

/// **Space Complexity**:  
///   - O(1) (In-place sorting)

/// **When to Use?**  
///   - Small datasets  
///   - When swapping cost is low  
///   - Simple and easy to implement  
pub fn selection_sort(arr: &mut Vec<i32>) {
    let n = arr.len();
    for i in 0..n {
        let mut min_index = i;
        for j in (i + 1)..n {
            if arr[j] < arr[min_index] {
                min_index = j;
            }          
        }
        if min_index != i {
            arr.swap(i, min_index);
        }
    }
}

/// ### **Insertion Sort**
/// Insertion Sort is a simple and efficient sorting algorithm that works similarly to how we sort playing cards in our hands. 
/// It builds the final sorted array one item at a time, shifting elements as necessary to insert each item into its correct position.

/// #### **Algorithm Steps**
/// 1. Start with the second element (index 1) and compare it with the first.
/// 2. If it's smaller, shift the first element to the right and insert the current element.
/// 3. Move to the next element and compare it with the sorted portion, shifting elements as needed.
/// 4. Repeat this process until the entire array is sorted.

/// #### **Complexity**
/// - **Best Case (Already Sorted)**: **O(n)**
/// - **Worst and Average Case**: **O(n²)**
/// - **Space Complexity**: **(O(1)** (In-place sorting)
pub fn insertion_sort(arr: &mut Vec<i32>) {
    let size_of_array = arr.len();
    for index in 1..size_of_array {
        let current_element = arr[index];
        let mut j = index;
        while j > 0 && arr[j - 1] > current_element {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = current_element;
    }
}

/// ### **Merge Sort Explanation**
/// Merge Sort is a **divide and conquer** sorting algorithm that splits an array into smaller subarrays, 
/// sorts them, and then merges them back together in a sorted manner. It has a **time complexity** of **O(n log n)** in all cases, 
/// making it efficient for large datasets.

/// ### **Steps of Merge Sort**
/// 1. **Divide**: Split the array into two halves until each subarray contains a single element.
/// 2. **Conquer**: Recursively sort the subarrays.
/// 3. **Merge**: Merge the sorted subarrays to produce a final sorted array.
/// 
/// ### **Breakdown of the Code**
/// - **`merge_sort` function**: 
///   - Recursively divides the array into two halves.
///   - Calls itself until each part has only one element.
///   - Merges the sorted halves using `merge`.
  
/// - **`merge` function**:
///   - Uses two pointers to compare elements from the left and right halves.
///   - Appends the smallest element to a new merged vector.
///   - Adds remaining elements from either left or right when one half is exhausted.

/// ### **Time Complexity Analysis**
/// | Case         | Complexity |
/// |-------------|------------|
/// | Best Case   | O(n log n) |
/// | Worst Case  | O(n log n) |
/// | Average Case| O(n log n) |
pub fn merge_sort(arr: &mut Vec<i32>) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    let mid = n / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    merge_sort(&mut left);
    merge_sort(&mut right);

    merge(arr, &left, &right);
}

fn merge(arr: &mut Vec<i32>, left: &Vec<i32>, right: &Vec<i32>) {
    println!("arr: {:?} left:{:?} right: {:?}", arr, left, right);
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }
}

/// Quicksort is a **divide-and-conquer** sorting algorithm that is highly efficient for large datasets. 
/// It works by selecting a **pivot** element, partitioning the array so that elements less than the pivot 
/// go to the left and elements greater go to the right, and then recursively sorting the partitions.

/// ### **Steps of QuickSort:**
/// 1. **Choose a Pivot**: Select an element (can be the first, last, random, or median).
/// 2. **Partitioning**: Rearrange the array so that:
///    - Elements **less than** the pivot are on the left.
///    - Elements **greater than** the pivot are on the right.
/// 3. **Recursion**: Apply QuickSort to the left and right partitions.
/// 4. **Base Case**: When a partition has one or zero elements, it's already sorted.

/// ### **Time Complexity:**
/// - **Best/Average Case:** \( O(n \log n) \) (Balanced partitions)
/// - **Worst Case:** \( O(n^2) \) (Unbalanced partitions, e.g., sorted array with bad pivot choice)
/// - **Space Complexity:** \( O(\log n) \) (Recursive stack calls)
pub fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot = partition(arr);
    quick_sort(&mut arr[0..pivot]);
    quick_sort(&mut arr[pivot + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr.len() - 1;
    println!("\nBefore Array: {:?}. Before Pivot: {}", arr, pivot);
    let mut i = 0;
    
    for j in 0..pivot {
        if arr[j] < arr[pivot] {
            arr.swap(i, j);
            println!("Swap. arr: {:?}", arr);
            i += 1;
        }
    }
    arr.swap(i, pivot);
    println!("After. arr: {:?}, pivot: {}", arr, i);
    i
}

fn heap_sort(arr: &mut Vec<i32>) {
    let n = arr.len();

    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify(arr: &mut Vec<i32>, n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] > arr[largest] {
        largest = left;
    }
    if right < n && arr[right] > arr[largest] {
        largest = right;
    }
    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

fn counting_sort(arr: &mut Vec<i32>) {
    if arr.is_empty() {
        return;
    }

    let max = *arr.iter().max().unwrap();
    let min = *arr.iter().min().unwrap();
    let range = (max - min + 1) as usize;
    let mut count = vec![0; range];
    let mut output = vec![0; arr.len()];

    for &num in arr.iter() {
        count[(num - min) as usize] += 1;
    }

    for i in 1..range {
        count[i] += count[i - 1];
    }

    for &num in arr.iter().rev() {
        output[count[(num - min) as usize] - 1] = num;
        count[(num - min) as usize] -= 1;
    }

    arr.copy_from_slice(&output);
}

fn radix_sort(arr: &mut Vec<i32>) {
    let max = *arr.iter().max().unwrap();
    let mut exp = 1;

    while max / exp > 0 {
        counting_sort_radix(arr, exp);
        exp *= 10;
    }
}

fn counting_sort_radix(arr: &mut Vec<i32>, exp: i32) {
    let mut output = vec![0; arr.len()];
    let mut count = vec![0; 10];

    for &num in arr.iter() {
        count[((num / exp) % 10) as usize] += 1;
    }

    for i in 1..10 {
        count[i] += count[i - 1];
    }

    for &num in arr.iter().rev() {
        output[count[((num / exp) % 10) as usize] - 1] = num;
        count[((num / exp) % 10) as usize] -= 1;
    }

    arr.copy_from_slice(&output);
}

fn bucket_sort(arr: &mut Vec<f32>) {
    let n = arr.len();
    let mut buckets: Vec<Vec<f32>> = vec![Vec::new(); n];

    for &num in arr.iter() {
        let idx = (num * n as f32) as usize;
        buckets[idx].push(num);
    }

    for bucket in &mut buckets {
        bucket.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }

    let mut idx = 0;
    for bucket in buckets {
        for &num in bucket.iter() {
            arr[idx] = num;
            idx += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut data = vec![5, 2, 3, 1, 7, 8, 9];
        let a = bubble_sort(&mut data);
        assert_eq!(vec![1, 2, 3, 5, 7, 8, 9], data);
    }

    #[test]
    fn test_selection_sort() {
        let mut data = vec![5, 2, 3, 1, 7, 8, 9];
        let a = selection_sort(&mut data);
        assert_eq!(vec![1, 2, 3, 5, 7, 8, 9], data);
    }


    #[test]
    fn test_insertion_sort() {
        let mut data = vec![5, 2, 3, 1, 7, 8, 9];
        let a = selection_sort(&mut data);
        assert_eq!(vec![1, 2, 3, 5, 7, 8, 9], data);
    }
}
