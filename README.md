# Tree-Iter

A Rust library for iterating over tree structures in different traversal orders.

## Features

- Generic support for any tree-like structure
- Breadth-first and depth-first traversal orders
- Immutable and mutable iteration
- Safe interior mutability during traversal using guard patterns

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tree-iter = "0.1.0"
```

## Usage

### Basic Example

```rust
use tree_iter::prelude::*;
use tree_iter::tree::Node;

// Create a simple tree
let tree = Node {
    value: 1,
    children: vec![
        Node {
            value: 2,
            children: vec![],
        },
        Node {
            value: 3,
            children: vec![],
        },
    ],
};

// Iterate over the tree in depth-first order
let values: Vec<i32> = tree.iter::<DepthFirst>()
                         .map(|node| node.value)
                         .collect();
assert_eq!(values, vec![1, 2, 3]);

// Iterate over the tree in breadth-first order
let values: Vec<i32> = tree.iter::<BreadthFirst>()
                         .map(|node| node.value)
                         .collect();
assert_eq!(values, vec![1, 2, 3]);
```

### Mutable Iteration

```rust
use tree_iter::prelude::*;
use tree_iter::tree::Node;

// Create a simple tree
let mut tree = Node {
    value: 1,
    children: vec![
        Node { value: 2, children: vec![] },
        Node { value: 3, children: vec![] },
    ],
};

// Mutably iterate and modify values
let mut iter = tree.iter_mut::<DepthFirst>();
while let Some(mut node) = iter.next() {
    node.value *= 2;
}

// Values have been doubled
assert_eq!(tree.value, 2);
```

### Custom Tree Structures

Implement the traits for your own tree structures:

```rust
use tree_iter::prelude::*;

struct MyTree<T> {
    value: T,
    children: Vec<MyTree<T>>,
}

// Implement TreeNode for immutable iteration
impl<T> TreeNode for MyTree<T> {
    fn children(&self) -> impl DoubleEndedIterator<Item = &Self> + '_ {
        self.children.iter()
    }
}

// Implement TreeNodeMut for mutable iteration
impl<T> TreeNodeMut for MyTree<T> {
    fn children_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Self> + '_ {
        self.children.iter_mut()
    }
}

// Now you can use all iterators with your custom tree
```
