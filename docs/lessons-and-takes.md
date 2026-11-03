# Rust Lessons and takes

## Stage 1: Rust fundamentals

- Installation and tooling
-

## Chapter 4: Understanding ownership

Rust does not have a Garbage collector but instead use the 'Ownership' concept to manage heap memory.

### Stack vs Heap

- **Stack**: Store values that whose is defined compile time and do changes in size, e.g Integers nd Booleans.
- **Heap**: Stores values whose size is not define is not defined or changes at runtime, e.g dynamic linked list, strings.

*Stack memory is fast than the heap because the CPU can easily access and manage adjacent memory. While the heap requires the allocator to find memory in the within the heap that best meets the require amount and also keep tractor of the owner*

### Ownership rules

- Every value has an owner.
- There can only be one owner per value.
- The value is dropped from the heap when the owner goes out of scope.

### Memory and allocation

- Assigning two string variables to the same value created two pointers that point to the same memory address in the heap. Example:
 `
let s1 = String::from("hello");
 let s2 = s1;`
 `
 s1 is dropped from and s2 is the new owner the value "hello"

### References and Borrowing

You can variables by reference if you want to use their value without taking ownership.

- References and immutable by default.
- To make references mutable, use the keyword `mut`. Example `fn test_function(s: &mut String)` will take a mutable reference to string.
- A mutable reference can only be borrowed once by another variable.
-  We *also* cannot have a mutable reference while we have an immutable one to the same value.

### [The Rules of References](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#the-rules-of-references)

- At any given time, you can have *either* one mutable reference *or* any number of immutable references.
- References must always be valid.

### Summary

The concepts of ownership, borrowing and slices ensure memory safety in Rust programs at compile time while still allowing you to work closer to the memory and not need manage memory allocation and deallocation yourself.

## Chapter 5 Using struct to structure related data

Structs are used to group related data similar to tuples. But unlike tuples you have to provide the name of the struct that best represents the data that is being grouped.

- Struct fields must have types when defining the struct.
- Struct fields are accessed using the dot operator.
- Derived traits like `#[derive(Debug)]` can help use print the contents of the struct using `println!("{example_struct:?}")` or the dbg! macro e.g `dbg!(&example_struct);`.

**Example of struct definition**:

```rust
struct Rectangle {
 width: u32,
 height: u32,
}
```

**Struct methods**:

```rust
struct Rectangle {
 width: u32,
 height: u32,
}

impl Rectangle {
 fn area(&self) -> u32 {
  self.width * self.height
 }
 
 fn can_hold(&self, other: &Rectangle) -> bool {
  self.width > other.width && self.height > other.height
 }
}
```

## Chapter 6: Enums and pattern matching

- Enums help create custom types that can be one of a set of enumerated values.
- Standard library's `Option<T>` type helps you use the type system to prevent errors.
- When Enum values have data, you can `match` or `if let` to extract those values.

## Chapter 8 Common Collections

Rust has three basic collections that are stored in the heap memory.

- **Vectors**: Store a list related data of the same type.
- **Strings**: Stores a list of characters (bytes) that represent UTF-8 encoded strings.
- **Hash map**: Store a list of key-value pairs.

### Notes

- Strings don't allow accessing single characters using the index notation. This is because UTF-8 encoded strings length does not always correspond to the number of characters, especially on for scripts like, kanji, Cyrillic and Hindi language.
- Vector value can be accessed with the indices notation. But the preferred way is the `get` method to returns `Option<&T>` which can be used to handle index out range exception through the `match`.
- To check if a key-value pair already exist in hash map and if does then add a new key-value pair use the `entry` method combined with `or_insert`.
  Example:

  - ```rust

   use std::collections:HashMap;

   let map = HashMap::new();
   map.insert("one", 1);
   map.insert("two", 2);

   map.entry("two").or_insert(0); // skips because key already exists.
   map.entry("three").or_insert(3); // add new entry to map.

   ```

### Chapter 9: Error Handling

Rust provides a number of features to handle situations in which something goes wrong. Rust groups errors into two major categories: Recoverable and unrecoverable errors and the mechanisms to handle these type of errors are `Result<T,E>` and `panic!` macro.

- **Result<T,E>**: used to handle errors code can graciously recover from. Example, failing to open a file, code can recover by show a message.
- **panic!**: This macro is used in situations when code encounters errors it cannot not safely recover from or may lead to security rist. Example is when code tries to access an index that is out of bound in an array or vector.

---

## Chapter 13 Functional Language feature: Iterators and Closures

Rust's iterators and closures are inspired by functional programming laguage ideas.

- **Closures**: Are anonymous functions that can be taken as arguments by other functions. They use double vertical pipes `||`  to accept arguments.  For example `|x| x + 1`  where  x is argument passed to the closure.
- **Iterators**: They allow traversal of the collections and manipulation of each item in the collection.

## Chapter 14 More about Cargo and Crates.io

Sharing code with Cargo and crates.io is part of what makes the Rust ecosystem useful for many different tasks. Rust’s standard library is small and stable, but crates are easy to share, use, and improve on a timeline different from that of the language. Don’t be shy about sharing code that’s useful to you on crates.io; it’s likely that it will be useful to someone else as well!

### Lessons

- Cargo can do more that just build, run and test.
- Cargo can be using to manage workspaces on large projects.
- You can install binary crates on your local machine using cargo. These binary crates will be installed in the $PATH under `.cargo/bin/` for example installing ripgrep will be stored in `~/.cargo/bin/rg`.
- Crates.io is the public inventory when Rust developers publish their crates. Think DockerHub for Docker.
- You can publish your crates to Crates.io using cargo.
- To publish a crates you need to provide the following on Cargo.toml under `[package]` name, version, license.

### Takes

- Cargo is more that just a build tool.
- It makes it convenient to packages and publish Rust projects.
- Crates.io provides a central places for finding useful crates when building a project.

### Questions

- Does `cargo install` only work for binary crates build with Rust?
