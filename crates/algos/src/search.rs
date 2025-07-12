/// ## **Binary Search Explained**
/// Binary Search is an efficient algorithm for finding a target element in a **sorted array**. It works by repeatedly dividing the search space in half, reducing the problem size exponentially.

/// ---

/// ## **How Binary Search Works**
/// 1. **Initialize Pointers**:  
///    - Set `left = 0` and `right = n - 1` (where `n` is the array length).
   
/// 2. **Find the Middle Element**:  
///    - Compute `mid = left + (right - left) / 2` (to prevent overflow).
   
/// 3. **Compare & Adjust Search Space**:
///    - If `arr[mid] == target`: Return `mid` (found the element).
///    - If `arr[mid] < target`: Search in the **right half** (`left = mid + 1`).
///    - If `arr[mid] > target`: Search in the **left half** (`right = mid - 1`).

/// 4. **Repeat** the process until `left > right`. If the element is not found, return `-1`.

/// ---

/// ## **Time Complexity Analysis**
/// | Case | Complexity |
/// |------|------------|
/// | Best Case | **O(1)** (if `mid` is the target) |
/// | Worst Case | **O(log n)** (reducing search space by half each step) |
/// | Average Case | **O(log n)** |

/// Binary Search is significantly **faster than Linear Search (O(n))** for large datasets.

/// ---

/// ## **When to Use Binary Search**
/// ✅ The array is **sorted**.  
/// ✅ The dataset is **large** (since binary search is O(log n)).  
/// ✅ Searching needs to be **fast** (e.g., searching in databases, dictionaries).  
pub fn binary_search(arr: &[i32], target: i32, left__: Option<usize>, right__: Option<usize>) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len();

    if left__.is_some() {
        left = left__.unwrap() as usize;
    }

    if right__.is_some() {
        right = right__.unwrap() as usize;
    }
    
    while left < right {
        // is a common pattern in binary search algorithms, and it's written this way for a specific reason: to prevent integer overflow.
        let mid = left + (right - left) / 2;

        if target == arr[mid] {
            return Some(mid);
        } else if target > arr[mid] {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    None
}

/// Exponential search is an algorithm for finding a target element in a sorted array. 
/// It works by first identifying a range where the element might be present and then performing a binary search within that range. 
/// This search method is particularly efficient for searching in **unbounded or infinite** sorted arrays.

/// ### **Steps of Exponential Search**
/// 1. **Find the Search Range**  
///    - Start at index `1` and keep doubling the index (`2, 4, 8, 16, ...`) until:
///      - The index goes out of bounds, or  
///      - The element at the index is greater than or equal to the target.

/// 2. **Perform Binary Search**  
///    - Once the range `[prev, current]` is found, apply **binary search** within this range.

/// ---

/// ### **Time Complexity Analysis**
/// - **Best case (O(1))**: If the element is found at the first position.  
/// - **Average case (O(log i))**: Where `i` is the index at which the element is found.  
/// - **Worst case (O(log n))**: When the element is at the end of the array.

/// This makes **Exponential Search faster** than Binary Search when searching in an **unbounded or large sorted dataset** because it reduces the number of comparisons.

/// ### **Use Cases**
/// - **Searching in large sorted arrays**  
/// - **Searching in infinite or unbounded arrays** (e.g., Google search index)  
/// - **Sparse datasets** (e.g., dictionary lookups)  
pub fn exponential_search(arr: &[i32], target: i32) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let mut bound = 1;
    while bound < arr.len() && arr[bound] < target {
        bound *= 2;
    }

    let left = bound / 2;
    let right = arr.len().min(bound) - 1;

    binary_search(arr, target, Some(left), Some(right))
}

/// ### Explanation:
/// 1. The function takes a reference to a sorted slice (`arr`), the `target` value, and the current search bounds (`low`, `high`).
/// 2. It calculates the middle index and checks:
///    - If `arr[mid]` is the target, return `Some(mid)`.
///    - If `arr[mid]` is greater than the target, search the left half.
///    - If `arr[mid]` is smaller, search the right half.
/// 3. The recursion continues until `low > high`, meaning the target isn't in the array.

/// ### Edge Cases Handled:
/// - Avoids underflow when `mid == 0` and `target` is in the left half.
/// - Works correctly with `usize` indices.
pub fn binary_search_recursive(arr: &[i32], target: i32, low: usize, high: usize) -> Option<usize> {
    if low > high {
        return None;
    }

    let mid = low + (high - low) / 2;

    if arr[mid] == target {
        Some(mid)
    } else if arr[mid] > target {
        if mid == 0 {
            None
        } else {
            binary_search_recursive(arr, target, low, mid - 1)
        }
    } else {
        binary_search_recursive(arr, target, mid + 1, high)
    }
}
