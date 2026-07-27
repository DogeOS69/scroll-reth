//! Compatibility trait retained for Scroll's bincode representations.

use core::fmt::Debug;
use serde::{de::DeserializeOwned, Serialize};

/// Converts a type to and from a representation that is stable under bincode serialization.
pub trait SerdeBincodeCompat: Sized + 'static {
    /// Bincode-compatible representation.
    type BincodeRepr<'a>: Debug + Serialize + DeserializeOwned;

    /// Returns the bincode-compatible representation.
    fn as_repr(&self) -> Self::BincodeRepr<'_>;

    /// Reconstructs the value from its bincode-compatible representation.
    fn from_repr(repr: Self::BincodeRepr<'_>) -> Self;
}
