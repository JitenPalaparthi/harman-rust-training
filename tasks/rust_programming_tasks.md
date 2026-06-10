# Rust Programming Tasks

## Topics Covered

- Variables and mutability
- Constants
- Arrays
- Slices
- Vectors
- Ownership transfer
- Borrowing
- Simple lifetimes
- Enums
- Match patterns

---

# 1. Variables and Mutability

## Task 1: Immutable Variable

Create a variable `x` with value `10` and print it.

### Requirement

```rust
let x = 10;
```

Try changing the value of `x` after declaration and observe the compiler error.

---

## Task 2: Mutable Variable

Create a mutable variable `count` with value `1`.
Increase it by `1` and print the result.

### Expected Output

```text
2
```

---

## Task 3: Shadowing

Create a variable `value` with value `5`.
Shadow it by multiplying it by `2`.
Then shadow it again by converting it into a string.

### Hint

```rust
let value = 5;
let value = value * 2;
let value = value.to_string();
```

---

# 2. Constants

## Task 4: Define a Constant

Define a constant `MAX_USERS` with value `100`.
Print the value.

### Requirement

```rust
const MAX_USERS: u32 = 100;
```

---

## Task 5: Area Using Constant

Define a constant `PI` as `3.14`.
Calculate the area of a circle using radius `5.0`.

### Formula

```text
area = PI * radius * radius
```

---

# 3. Arrays

## Task 6: Create an Array

Create an array of 5 integers and print all values.

### Example

```rust
let numbers = [10, 20, 30, 40, 50];
```

---

## Task 7: Access Array Elements

Print the first and last element of an array.

### Expected Output

```text
First: 10
Last: 50
```

---

## Task 8: Array Sum

Write a program to calculate the sum of all elements in an array.

### Example Input

```rust
let numbers = [1, 2, 3, 4, 5];
```

### Expected Output

```text
Sum = 15
```

---

# 4. Slices

## Task 9: Create a Slice

Create an array of numbers and create a slice from index `1` to `4`.

### Example

```rust
let numbers = [10, 20, 30, 40, 50];
let slice = &numbers[1..4];
```

Print the slice.

---

## Task 10: Function Taking Slice

Write a function `print_slice` that accepts a slice of integers and prints each value.

### Function Signature

```rust
fn print_slice(values: &[i32])
```

---

## Task 11: Find First Element in Slice

Write a function that takes a slice and returns the first element as an `Option<i32>`.

### Function Signature

```rust
fn first_element(values: &[i32]) -> Option<i32>
```

Use `match` to handle the result.

---

# 5. Vectors

## Task 12: Create a Vector

Create a vector of integers and push 3 values into it.

### Example

```rust
let mut numbers = Vec::new();
numbers.push(10);
numbers.push(20);
numbers.push(30);
```

Print the vector.

---

## Task 13: Iterate Over Vector

Create a vector of strings representing student names.
Print each name using a `for` loop.

---

## Task 14: Vector Sum

Write a function that accepts a vector reference and returns the sum of its values.

### Function Signature

```rust
fn sum_vector(values: &Vec<i32>) -> i32
```

---

## Task 15: Remove Element from Vector

Create a vector with values `[10, 20, 30, 40]`.
Remove the element at index `2` and print the vector.

### Expected Output

```text
[10, 20, 40]
```

---

# 6. Ownership Transfer

## Task 16: Transfer Ownership

Create a `String` variable and pass it to a function that takes ownership.
Try using the variable after the function call and observe the compiler error.

### Example

```rust
fn consume_name(name: String) {
    println!("Name: {}", name);
}
```

---

## Task 17: Return Ownership

Write a function that takes ownership of a `String` and returns it back.

### Function Signature

```rust
fn return_name(name: String) -> String
```

---

## Task 18: Ownership with Vector

Create a vector and pass it to a function that consumes it.
Then modify the function to borrow the vector instead of consuming it.

---

# 7. Borrowing

## Task 19: Immutable Borrow

Create a function that borrows a `String` and prints it without taking ownership.

### Function Signature

```rust
fn print_name(name: &String)
```

---

## Task 20: Mutable Borrow

Create a function that takes a mutable reference to a `String` and appends text to it.

### Function Signature

```rust
fn add_suffix(name: &mut String)
```

### Example

```text
Before: Rust
After: Rust Programming
```

---

## Task 21: Borrowing Rules

Write a program that demonstrates the following rule:

```text
You can have many immutable references OR one mutable reference, but not both at the same time.
```

Create examples that compile and examples that fail.

---

# 8. Simple Lifetimes

## Task 22: Function Returning Reference

Write a function that returns the longer of two string slices.

### Function Signature

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
```

---

## Task 23: Lifetime with Struct

Create a struct that stores a string slice.

### Example

```rust
struct Message<'a> {
    text: &'a str,
}
```

Create an instance and print the text.

---

## Task 24: Lifetime Error Experiment

Create a reference to a variable inside a block and try to use it outside the block.
Observe the lifetime error.

### Example

```rust
let r;
{
    let x = 10;
    r = &x;
}
println!("{}", r);
```

Explain why this fails.

---

# 9. Enums

## Task 25: Basic Enum

Create an enum `Direction` with the following variants:

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
```

Create one value and print a message using `match`.

---

## Task 26: Enum with Data

Create an enum `Message` with these variants:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

Use `match` to handle each variant.

---

## Task 27: Option Enum

Write a function that takes an index and returns an element from a vector using `Option`.

### Function Signature

```rust
fn get_value(values: &Vec<i32>, index: usize) -> Option<i32>
```

Use `match` to print either the value or `Not found`.

---

# 10. Match Patterns

## Task 28: Match Number

Write a program that matches a number and prints:

```text
1 => One
2 => Two
3 => Three
_ => Other number
```

---

## Task 29: Match Range

Write a program that checks age using match ranges.

### Rules

```text
0..=12   => Child
13..=19  => Teenager
20..=59  => Adult
60..=120 => Senior
_        => Invalid age
```

---

## Task 30: Match Tuple

Create a tuple `(x, y)` and use `match` to check:

```text
(0, 0) => Origin
(0, _) => On Y axis
(_, 0) => On X axis
(_, _) => Somewhere else
```

---

## Task 31: Match with Guards

Write a program that checks whether a number is:

```text
Positive even
Positive odd
Negative
Zero
```

Use match guards.

### Example

```rust
match number {
    n if n > 0 && n % 2 == 0 => println!("Positive even"),
    n if n > 0 => println!("Positive odd"),
    n if n < 0 => println!("Negative"),
    _ => println!("Zero"),
}
```

---

# 11. Combined Practice Tasks

## Task 32: Student Marks Using Vector and Match

Create a vector of marks.
For each mark, print grade using match ranges.

### Rules

```text
90..=100 => A
75..=89  => B
60..=74  => C
40..=59  => D
0..=39   => Fail
_        => Invalid mark
```

---

## Task 33: Command Processor Using Enum

Create an enum:

```rust
enum Command {
    Add(i32, i32),
    Subtract(i32, i32),
    Multiply(i32, i32),
    Divide(i32, i32),
    Exit,
}
```

Write a function that accepts a `Command` and uses `match` to execute it.

Handle division by zero.

---

## Task 34: Borrowing with Vector Update

Create a mutable vector of integers.
Write a function that takes `&mut Vec<i32>` and doubles each value.

### Function Signature

```rust
fn double_values(values: &mut Vec<i32>)
```

---

## Task 35: Lifetime + Enum Practice

Create an enum that stores borrowed string data.

### Example

```rust
enum UserStatus<'a> {
    Active(&'a str),
    Inactive(&'a str),
}
```

Use `match` to print the user status.

---

# Suggested Order for Students

1. Variables and constants
2. Arrays and slices
3. Vectors
4. Ownership transfer
5. Borrowing
6. Simple lifetimes
7. Enums
8. Match patterns
9. Combined practice tasks

---

# Mini Project

## Inventory Manager

Create a small inventory program using Rust.

### Requirements

- Use a `Vec` to store product names.
- Use another `Vec` to store quantities.
- Use functions to:
  - Add product
  - Remove product
  - Update quantity
  - Display all products
- Use borrowing wherever possible.
- Use `enum` for commands.
- Use `match` to process commands.
- Use slices when displaying part of the inventory.

### Example Enum

```rust
enum InventoryCommand {
    Add(String, u32),
    Remove(String),
    Update(String, u32),
    Display,
    Exit,
}
```

---

# Final Challenge

## Library Book Tracker

Build a Rust program to manage books in a library.

### Requirements

- Store book titles in a `Vec<String>`.
- Store availability status using an enum.
- Use borrowing to avoid unnecessary ownership transfer.
- Use match patterns to display book status.
- Use slices to print a range of books.
- Use a simple lifetime-based struct to represent borrowed book information.

### Example Enum

```rust
enum BookStatus {
    Available,
    Issued,
    Lost,
}
```

### Example Struct with Lifetime

```rust
struct BookInfo<'a> {
    title: &'a str,
    status: BookStatus,
}
```

---

# Submission Guidelines

For each task, students should submit:

1. Rust source code
2. Output screenshot or copied terminal output
3. Short explanation of what they learned
4. Compiler errors, if any, and how they fixed them

---

# Recommended Rust Commands

Create a new Rust project:

```bash
cargo new rust_tasks
cd rust_tasks
```

Run the program:

```bash
cargo run
```

Check code without running:

```bash
cargo check
```

Format code:

```bash
cargo fmt
```
