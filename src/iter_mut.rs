use std::{
    collections::VecDeque,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::traversal_order::{BreadthFirst, DepthFirst, TraversalOrder};

/// Trait for mutable tree traversal.
///
/// This trait defines the interface required to iterate over a tree structure
/// with mutable access to the nodes. Any type that implements this trait can be
/// traversed using the provided mutable iterators.
///
/// Implementing this trait requires providing a way to access the children of a node
/// mutably, which enables the iterator to traverse the tree structure.
///
/// # Examples
///
/// ```rust
/// use tree_iter::prelude::*;
/// use tree_iter::tree::Node;
///
/// // Create a simple tree
/// let mut tree = Node {
///     value: 1,
///     children: vec![
///         Node { value: 2, children: vec![] },
///         Node { value: 3, children: vec![] },
///     ],
/// };
///
/// // Mutably iterate and modify values
/// let mut iter = tree.iter_mut::<DepthFirst>();
/// while let Some(mut node) = iter.next() {
///     node.value *= 2;
/// }
///
/// // Values have been doubled
/// assert_eq!(tree.value, 2);
/// ```
pub trait TreeNodeMut {
    /// Returns a mutable iterator over the children of this node.
    ///
    /// This method must be implemented by all types implementing `TreeNodeMut`.
    fn children_mut(&mut self) -> impl DoubleEndedIterator<Item = &mut Self>;

    /// Creates a mutable iterator that traverses the tree starting from this node.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The traversal order strategy to use (e.g., `DepthFirst` or `BreadthFirst`).
    fn iter_mut<T: TraversalOrder>(&mut self) -> TreeMutIter<'_, Self, T>
    where
        Self: Sized,
    {
        TreeMutIter::new([self])
    }
}

/// A mutable iterator over tree nodes in a specified traversal order.
///
/// This struct provides the implementation for traversing a tree structure
/// with mutable access to nodes implementing the `TreeNodeMut` trait.
///
/// # Type Parameters
///
/// * `'a` - The lifetime of the tree nodes being traversed.
/// * `N` - The type of tree node.
/// * `T` - The traversal order strategy (e.g., `DepthFirst` or `BreadthFirst`).
#[derive(Debug)]
pub struct TreeMutIter<'a, N, T> {
    /// Queue of nodes to be visited.
    nodes: VecDeque<&'a mut N>,
    /// Phantom data to track the traversal order type.
    _order: PhantomData<T>,
}

impl<'a, N, T> TreeMutIter<'a, N, T> {
    /// Creates a new mutable tree iterator from a collection of root nodes.
    ///
    /// # Parameters
    ///
    /// * `roots` - An iterator yielding mutable references to the root nodes to start traversal from.
    pub fn new(roots: impl IntoIterator<Item = &'a mut N>) -> Self {
        Self {
            nodes: roots.into_iter().collect(),
            _order: PhantomData,
        }
    }
}

/// A guard for mutable node references in breadth-first traversal.
///
/// This guard ensures that when a mutable reference is dropped, the node's children
/// are correctly added to the traversal queue in breadth-first order.
///
/// The guard implements `Deref` and `DerefMut` to allow direct access to the underlying node.
pub struct MutRefBFSGuard<'a: 'b, 'b, N: TreeNodeMut> {
    /// Reference to the tree iterator.
    iter: &'b mut TreeMutIter<'a, N, BreadthFirst>,
    /// The current node being visited (wrapped in Option to allow taking ownership in drop).
    node: Option<&'a mut N>,
}

impl<N: TreeNodeMut> Drop for MutRefBFSGuard<'_, '_, N> {
    /// When the guard is dropped, add the node's children to the traversal queue.
    ///
    /// This is where the breadth-first traversal logic is implemented - children are
    /// added to the back of the queue to be processed after all nodes at the current level.
    fn drop(&mut self) {
        let node = self.node.take().unwrap();
        self.iter.nodes.extend(node.children_mut());
    }
}

impl<N: TreeNodeMut> Deref for MutRefBFSGuard<'_, '_, N> {
    type Target = N;

    /// Provides immutable access to the wrapped node.
    fn deref(&self) -> &Self::Target {
        self.node.as_ref().unwrap()
    }
}

impl<N: TreeNodeMut> DerefMut for MutRefBFSGuard<'_, '_, N> {
    /// Provides mutable access to the wrapped node.
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.node.as_mut().unwrap()
    }
}

impl<'a, N: TreeNodeMut> TreeMutIter<'a, N, BreadthFirst> {
    /// Returns the next node in breadth-first order.
    ///
    /// This method returns a guard that implements `DerefMut` to provide
    /// mutable access to the node. When the guard is dropped, the node's children
    /// are added to the traversal queue in breadth-first order.
    pub fn next<'b>(&'b mut self) -> Option<MutRefBFSGuard<'a, 'b, N>> {
        self.nodes.pop_front().map(|node| MutRefBFSGuard {
            iter: self,
            node: Some(node),
        })
    }
}

/// A guard for mutable node references in depth-first traversal.
///
/// This guard ensures that when a mutable reference is dropped, the node's children
/// are correctly added to the traversal queue in depth-first order.
///
/// The guard implements `Deref` and `DerefMut` to allow direct access to the underlying node.
pub struct MutRefDFSGuard<'a: 'b, 'b, N: TreeNodeMut> {
    /// Reference to the tree iterator.
    iter: &'b mut TreeMutIter<'a, N, DepthFirst>,
    /// The current node being visited (wrapped in Option to allow taking ownership in drop).
    node: Option<&'a mut N>,
}

impl<N: TreeNodeMut> Drop for MutRefDFSGuard<'_, '_, N> {
    /// When the guard is dropped, add the node's children to the traversal queue.
    ///
    /// This is where the depth-first traversal logic is implemented - children are
    /// added to the front of the queue in reverse order to ensure the leftmost child
    /// is processed first.
    fn drop(&mut self) {
        let node = self.node.take().unwrap();
        for child in node.children_mut().rev() {
            self.iter.nodes.push_front(child);
        }
    }
}

impl<N: TreeNodeMut> Deref for MutRefDFSGuard<'_, '_, N> {
    type Target = N;

    /// Provides immutable access to the wrapped node.
    fn deref(&self) -> &Self::Target {
        self.node.as_ref().unwrap()
    }
}

impl<N: TreeNodeMut> DerefMut for MutRefDFSGuard<'_, '_, N> {
    /// Provides mutable access to the wrapped node.
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.node.as_mut().unwrap()
    }
}

impl<'a, N: TreeNodeMut> TreeMutIter<'a, N, DepthFirst> {
    /// Returns the next node in depth-first order.
    ///
    /// This method returns a guard that implements `DerefMut` to provide
    /// mutable access to the node. When the guard is dropped, the node's children
    /// are added to the traversal queue in depth-first order.
    pub fn next<'b>(&'b mut self) -> Option<MutRefDFSGuard<'a, 'b, N>> {
        self.nodes.pop_front().map(|node| MutRefDFSGuard {
            iter: self,
            node: Some(node),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use crate::tree::Node;

    #[test]
    fn test_mutable_depth_first_traversal() {
        // Create a test tree:
        //      1
        //     / \
        //    2   3
        let mut tree = Node {
            value: 1,
            children: vec![Node::new(2), Node::new(3)],
        };

        // Double each value using mutable depth-first traversal
        {
            let mut iter = tree.iter_mut::<DepthFirst>();
            while let Some(mut node) = iter.next() {
                node.value = node.value * 2;
            }
        }

        // Verify the values were modified correctly
        let values: Vec<i32> = tree.iter::<DepthFirst>().map(|node| node.value).collect();
        assert_eq!(values, vec![2, 4, 6]);
    }

    #[test]
    fn test_mutable_breadth_first_traversal() {
        // Create a test tree:
        //      1
        //     / \
        //    2   3
        //   /
        //  4
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

        // Add 10 to each value using mutable breadth-first traversal
        {
            let mut iter = tree.iter_mut::<BreadthFirst>();
            while let Some(mut node) = iter.next() {
                node.value += 10;
            }
        }

        // Verify the values were modified correctly
        let values: Vec<i32> = tree.iter::<BreadthFirst>().map(|node| node.value).collect();
        assert_eq!(values, vec![11, 12, 13, 14]);
    }

    #[test]
    fn test_forest_mutable_traversal() {
        // Create two trees
        let mut tree1 = Node {
            value: 1,
            children: vec![Node::new(2)],
        };
        let mut tree2 = Node {
            value: 3,
            children: vec![Node::new(4)],
        };

        // Create a vector to hold mutable references
        let mut roots = Vec::new();
        roots.push(&mut tree1);
        roots.push(&mut tree2);

        // Create a mutable iterator with multiple roots - modify one at a time
        for tree in &mut roots {
            let mut iter = tree.iter_mut::<DepthFirst>();
            while let Some(mut node) = iter.next() {
                node.value += 100;
            }
        }

        // Verify the values were modified
        let values1: Vec<i32> = tree1.iter::<DepthFirst>().map(|node| node.value).collect();
        let values2: Vec<i32> = tree2.iter::<DepthFirst>().map(|node| node.value).collect();

        assert_eq!(values1, vec![101, 102]);
        assert_eq!(values2, vec![103, 104]);
    }

    #[test]
    fn test_adding_children_during_traversal() {
        // Create a simple tree
        let mut tree = Node::new(1);

        // Add children during traversal
        {
            let mut iter = tree.iter_mut::<DepthFirst>();
            while let Some(mut node) = iter.next() {
                // Only add children to the root node
                if node.value == 1 {
                    node.children.push(Node::new(2));
                    node.children.push(Node::new(3));
                }
            }
        }

        // Verify the new structure
        let values: Vec<i32> = tree.iter::<DepthFirst>().map(|node| node.value).collect();
        assert_eq!(values, vec![1, 2, 3]);
    }
}
