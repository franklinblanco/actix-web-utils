use std::fmt::Display;
use serde::Serialize;

use err::{Error, MessageResource};
use crate::{extensions::{typed_response::TypedHttpResponse, generic_error::GenericError}};

/// This trait aims to aid macros defined in this crate so that the macro can take any shape of error and
/// do the same thing for all.
pub trait ReturnableErrorShape {
    fn convert_to_returnable<T: Serialize> (&self, status_code: u16) -> TypedHttpResponse<T>;
}
impl ReturnableErrorShape for MessageResource {
    fn convert_to_returnable<T: Serialize>(&self, status_code: u16) -> TypedHttpResponse<T> {
        TypedHttpResponse::return_standard_error(status_code, self.clone())
    }
}
impl ReturnableErrorShape for Error {
    fn convert_to_returnable<T: Serialize>(&self, status_code: u16) -> TypedHttpResponse<T> {
        match self {
            Error::Unspecified => TypedHttpResponse::return_standard_error(status_code, MessageResource::from(self)),
            Error::Network(message) => TypedHttpResponse::return_standard_error(status_code, message.clone()),
            Error::UnexpectedStatusCode(_, actual, messages) => TypedHttpResponse::return_standard_error_list(*actual, messages.clone()),
            Error::Serde(message) => TypedHttpResponse::return_standard_error(status_code, message.clone()),
            Error::IO(message) => TypedHttpResponse::return_standard_error(status_code, message.clone()),
            Error::Privilege(message) => TypedHttpResponse::return_standard_error(status_code, message.clone()),
            Error::Parser(message) => TypedHttpResponse::return_standard_error(status_code, message.clone()),
        }
    }
}
impl ReturnableErrorShape for Vec<MessageResource> {
    fn convert_to_returnable<T: Serialize>(&self, status_code: u16) -> TypedHttpResponse<T> {
        TypedHttpResponse::return_standard_error_list(status_code, self.to_vec())
    }
}
impl<E: Display> ReturnableErrorShape for GenericError<E>{
    fn convert_to_returnable<T: Serialize>(&self, status_code: u16) -> TypedHttpResponse<T> {
        TypedHttpResponse::return_standard_error(status_code, MessageResource::new_from_str(&self.error.to_string()))
    }
}
