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
///     children: vec![
///         Node {
///             value: 2,
///             children: vec![],
///         },
///         Node {
///             value: 3,
///             children: vec![],
///         },
///     ],
/// };
/// 
/// // Traverse in depth-first order
/// let values: Vec<i32> = tree.iter::<DepthFirst>()
///                           .map(|node| node.value)
///                           .collect();
/// assert_eq!(values, vec![1, 2, 3]);
/// ```
pub struct Node<T> {
    /// The value stored in this node.
    pub value: T,
    /// The children of this node.
    pub children: Vec<Node<T>>,
}

/// Implementation of `TreeNode` for `Node<T>`.
/// 
/// This allows immutable iteration over the tree.
impl<T> TreeNode for Node<T> {
    /// Returns an iterator over the children of this node.
    fn children(&self) -> impl DoubleEndedIterator<Item = &Self> + '_ {
        self.children.iter()
    }
}

/// Implementation of `TreeNodeMut` for `Node<T>`.
/// 
/// This allows mutable iteration over the tree.
impl<T> TreeNodeMut for Node<T> {
    /// Returns a mutable iterator over the children of this node.
    fn children_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Self> + '_ {
        self.children.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_empty_tree() {
        let tree: Node<i32> = Node {
            value: 42,
            children: vec![],
        };

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
                    children: vec![
                        Node {
                            value: 4,
                            children: vec![],
                        },
                        Node {
                            value: 5,
                            children: vec![],
                        },
                    ],
                },
                Node {
                    value: 3,
                    children: vec![
                        Node {
                            value: 6,
                            children: vec![],
                        },
                    ],
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
                    children: vec![
                        Node {
                            value: 4,
                            children: vec![],
                        },
                        Node {
                            value: 5,
                            children: vec![],
                        },
                    ],
                },
                Node {
                    value: 3,
                    children: vec![
                        Node {
                            value: 6,
                            children: vec![],
                        },
                    ],
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
                    children: vec![
                        Node {
                            value: 4,
                            children: vec![],
                        },
                    ],
                },
                Node {
                    value: 3,
                    children: vec![],
                },
            ],
        };

        // Add 10 to each value in the tree
        let mut iter = tree.iter_mut::<BreadthFirst>();
        while let Some(mut node) = iter.next() {
            node.value += 10;
        }

        // Verify values were changed in breadth-first order
        let values: Vec<i32> = tree.iter::<BreadthFirst>().map(|n| n.value).collect();
        assert_eq!(values, vec![11, 12, 13, 14]);
    }

    #[test]
    fn test_tree_modification() {
        // Create a tree manually with specific structure
        let mut tree = Node {
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
        
        // Verify initial structure
        let initial_values: Vec<i32> = tree.iter::<DepthFirst>().map(|n| n.value).collect();
        assert_eq!(initial_values, vec![1, 2, 3]);
        
        // Modify the tree directly, avoiding iterator with borrow issues
        tree.children[0].children.push(Node {
            value: 20,
            children: vec![],
        });
        
        tree.children[1].children.push(Node {
            value: 30,
            children: vec![],
        });
        
        // Verify the modified structure
        let final_values: Vec<i32> = tree.iter::<DepthFirst>().map(|n| n.value).collect();
        assert_eq!(final_values, vec![1, 2, 20, 3, 30]);
    }
    
    #[test]
    fn test_complex_tree_traversal() {
        // Create a more complex tree
        let tree = Node {
            value: 1,
            children: vec![
                Node {
                    value: 2,
                    children: vec![
                        Node {
                            value: 4,
                            children: vec![
                                Node {
                                    value: 7,
                                    children: vec![],
                                },
                            ],
                        },
                        Node {
                            value: 5,
                            children: vec![],
                        },
                    ],
                },
                Node {
                    value: 3,
                    children: vec![
                        Node {
                            value: 6,
                            children: vec![
                                Node {
                                    value: 8,
                                    children: vec![],
                                },
                                Node {
                                    value: 9,
                                    children: vec![],
                                },
                            ],
                        },
                    ],
                },
            ],
        };
        
        // Test depth-first traversal
        let df_values: Vec<i32> = tree.iter::<DepthFirst>().map(|n| n.value).collect();
        assert_eq!(df_values, vec![1, 2, 4, 7, 5, 3, 6, 8, 9]);
        
        // Test breadth-first traversal
        let bf_values: Vec<i32> = tree.iter::<BreadthFirst>().map(|n| n.value).collect();
        assert_eq!(bf_values, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
