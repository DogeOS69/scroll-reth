use reth_eth_wire_types::BasicNetworkPrimitives;
use reth_network::{
    config::NetworkMode,
    protocol::{RlpxSubProtocol, RlpxSubProtocols},
    transactions::{config::AnnouncementAcceptance, AnnouncementFilteringPolicy},
    NetworkConfig, NetworkHandle, NetworkManager, PeersInfo,
};
use reth_node_api::TxTy;
use reth_node_builder::{components::NetworkBuilder, BuilderContext, FullNodeTypes};
use reth_node_types::NodeTypes;
use reth_scroll_chainspec::ScrollChainSpec;
use reth_scroll_primitives::ScrollPrimitives;
use reth_tracing::tracing::info;
use reth_transaction_pool::{PoolTransaction, TransactionPool};
use scroll_alloy_consensus::{ScrollPooledTransaction, ScrollTxType};
use std::fmt::Debug;
/// The network builder for Scroll.
#[derive(Debug, Default)]
pub struct ScrollNetworkBuilder {
    /// Additional `RLPx` sub-protocols to be added to the network.
    scroll_sub_protocols: RlpxSubProtocols,
}

impl ScrollNetworkBuilder {
    /// Create a new [`ScrollNetworkBuilder`] with default configuration.
    pub fn new() -> Self {
        Self { scroll_sub_protocols: RlpxSubProtocols::default() }
    }

    /// Add a scroll sub-protocol to the network builder.
    pub fn with_sub_protocol(mut self, protocol: RlpxSubProtocol) -> Self {
        self.scroll_sub_protocols.push(protocol);
        self
    }
}

impl<Node, Pool> NetworkBuilder<Node, Pool> for ScrollNetworkBuilder
where
    Node:
        FullNodeTypes<Types: NodeTypes<ChainSpec = ScrollChainSpec, Primitives = ScrollPrimitives>>,
    Pool: TransactionPool<
            Transaction: PoolTransaction<
                Consensus = TxTy<Node::Types>,
                Pooled = scroll_alloy_consensus::ScrollPooledTransaction,
            >,
        > + Unpin
        + 'static,
{
    type Network = NetworkHandle<ScrollNetworkPrimitives>;

    async fn build_network(
        self,
        ctx: &BuilderContext<Node>,
        pool: Pool,
    ) -> eyre::Result<Self::Network> {
        // set the network mode to work.
        let config = ctx.network_config()?;
        let config = NetworkConfig {
            network_mode: NetworkMode::Work,
            extra_protocols: self.scroll_sub_protocols,
            ..config
        };

        let network = NetworkManager::builder(config).await?;
        let handle = ctx.start_network_with_policies(
            network,
            pool,
            ctx.config().network.transactions_manager_config(),
            ctx.config().network.tx_propagation_policy,
            ScrollAnnouncementFilter,
            None,
        );
        info!(target: "reth::cli", enode=%handle.local_node_record(), "P2P networking initialized");
        Ok(handle)
    }
}

/// Network primitive types used by Scroll networks.
pub type ScrollNetworkPrimitives =
    BasicNetworkPrimitives<ScrollPrimitives, ScrollPooledTransaction>;

/// Announcement filter that accepts Scroll transaction type bytes.
#[derive(Debug, Clone, Copy, Default)]
struct ScrollAnnouncementFilter;

impl AnnouncementFilteringPolicy<ScrollNetworkPrimitives> for ScrollAnnouncementFilter {
    fn decide_on_announcement(
        &self,
        ty: u8,
        _hash: &alloy_primitives::B256,
        _size: usize,
    ) -> AnnouncementAcceptance {
        if ScrollTxType::try_from(ty).is_ok() {
            AnnouncementAcceptance::Accept
        } else {
            AnnouncementAcceptance::Reject { penalize_peer: true }
        }
    }
}
