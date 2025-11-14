# 🦀 Algorithms in Rust

A comprehensive collection of **algorithms and data structures** implemented in **Rust** — focusing on performance, clarity, and idiomatic Rust design.

## 🚀 Overview

This project is a personal and educational journey to explore **core computer science algorithms** using **Rust**.
It covers topics like sorting, searching, graph theory, dynamic programming, data structures, and more — with an emphasis on:

- Clean and modular Rust code
- Well-documented implementations
- Benchmarking and performance comparisons
- Real-world examples where applicable

Whether you’re learning Rust, brushing up on algorithms, or preparing for interviews — this repository aims to be a **go-to reference**.

---

## 📂 Project Structure

```bash
algorithms-in-rust/
├── src/
│   ├── sorting/
│   │   ├── bubble_sort.rs
│   │   ├── quick_sort.rs
│   │   └── merge_sort.rs
│   ├── searching/
│   │   ├── binary_search.rs
│   │   └── linear_search.rs
│   ├── graphs/
│   │   ├── dijkstra.rs
│   │   └── bfs.rs
│   ├── data_structures/
│   │   ├── stack.rs
│   │   ├── queue.rs
│   │   └── linked_list.rs
│   └── main.rs
├── Cargo.toml
└── README.md
```

Each algorithm lives in its own module for modularity and readability.

---

## 🧠 Topics Covered (Planned & Implemented)

- **Sorting Algorithms**

  - [x] Bubble Sort
  - [x] Merge Sort
  - [ ] Quick Sort
  - [ ] Heap Sort

- **Searching Algorithms**

  - [x] Linear Search
  - [x] Binary Search

- **Graph Algorithms**

  - [ ] Breadth-First Search (BFS)
  - [ ] Depth-First Search (DFS)
  - [ ] Dijkstra’s Algorithm

- **Dynamic Programming**

  - [ ] Fibonacci (Top-Down & Bottom-Up)
  - [ ] Longest Common Subsequence

- **Data Structures**

  - [x] Stack
  - [x] Queue
  - [ ] Linked List
  - [ ] Binary Tree

---

## 🦾 Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (1.70+ recommended)
- Cargo (comes with Rust)

### Build and Run

```bash
# Clone the repository
git clone https://github.com/<your-username>/algorithms-in-rust.git
cd algorithms-in-rust

# Run all examples
cargo run

# Run specific algorithm
cargo run --example quick_sort
```

### Run Tests

```bash
cargo test
```

---

## 📊 Benchmarks

Benchmarks will be added for comparing algorithmic performance using [`criterion`](https://crates.io/crates/criterion).

Example:

```bash
cargo bench
```

---

## 🧩 Contributing

Contributions are welcome!
If you’d like to improve existing algorithms, add new ones, or optimize code:

1. Fork this repo
2. Create a feature branch
3. Submit a pull request

Please follow idiomatic Rust patterns and add unit tests.

---

## 📖 References

- [The Rust Programming Language (Book)](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [CLRS — Introduction to Algorithms](https://mitpress.mit.edu/9780262046305/introduction-to-algorithms/)

---

## 🧑‍💻 Author

**Sankar**
Software Developer • Rust Enthusiast • Algorithm Explorer
📬 [GitHub](https://github.com/<your-username>)

---

## 🪶 License

This project is licensed under the [MIT License](LICENSE).

---
