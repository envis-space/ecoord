/// Determines where data is stored within an octree structure.
///
/// This enum controls the storage strategy for data in an octree, allowing you to choose
/// between different storage patterns for interior and leaf octants.
///
/// # Variants
///
/// * `AllOctants` - Data can be stored in both interior and leaf octants depending on
///   where it naturally belongs in the tree hierarchy. Each piece of data is stored
///   only once at its appropriate level. Useful for hierarchical data where different
///   data exists at different levels of detail.
///
/// * `AllOctantsWithPropagation` - Data stored at any level is also propagated to all
///   child octants (duplicated down the tree). This enables fast queries at any level
///   while maintaining hierarchical relationships, at the cost of increased memory usage.
///
/// * `LeafOctantsOnly` - Data is stored exclusively in leaf octants (the deepest level
///   of subdivision). This constrains all data to the finest level of detail.
///
/// # Performance Considerations
///
/// - `AllOctants`: Memory efficient, data stored once at the appropriate level
/// - `AllOctantsWithPropagation`: Higher memory usage, faster queries at all levels
/// - `LeafOctantsOnly`: Uniform depth, simpler traversal logic
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Ord, PartialOrd, Default)]
pub enum StorageMode {
    #[default]
    /// Store data in all octants (interior + leaf) without duplication
    AllOctants,
    /// Store data in all octants with propagation to children
    AllOctantsWithPropagation,
    /// Store data only in leaf octants
    LeafOctantsOnly,
}
