use std::fmt::Display;


pub struct GenericError<E: Display> {
    pub error: E
}