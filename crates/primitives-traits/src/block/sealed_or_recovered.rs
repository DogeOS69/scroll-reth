//! Shared sealed-or-recovered block container.

use crate::{
    block::{error::SealedBlockRecoveryError, RecoveredBlock, SealedBlock},
    Block,
};
use alloc::sync::Arc;
use core::ops::Deref;

/// A block that is either sealed or sealed with recovered transaction senders.
///
/// This is useful for APIs that must accept ordinary sealed blocks, but can skip sender recovery
/// when the caller already has a [`RecoveredBlock`].
#[derive(Debug, Clone)]
pub enum SealedOrRecoveredBlock<B: Block> {
    /// A sealed block without recovered senders.
    Sealed(Arc<SealedBlock<B>>),
    /// A sealed block with recovered senders.
    Recovered(Arc<RecoveredBlock<B>>),
}

impl<B: Block> SealedOrRecoveredBlock<B> {
    /// Creates a [`SealedOrRecoveredBlock`] from a sealed block.
    #[inline]
    pub fn sealed(block: SealedBlock<B>) -> Self {
        Self::Sealed(Arc::new(block))
    }

    /// Creates a [`SealedOrRecoveredBlock`] from a shared sealed block.
    #[inline]
    pub const fn sealed_arc(block: Arc<SealedBlock<B>>) -> Self {
        Self::Sealed(block)
    }

    /// Creates a [`SealedOrRecoveredBlock`] from a recovered block.
    #[inline]
    pub fn recovered(block: RecoveredBlock<B>) -> Self {
        Self::Recovered(Arc::new(block))
    }

    /// Creates a [`SealedOrRecoveredBlock`] from a shared recovered block.
    #[inline]
    pub const fn recovered_arc(block: Arc<RecoveredBlock<B>>) -> Self {
        Self::Recovered(block)
    }

    /// Returns the sealed block view.
    #[inline]
    pub fn sealed_block(&self) -> &SealedBlock<B> {
        match self {
            Self::Sealed(block) => block,
            Self::Recovered(block) => block.sealed_block(),
        }
    }

    /// Returns the recovered block if this block has recovered senders.
    #[inline]
    pub fn recovered_block(&self) -> Option<&RecoveredBlock<B>> {
        match self {
            Self::Sealed(_) => None,
            Self::Recovered(block) => Some(block),
        }
    }

    /// Consumes this block and returns the sealed block.
    pub fn into_sealed_block(self) -> SealedBlock<B> {
        match self {
            Self::Sealed(block) => Arc::unwrap_or_clone(block),
            Self::Recovered(block) => match Arc::try_unwrap(block) {
                Ok(block) => block.into_sealed_block(),
                Err(block) => block.clone_sealed_block(),
            },
        }
    }

    /// Consumes this block and returns the recovered block, recovering sealed-only blocks if
    /// needed.
    pub fn into_recovered_block(self) -> Result<RecoveredBlock<B>, SealedBlockRecoveryError<B>> {
        match self {
            Self::Sealed(block) => Arc::unwrap_or_clone(block).try_recover(),
            Self::Recovered(block) => Ok(Arc::unwrap_or_clone(block)),
        }
    }
}

impl<B: Block> From<SealedBlock<B>> for SealedOrRecoveredBlock<B> {
    #[inline]
    fn from(block: SealedBlock<B>) -> Self {
        Self::sealed(block)
    }
}

impl<B: Block> From<Arc<SealedBlock<B>>> for SealedOrRecoveredBlock<B> {
    #[inline]
    fn from(block: Arc<SealedBlock<B>>) -> Self {
        Self::sealed_arc(block)
    }
}

impl<B: Block> From<RecoveredBlock<B>> for SealedOrRecoveredBlock<B> {
    #[inline]
    fn from(block: RecoveredBlock<B>) -> Self {
        Self::recovered(block)
    }
}

impl<B: Block> From<Arc<RecoveredBlock<B>>> for SealedOrRecoveredBlock<B> {
    #[inline]
    fn from(block: Arc<RecoveredBlock<B>>) -> Self {
        Self::recovered_arc(block)
    }
}

impl<B: Block> Deref for SealedOrRecoveredBlock<B> {
    type Target = SealedBlock<B>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.sealed_block()
    }
}

#[cfg(feature = "serde")]
impl<B> serde::Serialize for SealedOrRecoveredBlock<B>
where
    B: Block,
    SealedBlock<B>: serde::Serialize,
{
    #[inline]
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.sealed_block().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, B> serde::Deserialize<'de> for SealedOrRecoveredBlock<B>
where
    B: Block,
    SealedBlock<B>: serde::Deserialize<'de>,
{
    #[inline]
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        SealedBlock::<B>::deserialize(deserializer).map(Self::sealed)
    }
}
