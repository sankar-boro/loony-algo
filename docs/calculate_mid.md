The formula:

\[
\text{mid} = \text{left} + \frac{(\text{right} - \text{left})}{2}
\]

is used to calculate the middle index when performing a binary search or other divide-and-conquer techniques in Data Structures (DS).

### **How It Works**

1. **Standard Mid Calculation**  
   Normally, the middle index between `left` and `right` in an array is calculated as:
   \[
   \text{mid} = \frac{\text{left} + \text{right}}{2}
   \]
   However, this can cause **integer overflow** in languages with fixed integer sizes when `left + right` exceeds the maximum integer value.

2. **Avoiding Integer Overflow**  
   Instead of directly adding `left` and `right`, we use:
   \[
   \text{mid} = \text{left} + \frac{(\text{right} - \text{left})}{2}
   \]
   - `right - left` ensures that the difference remains within a safe range.
   - Adding `left` back adjusts the mid-point correctly.

### **Example**

#### **Case Without Overflow**

For `left = 2`, `right = 8`:

\[
\text{mid} = 2 + \frac{(8 - 2)}{2} = 2 + \frac{6}{2} = 2 + 3 = 5
\]

#### **Case Avoiding Overflow**

For very large values, e.g., `left = 2,000,000,000` and `right = 2,100,000,000`:

- Standard formula:  
  \[
  \text{mid} = \frac{2000000000 + 2100000000}{2} = \frac{4100000000}{2}
  \]
  This might **cause overflow** in a 32-bit integer system.

- Safer formula:
  \[
  \text{mid} = 2000000000 + \frac{(2100000000 - 2000000000)}{2}
  \]
  \[
  = 2000000000 + \frac{100000000}{2} = 2000000000 + 50000000 = 2050000000
  \]
  This prevents overflow while achieving the correct midpoint.

### **Use Case**

This formula is widely used in:

- **Binary Search**
- **Divide and Conquer Algorithms**
- **Sorting Algorithms (Merge Sort, Quick Sort)**

Let me know if you need further clarification! 🚀
