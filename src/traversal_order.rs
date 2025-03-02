/// Marker struct representing breadth-first traversal order.
///
/// Breadth-first traversal visits all nodes at the same depth level before moving to the next level.
/// This is implemented using a queue where children are added to the back of the queue.
#[derive(Debug)]
pub struct BreadthFirst;

/// Marker struct representing depth-first traversal order.
///
/// Depth-first traversal explores as far down a branch as possible before backtracking.
/// This is implemented by adding children to the front of the queue or using a stack.
#[derive(Debug)]
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
