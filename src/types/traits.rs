
// Traits to help with splitting / derivation of components.
// Note that ordering must be preserved in all cases.
// example:
//   a = [A, B, C]
//   b = [A, C, C]
//   common = extract_common(a, b) -> [A]
//   common.is_superset(a) -> true
//   a.is_subset(common) -> true
//   diff_a = extract_diff([A], a) -> [B, C]
//   diff_b = extract_diff([A], b) -> [C, C]
//   a == merge(common, diff_a)
//   b == merge(common, diff_b)


pub trait Subset {
    /// Subset checks if one object is a subset of another
    fn is_subset(&self, other: &Self) -> bool;
}

pub trait Superset {
    /// Superset checks if one object is a superset of another
    fn is_superset(&self, other: &Self) -> bool;
}

pub trait Common {
    /// Extract common creates a new object from the commonality between objects
    fn extract_common(&self, other: &Self) -> Self;
}

pub trait Diff {
    /// Diff creates a new object from the difference between two objects
    fn diff(&self, other: &Self) -> Self;
}

pub trait Merge {
    /// Merge creates a new object from the combination of two objects
    fn merge(&self, other: &Self) -> Self;
}
