
pub trait Subset {
    fn is_subset(&self, other: &Self) -> bool;
}

pub trait Superset {
    fn is_superset(&self, other: &Self) -> bool;
}

pub trait Common {
    fn extract_common(&self, other: &Self) -> Self;
}
