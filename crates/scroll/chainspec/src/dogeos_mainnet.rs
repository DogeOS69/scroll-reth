//! Chain specification for the DogeOS Mainnet network.

use crate::{
    constants::SCROLL_BASE_FEE_PARAMS_FEYNMAN, make_genesis_header, LazyLock, ScrollChainConfig,
    ScrollChainSpec, DOGEOS_MAINNET_GENESIS_HASH,
};
use alloc::{sync::Arc, vec};

use alloy_chains::Chain;
use reth_chainspec::{BaseFeeParamsKind, ChainSpec, Hardfork};
use reth_primitives_traits::SealedHeader;
use reth_scroll_forks::DOGEOS_MAINNET_HARDFORKS;
use scroll_alloy_hardforks::ScrollHardfork;

/// The DogeOS Mainnet spec
pub static DOGEOS_MAINNET: LazyLock<Arc<ScrollChainSpec>> = LazyLock::new(|| {
    let genesis = serde_json::from_str(include_str!("../res/genesis/dogeos_mainnet.json")) // FIXME: update the genesis file
        .expect("Can't deserialize DogeOS Mainnet genesis json");
    ScrollChainSpec {
        inner: ChainSpec {
            chain: Chain::from_id(0xdeadbeef), // FIXME: replace with real one.
            genesis_header: SealedHeader::new(
                make_genesis_header(&genesis),
                DOGEOS_MAINNET_GENESIS_HASH,
            ),
            genesis,
            hardforks: DOGEOS_MAINNET_HARDFORKS.clone(),
            base_fee_params: BaseFeeParamsKind::Variable(
                vec![(ScrollHardfork::Feynman.boxed(), SCROLL_BASE_FEE_PARAMS_FEYNMAN)].into(),
            ),
            ..Default::default()
        },
        config: ScrollChainConfig::dogeos_mainnet(),
    }
        .into()
});
