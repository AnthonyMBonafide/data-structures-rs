# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust project implementing custom data structures from scratch for educational purposes. The library (`datastructs`) contains manual implementations of common data structures without relying on standard library collections.

## Build and Test Commands

```bash
# Build the project
cargo build

# Run all tests
cargo test

# Run tests for a specific module
cargo test --test hashmap  # for hashmap tests
cargo test --test tree     # for tree tests

# Run a specific test by name
cargo test test_hashmap_insert

# Build with verbose output
cargo build --verbose

# Check code without building
cargo check

# Format code
cargo fmt

# Run clippy for linting
cargo clippy
```

## Architecture

### Module Structure

The codebase is organized into separate modules in `src/`:
- `src/lib.rs` - Library root, re-exports modules and contains basic test functions
- `src/hashmap.rs` - Custom hashmap implementation
- `src/tree.rs` - Binary search tree implementation

### Hashmap Implementation (`src/hashmap.rs`)

The `MyHashmap<K, V>` uses **separate chaining** for collision resolution:
- Fixed-size vector of buckets (default 256)
- Each bucket is a linked list (`Bucket<K, V>`)
- Linked list nodes are `KeyValue<K, V>` structs with `Box<KeyValue<K, V>>` for next pointers
- Hash function: `DefaultHasher` from `std::hash`
- Key requirements: `PartialEq + Clone + Hash`
- Value requirements: `Clone`

Important implementation details:
- Collision handling uses a singly-linked list within each bucket
- `insert()` updates existing keys or appends to the chain
- `remove()` requires linked list traversal with previous/current node tracking
- `clear()` recreates the entire bucket vector

### Tree Implementation (`src/tree.rs`)

The `Tree<T>` is a binary search tree:
- Root node wrapped in `Option<Node<T>>`
- Each `Node<T>` has optional `left_child` and `right_child` as `Box<Node<T>>`
- Type requirements: `PartialOrd + PartialEq + Clone`
- Note: `add()` method has incomplete implementation (mutable traversal issue)
- Note: `remove()` method is a stub (not implemented)

Important implementation details:
- Left child contains values less than parent
- Right child contains values greater than parent
- `has()` method performs search traversal
- Tree structure uses immutable references in traversal logic

## Development Notes

### Known Issues and TODOs

1. **Hashmap** (`src/hashmap.rs:168`): Hash function is using `DefaultHasher` with a TODO comment to implement a real hash function
2. **Tree** (`src/tree.rs:26-29`): The `add()` method has incorrect mutability - attempts to mutate through immutable reference
3. **Tree** (`src/tree.rs:44`): The `remove()` method is unimplemented
4. **Tree** (`src/tree.rs:153`): Comment indicates tests should be replaced with fuzz tests once add functionality is complete

### Testing Strategy

- Extensive unit tests for hashmap covering insertions, collisions, duplicates, removal, and clear operations
- Minimal tree tests (only `has()` is tested currently)
- Tests use module-level `#[cfg(test)]` blocks
- Hashmap tests use various capacities to test collision handling (especially capacity=10 with 12+ items)

### Dependencies

- `array-init = "2.1.0"` - Used for array initialization (check actual usage in code)
