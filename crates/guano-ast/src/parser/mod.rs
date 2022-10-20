pub mod expression;
mod parser;
pub mod scope;
pub mod statement;
pub mod typing;
pub mod identifier;

pub use parser::*;

pub trait ConvertResult<T, E: std::error::Error> {
    fn convert_result(self) -> Result<T, Option<E>>;
}

impl<T, E: std::error::Error> ConvertResult<T, E> for Option<T> {
    fn convert_result(self) -> Result<T, Option<E>> {
        self.ok_or(None)
    }
}

#[macro_export]
macro_rules! convert_result_impl {
    ($error_type:ty) => {
        impl<T, E: std::error::Error + Into<$error_type>>
            $crate::parser::ConvertResult<T, $error_type> for Result<T, Option<E>>
        {
            fn convert_result(self) -> Result<T, Option<$error_type>> {
                self.map_err(|e| e.map(|e| e.into()))
            }
        }

        impl<T, E: std::error::Error + Into<$error_type>>
            $crate::parser::ConvertResult<T, $error_type> for E
        {
            fn convert_result(self) -> Result<T, Option<$error_type>> {
                Err(Some(self.into()))
            }
        }
    };
}
