mod error_output;
mod error_trait;

pub use error_trait::{
    DebugInfo,
    DescribableError
};
pub use error_output::print_error;
