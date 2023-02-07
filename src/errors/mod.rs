mod error_span;
mod error_trait;

pub use error_trait::{
    DebugInfo,
    DescribableError,
};

pub use error_span::Span;
