use crate::Hash;

pub trait Hashable {
    fn hash(&self) -> Hash;
}
