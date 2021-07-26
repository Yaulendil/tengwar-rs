//! This module consolidates the modes of transliteration for simplicity.

pub mod beleriand;
// pub mod general;
pub mod gondor;
pub mod quenya;

pub use beleriand::Beleriand;
// pub use general::General;
pub use gondor::Gondor;
pub use quenya::Quenya;
