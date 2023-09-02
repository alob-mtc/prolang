use std::any::Any;

pub mod ast;
mod parse_func;
pub mod parser;
#[cfg(test)]
mod test;

pub fn is_of_type<T: 'static>(x: &dyn Any) -> bool {
    x.is::<T>()
}

pub fn get_of_type<T: 'static>(x: &dyn Any) -> Option<&T> {
    x.downcast_ref::<T>()
}
