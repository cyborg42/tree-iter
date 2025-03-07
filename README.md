# Tree-Iter

A Rust library for iterating over tree structures in different traversal orders.

## Features

- Generic support for any tree-like structure
- Breadth-first and depth-first traversal orders
- Immutable and mutable iteration
- Modifying the tree structure during mutable iteration

## Usage

### Basic Example

```rust
use tree_iter::prelude::*;
use tree_iter::tree::Node;

// Create a simple tree
let tree = Node {
    value: 1,
    children: vec![Node::new(2), Node::new(3)],
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

Mutate nodes during iteration, including modifying values and changing the structure by adding or removing children:

```rust
use tree_iter::prelude::*;
use tree_iter::tree::Node;

let mut tree = Node {
    value: 1,
    children: vec![
        Node {
            value: 2,
            children: vec![Node::new(4)],
        },
        Node::new(3),
    ],
};

// Add 10 to each value in the tree, and add a child to node with value 2
let mut iter = tree.iter_mut::<BreadthFirst>();
while let Some(mut node) = iter.next() {
    if node.value == 2 {
        node.children.push(Node::new(10));
    }
    node.value += 10;
}

// Verify values were changed in breadth-first order
let values: Vec<i32> = tree.iter::<BreadthFirst>().map(|n| n.value).collect();
assert_eq!(values, vec![11, 12, 13, 14, 20]);
```

### Forest Iteration

Iterate over a forest of trees:

```rust
use tree_iter::prelude::*;
use tree_iter::tree::Node;

// Create a forest with two trees
let mut forest = vec![
    Node {
        value: 1,
        children: vec![Node::new(2)],
    },
    Node {
        value: 3,
        children: vec![Node::new(4)],
    },
];
// Create a mutable iterator over the forest
let mut iter = TreeIterMut::<'_, _, BreadthFirst>::new(forest.iter_mut());
while let Some(mut node) = iter.next() {
    node.value += 10;
}
// Verify the modified structure
let value = TreeIter::<'_, _, BreadthFirst>::new(forest.iter())
    .map(|node| node.value)
    .collect::<Vec<_>>();
assert_eq!(value, vec![11, 13, 12, 14]);
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
    fn children(&self) -> impl DoubleEndedIterator<Item = &Self> {
        self.children.iter()
    }
}

// Implement TreeNodeMut for mutable iteration
impl<T> TreeNodeMut for MyTree<T> {
    fn children_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Self> {
        self.children.iter_mut()
    }
}

// Now you can use all iterators with your custom tree
```
