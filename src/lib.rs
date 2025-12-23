pub mod classic;
pub mod traits;
pub mod utils;
pub mod error;

pub use traits::Cipher;
pub use error::PolygraphiaError;
pub use utils::TextMode;
pub use classic::Caesar;
pub use classic::Affine;