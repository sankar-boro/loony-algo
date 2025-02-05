Here’s the completion of the **15 essential LeetCode patterns** with detailed explanations and examples for each remaining one.

---

## **11. Graph Traversal (BFS/DFS)**

Used for problems involving **graphs, trees, and connected components**.

🔹 **Common Problems**:

- Number of Islands (DFS/BFS)
- Clone Graph
- Word Ladder (BFS)
- Course Schedule (Topological Sort)

### **Example: Number of Islands (DFS)**

```ts
function numIslands(grid: string[][]): number {
  if (!grid.length) return 0;
  let count = 0;
  let rows = grid.length,
    cols = grid[0].length;

  function dfs(r: number, c: number) {
    if (r < 0 || c < 0 || r >= rows || c >= cols || grid[r][c] === "0") return;
    grid[r][c] = "0"; // Mark as visited
    dfs(r + 1, c);
    dfs(r - 1, c);
    dfs(r, c + 1);
    dfs(r, c - 1);
  }

  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < cols; c++) {
      if (grid[r][c] === "1") {
        count++;
        dfs(r, c);
      }
    }
  }

  return count;
}
```

🔹 **Why DFS?**: Recursively explores all connected components.  
🔹 **Why BFS?**: Can be used when shortest paths are needed.

---

## **12. Heap (Priority Queue)**

Used when **finding the smallest/largest element in a dynamic dataset**.

🔹 **Common Problems**:

- Kth Largest Element in an Array
- Merge K Sorted Lists
- Top K Frequent Elements

### **Example: Kth Largest Element**

```ts
class MinHeap {
  heap: number[] = [];

  push(val: number) {
    this.heap.push(val);
    this.heap.sort((a, b) => a - b);
  }

  pop(): number {
    return this.heap.shift()!;
  }

  size(): number {
    return this.heap.length;
  }
}

function findKthLargest(nums: number[], k: number): number {
  let heap = new MinHeap();

  for (let num of nums) {
    heap.push(num);
    if (heap.size() > k) heap.pop();
  }

  return heap.pop();
}
```

🔹 **Why a Min-Heap?**: Keeps the smallest k elements in memory efficiently.  
🔹 **Time Complexity**: **O(N log K)** for inserting elements in the heap.

---

## **13. Union-Find (Disjoint Set)**

Used for **connected components, cycle detection, and network connectivity**.

🔹 **Common Problems**:

- Number of Connected Components
- Redundant Connection (Find Cycle)
- Accounts Merge

### **Example: Find the Number of Connected Components**

```ts
class UnionFind {
  parent: number[];
  rank: number[];

  constructor(size: number) {
    this.parent = Array.from({ length: size }, (_, i) => i);
    this.rank = new Array(size).fill(1);
  }

  find(x: number): number {
    if (this.parent[x] !== x) {
      this.parent[x] = this.find(this.parent[x]); // Path compression
    }
    return this.parent[x];
  }

  union(x: number, y: number): boolean {
    let rootX = this.find(x);
    let rootY = this.find(y);

    if (rootX === rootY) return false;

    if (this.rank[rootX] > this.rank[rootY]) {
      this.parent[rootY] = rootX;
    } else if (this.rank[rootX] < this.rank[rootY]) {
      this.parent[rootX] = rootY;
    } else {
      this.parent[rootY] = rootX;
      this.rank[rootX]++;
    }

    return true;
  }
}

function countComponents(n: number, edges: number[][]): number {
  let uf = new UnionFind(n);
  let count = n;

  for (let [u, v] of edges) {
    if (uf.union(u, v)) count--;
  }

  return count;
}
```

🔹 **Why Union-Find?**: Efficient for **merging and finding** connected components.  
🔹 **Time Complexity**: **O(α(N)) ≈ O(1)** (Inverse Ackermann function).

---

## **14. Trie (Prefix Tree)**

Used for **prefix-based problems, autocomplete, and dictionary lookups**.

🔹 **Common Problems**:

- Implement Trie (Prefix Tree)
- Word Search II
- Longest Word in Dictionary

### **Example: Implement Trie**

```ts
class TrieNode {
  children: Map<string, TrieNode>;
  isEndOfWord: boolean;

  constructor() {
    this.children = new Map();
    this.isEndOfWord = false;
  }
}

class Trie {
  root: TrieNode;

  constructor() {
    this.root = new TrieNode();
  }

  insert(word: string): void {
    let node = this.root;
    for (let char of word) {
      if (!node.children.has(char)) {
        node.children.set(char, new TrieNode());
      }
      node = node.children.get(char)!;
    }
    node.isEndOfWord = true;
  }

  search(word: string): boolean {
    let node = this.root;
    for (let char of word) {
      if (!node.children.has(char)) return false;
      node = node.children.get(char)!;
    }
    return node.isEndOfWord;
  }

  startsWith(prefix: string): boolean {
    let node = this.root;
    for (let char of prefix) {
      if (!node.children.has(char)) return false;
      node = node.children.get(char)!;
    }
    return true;
  }
}
```

🔹 **Why Trie?**: Efficient for **prefix search** operations.  
🔹 **Time Complexity**: **O(N)** per insertion/search.

---

## **15. Greedy Algorithms**

Used when **locally optimal choices lead to a globally optimal solution**.

🔹 **Common Problems**:

- Jump Game
- Activity Selection
- Huffman Encoding

### **Example: Jump Game**

```ts
function canJump(nums: number[]): boolean {
  let maxReach = 0;

  for (let i = 0; i < nums.length; i++) {
    if (i > maxReach) return false; // If we can't reach this index, return false
    maxReach = Math.max(maxReach, i + nums[i]);
  }

  return true;
}
```

🔹 **Why Greedy?**: Keeps track of the farthest index we can reach at each step.  
🔹 **Time Complexity**: **O(N)**.

---

## **Final Summary of the 15 Patterns**

1. **Sliding Window** → Contiguous subarray problems
2. **Two Pointers** → Sorted arrays, linked lists, palindrome checks
3. **Fast & Slow Pointers** → Cycle detection
4. **Merge Intervals** → Sorting & merging overlapping intervals
5. **Cyclic Sort** → Numbers from **1 to N**
6. **Binary Search** → Sorted arrays, finding elements
7. **Bit Manipulation** → XOR tricks
8. **Topological Sort (Kahn’s Algorithm)** → DAG problems
9. **Backtracking** → Combinatorial search
10. **Dynamic Programming** → Subproblems & memoization
11. **Graph Traversal (BFS/DFS)** → Graph connectivity problems
12. **Heap (Priority Queue)** → Kth largest/smallest problems
13. **Union-Find (Disjoint Set)** → Connected components & cycle detection
14. **Trie (Prefix Tree)** → Word search & autocomplete
15. **Greedy Algorithms** → Locally optimal choices

---

### **Next Steps**

🔥 **Practice**: Focus on **1-2 patterns per week** with related problems.  
🔥 **Mock Interviews**: Solve problems **without looking at solutions** first.  
🔥 **System Design**: Learn **high-level optimizations** for real-world apps.

Would you like a **practice roadmap** with **problem sets**? 🚀
