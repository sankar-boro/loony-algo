use std::collections::HashSet;

// The **Sliding Window** technique is useful for problems involving subarrays or contiguous sequences. It helps reduce the time complexity compared to brute force solutions.

// ### **Example: Maximum Sum Subarray of Fixed Size (K)**
// Let’s implement a sliding window approach to find the **maximum sum of a subarray of size `k`**.


/// ### **How It Works:**
/// 1. **Initialize the first window sum** by summing the first `k` elements.
/// 2. **Slide the window**:
///    - Add the next element.
///    - Remove the first element of the previous window.
///    - Update `max_sum` if `current_sum` is greater.
/// 3. Repeat until the array is fully traversed.

/// ### **Time Complexity:**
/// - **O(N)** → Since we traverse the array once.
pub fn max_sum_subarray(arr: &[i32], k: usize) -> Option<i32> {
    if arr.len() < k {
        return None;
    }

    let mut max_sum = arr[..k].iter().sum::<i32>(); // Initial sum of first window
    let mut current_sum = max_sum;

    for i in k..arr.len() {
        current_sum += arr[i] - arr[i - k]; // Slide the window
        max_sum = max_sum.max(current_sum);
    }

    Some(max_sum)
}


// ### **Example: Longest Substring with Unique Characters**
// Here’s a **variable-size** sliding window example to find the **longest substring with unique characters**.
pub fn longest_unique_substring(s: &str) -> usize {
    let mut char_set = HashSet::new();
    let (mut left, mut max_len) = (0, 0);
    let chars: Vec<char> = s.chars().collect();

    for right in 0..chars.len() {
        while char_set.contains(&chars[right]) {
            char_set.remove(&chars[left]);
            left += 1;
        }

        char_set.insert(chars[right]);
        max_len = max_len.max(right - left + 1);
    }

    max_len
}
