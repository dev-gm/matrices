pub use std::collections::HashMap;
use std::any::Any;

pub trait Group {
    fn empty() -> Self;

    fn zero(&self) -> Self; // self * zero() == zero() and self + zero() == self

    fn operation(&self, key: OperationType) -> Option<Operation<Self>>;
    
    fn identity(&self, key: OperationType) -> Option<Box<Self>>
}

#[derive(Hash, Eq, PartialEq)]
pub enum OperationType { Add, Sub, Mul, Div }

pub enum Operation<T: ?Sized> {
    Closed(fn(T, T) -> T),
    Open(fn(T, T) -> Box<dyn Any>),
}
