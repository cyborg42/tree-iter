use std::{collections::VecDeque, marker::PhantomData};

use crate::traversal_order::{BreadthFirst, DepthFirst, TraversalOrder};

/// Trait for immutable tree traversal.
///
/// This trait defines the interface required to iterate over a tree structure
/// in an immutable fashion. Any type that implements this trait can be traversed
/// using the provided iterators.
///
/// # Examples
///
/// ```rust
/// use tree_iter::prelude::*;
/// use tree_iter::tree::Node;
///
/// // Create a custom tree structure
/// struct MyTree<T> {
///     value: T,
///     children: Vec<MyTree<T>>,
/// }
///
/// // Implement TreeNode for custom tree structure
/// impl<T> TreeNode for MyTree<T> {
///     fn children(&self) -> impl DoubleEndedIterator<Item = &Self> + '_ {
///         self.children.iter()
///     }
/// }
///
/// // Now you can use the tree iterator
/// let my_tree = MyTree {
///     value: 1,
///     children: vec![
///         MyTree { value: 2, children: vec![] },
///         MyTree { value: 3, children: vec![] },
///     ],
/// };
///
/// let values: Vec<i32> = my_tree.iter::<DepthFirst>()
///                             .map(|node| node.value)
///                             .collect();
/// ```
pub trait TreeNode {
    /// Returns an iterator over the children of this node.
    ///
    /// This method must be implemented by all types implementing `TreeNode`.
    fn children(&self) -> impl DoubleEndedIterator<Item = &Self> + '_;

    /// Creates an iterator that traverses the tree starting from this node.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The traversal order strategy to use (e.g., `DepthFirst` or `BreadthFirst`).
    fn iter<T: TraversalOrder>(&self) -> TreeIter<'_, Self, T>
    where
        Self: Sized,
    {
        TreeIter::new([self])
    }
}

/// An iterator over tree nodes in a specified traversal order.
///
/// This struct provides the implementation for traversing a tree structure
/// with nodes implementing the `TreeNode` trait.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the tree nodes being traversed.
/// * `N` - The type of tree node.
/// * `T` - The traversal order strategy (e.g., `DepthFirst` or `BreadthFirst`).
pub struct TreeIter<'a, N, T> {
    /// Queue of nodes to be visited.
    nodes: VecDeque<&'a N>,
    /// Phantom data to track the traversal order type.
    _order: PhantomData<T>,
}

impl<'a, N, T: TraversalOrder> TreeIter<'a, N, T> {
    /// Creates a new tree iterator from a collection of root nodes.
    ///
    /// # Parameters
    ///
    /// * `roots` - An iterator yielding the root nodes to start traversal from.
    fn new(roots: impl IntoIterator<Item = &'a N>) -> Self {
        Self {
            nodes: roots.into_iter().collect(),
            _order: PhantomData,
        }
    }
}

/// Implementation of `Iterator` for breadth-first traversal.
impl<'a, N: TreeNode> Iterator for TreeIter<'a, N, BreadthFirst> {
    type Item = &'a N;

    /// Returns the next node in breadth-first order.
    ///
    /// Breadth-first traversal visits all nodes at a given depth before moving to the next level.
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.nodes.pop_front()?;
        self.nodes.extend(node.children());
        Some(node)
    }
}

/// Implementation of `Iterator` for depth-first traversal.
impl<'a, N: TreeNode> Iterator for TreeIter<'a, N, DepthFirst> {
    type Item = &'a N;

    /// Returns the next node in depth-first order.
    ///
    /// Depth-first traversal explores as far down a branch as possible before backtracking.
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.nodes.pop_front()?;
        for child in node.children().rev() {
            self.nodes.push_front(child);
        }
        Some(node)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    // Define a simple tree structure for testing
    struct TestTree<T> {
        value: T,
        children: Vec<TestTree<T>>,
    }

    impl<T> TestTree<T> {
        fn new(value: T) -> Self {
            Self {
                value,
                children: Vec::new(),
            }
        }

        fn with_children(value: T, children: Vec<TestTree<T>>) -> Self {
            Self { value, children }
        }
    }

    // Implement TreeNode for TestTree
    impl<T> super::TreeNode for TestTree<T> {
        fn children(&self) -> impl DoubleEndedIterator<Item = &Self> + '_ {
            self.children.iter()
        }
    }

    #[test]
    fn test_custom_tree_depth_first() {
        // Create a test tree:
        //      1
        //     / \
        //    2   3
        //   / \
        //  4   5
        let tree = TestTree::with_children(
            1,
            vec![
                TestTree::with_children(2, vec![TestTree::new(4), TestTree::new(5)]),
                TestTree::new(3),
            ],
        );

        // Collect values in depth-first order
        let values: Vec<i32> = tree.iter::<DepthFirst>().map(|node| node.value).collect();
        assert_eq!(values, vec![1, 2, 4, 5, 3]);
    }

    #[test]
    fn test_custom_tree_breadth_first() {
        // Create a test tree:
        //      1
        //     / \
        //    2   3
        //   / \
        //  4   5
        let tree = TestTree::with_children(
            1,
            vec![
                TestTree::with_children(2, vec![TestTree::new(4), TestTree::new(5)]),
                TestTree::new(3),
            ],
        );

        // Collect values in breadth-first order
        let values: Vec<i32> = tree.iter::<BreadthFirst>().map(|node| node.value).collect();
        assert_eq!(values, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_empty_custom_tree() {
        let tree = TestTree::<i32>::new(42);

        // Test depth-first traversal
        let df_values: Vec<i32> = tree.iter::<DepthFirst>().map(|node| node.value).collect();
        assert_eq!(df_values, vec![42]);

        // Test breadth-first traversal
        let bf_values: Vec<i32> = tree.iter::<BreadthFirst>().map(|node| node.value).collect();
        assert_eq!(bf_values, vec![42]);
    }

    #[test]
    fn test_forest_traversal() {
        // Test traversing a forest (multiple root nodes)
        let tree1 = TestTree::with_children(1, vec![TestTree::new(2)]);
        let tree2 = TestTree::with_children(3, vec![TestTree::new(4)]);

        // Create an iterator with multiple roots
        let forest_iter = super::TreeIter::<_, DepthFirst>::new([&tree1, &tree2]);
        let values: Vec<i32> = forest_iter.map(|node| node.value).collect();

        // Should traverse tree1 completely before starting tree2
        assert_eq!(values, vec![1, 2, 3, 4]);
    }
}
