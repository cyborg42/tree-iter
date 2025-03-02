use crate::{iter::TreeNode, prelude::TreeNodeMut};

/// A generic tree node implementation.
///
/// This struct provides a simple implementation of a tree node with a value and a vector of children.
/// It implements both `TreeNode` and `TreeNodeMut` traits, allowing it to be used with both
/// immutable and mutable iterators.
///
/// # Type Parameters
///
/// * `T` - The type of value stored in each node.
///
/// # Examples
///
/// ```rust
/// use tree_iter::tree::Node;
/// use tree_iter::prelude::*;
///
/// // Create a simple tree
/// let tree = Node {
///     value: 1,
///     children: vec![Node::new(2), Node::new(3)],
/// };
///
/// // Traverse in depth-first order
/// let values: Vec<i32> = tree.iter::<DepthFirst>()
///                           .map(|node| node.value)
///                           .collect();
/// assert_eq!(values, vec![1, 2, 3]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default, Hash)]
pub struct Node<T> {
    /// The value stored in this node.
    pub value: T,
    /// The children of this node.
    pub children: Vec<Node<T>>,
}

impl<T> Node<T> {
    /// Creates a new node with the given value and no children.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to store in the node.
    ///
    /// # Returns
    ///
    /// A new `Node` with the given value and an empty children vector.
    pub fn new(value: T) -> Self {
        Self {
            value,
            children: Vec::new(),
        }
    }
}

/// Implementation of `TreeNode` for `Node<T>`.
///
/// This allows immutable iteration over the tree.
impl<T> TreeNode for Node<T> {
    /// Returns an iterator over the children of this node.
    fn children(&self) -> impl DoubleEndedIterator<Item = &Self> {
        self.children.iter()
    }
}

/// Implementation of `TreeNodeMut` for `Node<T>`.
///
/// This allows mutable iteration over the tree.
impl<T> TreeNodeMut for Node<T> {
    /// Returns a mutable iterator over the children of this node.
    fn children_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Self> {
        self.children.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_empty_tree() {
        let tree: Node<i32> = Node::new(42);
        let values: Vec<i32> = tree.iter::<DepthFirst>().map(|n| n.value).collect();
        assert_eq!(values, vec![42]);

        let values: Vec<i32> = tree.iter::<BreadthFirst>().map(|n| n.value).collect();
        assert_eq!(values, vec![42]);
    }

    #[test]
    fn test_depth_first_traversal() {
        let tree = Node {
            value: 1,
            children: vec![
                Node {
                    value: 2,
                    children: vec![Node::new(4), Node::new(5)],
                },
                Node {
                    value: 3,
                    children: vec![Node::new(6)],
                },
            ],
        };

        let values: Vec<i32> = tree.iter::<DepthFirst>().map(|n| n.value).collect();
        assert_eq!(values, vec![1, 2, 4, 5, 3, 6]);
    }

    #[test]
    fn test_breadth_first_traversal() {
        let tree = Node {
            value: 1,
            children: vec![
                Node {
                    value: 2,
                    children: vec![Node::new(4), Node::new(5)],
                },
                Node {
                    value: 3,
                    children: vec![Node::new(6)],
                },
            ],
        };

        let values: Vec<i32> = tree.iter::<BreadthFirst>().map(|n| n.value).collect();
        assert_eq!(values, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_mutable_depth_first_traversal() {
        let mut tree = Node {
            value: 1,
            children: vec![Node::new(2), Node::new(3)],
        };

        // Double each value in the tree
        let mut iter = tree.iter_mut::<DepthFirst>();
        while let Some(mut node) = iter.next() {
            node.value *= 2;
        }

        // Verify values were changed
        let values: Vec<i32> = tree.iter::<DepthFirst>().map(|n| n.value).collect();
        assert_eq!(values, vec![2, 4, 6]);
    }

    #[test]
    fn test_mutable_breadth_first_traversal() {
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
    }

    #[test]
    fn test_tree_modification() {
        // Create a tree manually with specific structure
        let mut tree = Node {
            value: 1,
            children: vec![Node::new(2), Node::new(3)],
        };

        // Verify initial structure
        let initial_values: Vec<i32> = tree.iter::<DepthFirst>().map(|n| n.value).collect();
        assert_eq!(initial_values, vec![1, 2, 3]);

        tree.children[0].children.push(Node::new(20));
        tree.children[1].children.push(Node::new(30));

        // Verify the modified structure
        let final_values: Vec<i32> = tree.iter::<DepthFirst>().map(|n| n.value).collect();
        assert_eq!(final_values, vec![1, 2, 20, 3, 30]);
    }

    #[test]
    fn test_forest_traversal() {
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
        let mut iter = TreeMutIter::<'_, _, BreadthFirst>::new(forest.iter_mut());
        while let Some(mut node) = iter.next() {
            node.value += 10;
        }
        // Verify the modified structure
        let value = TreeIter::<'_, _, BreadthFirst>::new(forest.iter())
            .map(|node| node.value)
            .collect::<Vec<_>>();
        assert_eq!(value, vec![11, 13, 12, 14]);
    }

    #[test]
    fn test_complex_tree_traversal() {
        // Create a more complex tree
        let mut tree = Node {
            value: 1,
            children: vec![
                Node {
                    value: 2,
                    children: vec![
                        Node {
                            value: 4,
                            children: vec![Node::new(7)],
                        },
                        Node::new(5),
                    ],
                },
                Node {
                    value: 3,
                    children: vec![Node {
                        value: 6,
                        children: vec![Node::new(8), Node::new(9)],
                    }],
                },
            ],
        };

        // Test depth-first traversal
        let df_values: Vec<i32> = tree.iter::<DepthFirst>().map(|n| n.value).collect();
        assert_eq!(df_values, vec![1, 2, 4, 7, 5, 3, 6, 8, 9]);

        // Test mutable depth-first traversal
        let mut df_values_mut = vec![];
        let mut iter = tree.iter_mut::<DepthFirst>();
        while let Some(node) = iter.next() {
            df_values_mut.push(node.value);
        }
        assert_eq!(df_values_mut, vec![1, 2, 4, 7, 5, 3, 6, 8, 9]);

        // Test breadth-first traversal
        let bf_values: Vec<i32> = tree.iter::<BreadthFirst>().map(|n| n.value).collect();
        assert_eq!(bf_values, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

        // Test mutable breadth-first traversal
        let mut bf_values_mut = vec![];
        let mut iter = tree.iter_mut::<BreadthFirst>();
        while let Some(node) = iter.next() {
            bf_values_mut.push(node.value);
        }
        assert_eq!(bf_values_mut, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
