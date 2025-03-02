/*!
 * # Tree-Iter
 *
 * A Rust library for iterating over tree structures in different traversal orders.
 *
 * This library provides traits and implementations for efficient iteration over tree-like data
 * structures in both breadth-first and depth-first traversal orders, with support for both
 * immutable and mutable traversal.
 *
 * ## Features
 *
 * - Generic support for any tree-like structure
 * - Breadth-first and depth-first traversal orders
 * - Immutable and mutable iteration
 * - Safe interior mutability during traversal using guard patterns
 *
 * ## Example
 *
 * ```rust
 * use tree_iter::prelude::*;
 * use tree_iter::tree::Node;
 *
 * // Create a simple tree
 * let tree = Node {
 *     value: 1,
 *     children: vec![
 *         Node {
 *             value: 2,
 *             children: vec![],
 *         },
 *         Node {
 *             value: 3,
 *             children: vec![],
 *         },
 *     ],
 * };
 *
 * // Iterate over the tree in depth-first order
 * let values: Vec<i32> = tree.iter::<DepthFirst>()
 *                           .map(|node| node.value)
 *                           .collect();
 * assert_eq!(values, vec![1, 2, 3]);
 * ```
 */

/// Tree iteration modules for immutable references
pub mod iter;
/// Tree iteration modules for mutable references
pub mod iter_mut;
/// Traversal order definitions (breadth-first and depth-first)
pub mod traversal_order;
/// Default tree implementation
pub mod tree;

/// Prelude module for convenient imports of common types
pub mod prelude {
    pub use crate::iter::TreeNode;
    pub use crate::iter_mut::TreeNodeMut;
    pub use crate::traversal_order::{BreadthFirst, DepthFirst, TraversalOrder};
}
