# Rust Sorting Algorithms

This project involves the implementation of various sorting algorithms in Rust.

## Table of Contents

- [Rust Sorting Algorithms](#rust-sorting-algorithms)
  - [Table of Contents](#table-of-contents)
  - [Background](#background)
  - [Algorithms Included](#algorithms-included)
  - [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
  - [Installing](#installing)
  - [Running](#running)
  - [Testing](#testing)
  - [License](#license)
  - [Contribute](#contribute)
  - [Disclaimer](#disclaimer)

## Background

[Sorting algorithms](https://en.wikipedia.org/wiki/Sorting_algorithm) are an essential part of computer science. They are used to rearrange a sequence of elements in a specific order. There are many different sorting algorithms, each with its own strengths and weaknesses. Some sorting algorithms are more efficient than others, depending on the size of the input data and the distribution of the elements.

This project aims to implement a variety of sorting algorithms in Rust. Rust is a systems programming language that is known for its performance, safety, and concurrency features. By implementing sorting algorithms in Rust, we can learn more about the language and how it can be used to solve complex problems.

## Algorithms Included

✅ = Working
❌ = Not working
⏰ = Not started

| Algorithm            | Status | Type                 | Learn more                                                                       |
| -------------------- | ------ | -------------------- | -------------------------------------------------------------------------------- |
| Bitonic sort         | ❌     |                      | [Link](https://en.wikipedia.org/wiki/Bitonic_sorter)                             |
| Block sort           | ⏰     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/Block_sort)                                 |
| Bogo sort            | ✅     |                      | [Link](https://en.wikipedia.org/wiki/Bogosort)                                   |
| Bubble sort          | ✅     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/Bubble_sort)                                |
| Burst sort           | ⏰     |                      | [Link](https://en.wikipedia.org/wiki/Burstsort)                                  |
| Cocktail shaker sort | ✅     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/Cocktail_shaker_sort)                       |
| Comb sort            | ✅     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/Comb_sort)                                  |
| Counting sort        | ✅     |                      | [Link](https://en.wikipedia.org/wiki/Counting_sort)                              |
| Cubesort             | ⏰     |                      | [Link](https://en.wikipedia.org/wiki/Cubesort)                                   |
| Exchange sort        | ⏰     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/Exchange_sort)                              |
| Cycle sort           | ❌     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/Cycle_sort)                                 |
| Flash sort           | ⏰     |                      | [Link](https://en.wikipedia.org/wiki/Flashsort)                                  |
| Gnome sort           | ✅     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/Gnome_sort)                                 |
| Heap sort            | ✅     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/Heapsort)                                   |
| Introsort            | ⏰     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/Introsort)                                  |
| In place merge sort  | ⏰     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/In-place_merge_sort)                        |
| Tournament sort      | ⏰     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/Tournament_sort)                            |
| Insertion sort       | ✅     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/Insertion_sort)                             |
| Library sort         | ❌     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/Library_sort)                               |
| Merge sort           | ✅     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/Merge_sort)                                 |
| Pancake sort         | ✅     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/Pancake_sorting)                            |
| Patience sorting     | ❌     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/Patience_sorting)                           |
| Postman sort         | ⏰     |                      | [Link](https://en.wikipedia.org/wiki/Postman_sort)                               |
| Quick sort           | ✅     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/Quicksort)                                  |
| Radix sort           | ✅     |                      | [Link](https://en.wikipedia.org/wiki/Radix_sort)                                 |
| Red-Black Tree sort  | ✅     |                      | [Link](https://en.wikipedia.org/wiki/Red%E2%80%93black_tree)                     |
| Selection sort       | ✅     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/Selection_sort)                             |
| Shell sort           | ✅     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/Shellsort)                                  |
| Sleep sort           | ✅     |                      | [Link](https://www.geeksforgeeks.org/sleep-sort-king-laziness-sorting-sleeping/) |
| Smooth sort          | ❌     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/Smoothsort)                                 |
| Stooge sort          | ✅     |                      | [Link](https://en.wikipedia.org/wiki/Stooge_sort)                                |
| Strand sort          | ✅     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/Strand_sort)                                |
| Tim sort             | ✅     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/Timsort)                                    |
| Tree sort            | ✅     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/Tree_sort)                                  |
| Odd-even sort        | ⏰     | Comparison sorts     | [Link](https://en.wikipedia.org/wiki/Odd%E2%80%93even_sort)                      |
| Pigeonhole sort      | ⏰     | Non-comparison sorts | [Link](https://en.wikipedia.org/wiki/Pigeonhole_sort)                            |

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

You need to have Rust installed on your machine. If you don't have Rust installed, you can install it by following these steps:

1. Download and install `rustup` by following the instructions at [https://rustup.rs/](https://rustup.rs/).

2. Once `rustup` is installed, open a new terminal and run the following command to install the Rust compiler:

```bash
rustup install stable
```

3. Add the cargo binary to your PATH with the following command:

```bash
source $HOME/.cargo/env
```

## Installing

1. Clone the repository

```bash
git clone https://github.com/arvid-berndtsson/Rust-Sorting-Algorithms.git
```

2. Navigate into the cloned repository

```bash
cd rust-sorting-algorithms
```

3. Build the project

```bash
cargo build
```

## Running

You can run the project with:

```bash
cargo run
```

## Testing

You can run the tests with:

```bash
cargo test
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contribute

If you would like to contribute to this project, please open an issue or a pull request. All contributions are welcome!

## Disclaimer

This project is for educational purposes. I do not recommend using these sorting algorithms in production code. Rust has built-in sorting functions that are more efficient and reliable than the algorithms implemented in this project.

I made this project to learn more about Rust and sorting algorithms. I hope you find it useful and informative!
