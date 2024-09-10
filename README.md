# rust_median_mode

This Rust library provides functions to calculate the median and mode of a list of integers. It includes functionality to handle both empty and populated lists, and is fully tested with unit tests.

## Features

- **Median Calculation**: Computes the median of a list of integers. The median is the value in the middle of a sorted list. If the list has an even number of elements, the median is the average of the two middle elements.
- **Mode Calculation**: Computes the mode(s) of a list of integers. The mode is the value that appears most frequently in the list.

## Installation

Add this to your Cargo.toml:

```toml
[dependencies]
rust_median_mode = "0.1.0"
```

## Usage

### 1. `list_median`

This function takes a reference to a vector of integers and returns the median value. If the list is empty, it returns `0`.

```rust
use rust_median_mode::list_median;

let numbers = vec![1, 2, 3, 4, 5];
let median = list_median(&numbers);
println!("The median is: {}", median); // Output: The median is: 3
```

### 2. list_mode

This function takes a reference to a vector of integers and returns a HashMap where the keys are the numbers in the list, and the values are the counts of how many times each number appears.

```rust
use rust_median_mode::list_mode;
use std::collections::HashMap;

let numbers = vec![1, 2, 2, 3, 4, 4, 4];
let mode = list_mode(&numbers);

for (key, value) in &mode {
    println!("Number {} appears {} times", key, value);
}
// Output:
// Number 4 appears 3 times
// Number 2 appears 2 times
// Number 1 appears 1 time

```

## Testing

Unit tests are included and can be run using:

```bash
cargo test
```





