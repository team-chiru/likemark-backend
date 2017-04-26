use std::marker;

pub trait Composite<T> {
    fn is_empty(&self) -> bool;

    fn decompose(&self) -> Vec<T>;

    fn compose(&T) -> Self;

    fn compose_vec(Vec<T>) -> Vec<Self>
        where Self: marker::Sized;
}
