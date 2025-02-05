/// Bubble Sort Algorithm

/// Bubble Sort is a simple sorting algorithm that repeatedly steps through the list, 
/// compares adjacent elements, and swaps them if they are in the wrong order. The process continues until the list is sorted.

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
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..(n - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break; // Optimization: Stop if already sorted
        }
    }
}

fn selection_sort(arr: &mut Vec<i32>) {
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

fn insertion_sort(arr: &mut Vec<i32>) {
    let n = arr.len();
    for i in 1..n {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

fn merge_sort(arr: &mut Vec<i32>) {
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

fn quick_sort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot = partition(arr);
    quick_sort(&mut arr[0..pivot]);
    quick_sort(&mut arr[pivot + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let pivot = arr.len() - 1;
    let mut i = 0;

    for j in 0..pivot {
        if arr[j] < arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot);
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
