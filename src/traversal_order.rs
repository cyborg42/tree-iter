/// Marker struct representing breadth-first traversal order.
/// 
/// Breadth-first traversal visits all nodes at the same depth level before moving to the next level.
/// This is implemented using a queue where children are added to the back of the queue.
pub struct BreadthFirst;

/// Marker struct representing depth-first traversal order.
/// 
/// Depth-first traversal explores as far down a branch as possible before backtracking.
/// This is implemented by adding children to the front of the queue or using a stack.
pub struct DepthFirst;

/// Private module to implement the sealed trait pattern.
/// 
/// This prevents external crates from implementing the `TraversalOrder` trait,
/// allowing us to maintain control over the possible traversal orders.
mod seal {
    pub trait Sealed {}
    impl Sealed for super::DepthFirst {}
    impl Sealed for super::BreadthFirst {}
}

/// Trait for tree traversal order strategies.
/// 
/// This trait is sealed (cannot be implemented outside this crate) and is used as
/// a marker for different traversal strategies that can be used with tree iterators.
pub trait TraversalOrder: seal::Sealed {}
impl TraversalOrder for BreadthFirst {}
impl TraversalOrder for DepthFirst {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::iter::TreeNode;
    use std::collections::VecDeque;
    use std::marker::PhantomData;

    // Simple test structure to verify trait bounds
    #[allow(dead_code)]
    struct TestNode<T> {
        value: T,
        children: Vec<TestNode<T>>,
    }

    impl<T> TreeNode for TestNode<T> {
        fn children(&self) -> impl DoubleEndedIterator<Item = &Self> + '_ {
            self.children.iter()
        }
    }

    // Basic struct to test traversal order trait bounds
    #[allow(dead_code)]
    struct TestTreeIter<'a, N, T> {
        nodes: VecDeque<&'a N>,
        _order: PhantomData<T>,
    }

    impl<'a, N, T: TraversalOrder> TestTreeIter<'a, N, T> {
        fn new(roots: impl IntoIterator<Item = &'a N>) -> Self {
            Self {
                nodes: roots.into_iter().collect(),
                _order: PhantomData,
            }
        }
    }

    // Test that we can create iterators with different traversal orders
    #[test]
    fn test_traversal_order_trait_bounds() {
        let node = TestNode {
            value: 1,
            children: vec![],
        };

        // These should compile successfully if the trait bounds are correctly set up
        let _df_iter = TestTreeIter::<_, DepthFirst>::new([&node]);
        let _bf_iter = TestTreeIter::<_, BreadthFirst>::new([&node]);
    }

    // Test that the sealed trait pattern prevents outside implementation
    #[test]
    fn test_sealed_trait() {
        // This test doesn't actually assert anything at runtime
        // But it verifies that the code below doesn't compile (would fail at compile time)
        // The test passes if the following code remains commented out:
        
        // This would fail because CustomOrder doesn't implement the sealed Sealed trait
        // struct CustomOrder;
        // impl TraversalOrder for CustomOrder {}
    }
}
