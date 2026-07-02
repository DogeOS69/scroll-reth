#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg))]

use alloy_consensus::TxType;
pub use alloy_network::*;
use alloy_primitives::{Address, Bytes, ChainId, TxKind, U256};
use alloy_provider::fillers::{
    ChainIdFiller, GasFiller, JoinFill, NonceFiller, RecommendedFillers,
};
use alloy_rpc_types_eth::{
    request::TransactionRequest as EthTransactionRequest, AccessList, TransactionInputKind,
};
use alloy_signer as _;
use scroll_alloy_consensus::{self, ScrollTxEnvelope, ScrollTxType, ScrollTypedTransaction};
use scroll_alloy_rpc_types::{ScrollTransactionRequest, Transaction};
use serde::{Deserialize, Serialize};

/// Types for a Scroll-stack network.
#[derive(Clone, Copy, Debug)]
pub struct Scroll {
    _private: (),
}

impl Network for Scroll {
    type TxType = ScrollTxType;

    type TxEnvelope = scroll_alloy_consensus::ScrollTxEnvelope;

    type UnsignedTx = scroll_alloy_consensus::ScrollTypedTransaction;

    type ReceiptEnvelope = scroll_alloy_consensus::ScrollReceiptEnvelope;

    type Header = alloy_consensus::Header;

    type TransactionRequest = ScrollNetworkTransactionRequest;

    type TransactionResponse = scroll_alloy_rpc_types::Transaction;

    type ReceiptResponse = scroll_alloy_rpc_types::ScrollTransactionReceipt;

    type HeaderResponse = alloy_rpc_types_eth::Header;

    type BlockResponse =
        alloy_rpc_types_eth::Block<Self::TransactionResponse, Self::HeaderResponse>;
}

/// Scroll transaction request used by the Alloy network abstraction.
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ScrollNetworkTransactionRequest(ScrollTransactionRequest);

impl From<ScrollTransactionRequest> for ScrollNetworkTransactionRequest {
    fn from(value: ScrollTransactionRequest) -> Self {
        Self(value)
    }
}

impl From<ScrollNetworkTransactionRequest> for ScrollTransactionRequest {
    fn from(value: ScrollNetworkTransactionRequest) -> Self {
        value.0
    }
}

impl AsRef<ScrollTransactionRequest> for ScrollNetworkTransactionRequest {
    fn as_ref(&self) -> &ScrollTransactionRequest {
        &self.0
    }
}

impl AsMut<ScrollTransactionRequest> for ScrollNetworkTransactionRequest {
    fn as_mut(&mut self) -> &mut ScrollTransactionRequest {
        &mut self.0
    }
}

impl AsRef<EthTransactionRequest> for ScrollNetworkTransactionRequest {
    fn as_ref(&self) -> &EthTransactionRequest {
        self.inner()
    }
}

impl AsMut<EthTransactionRequest> for ScrollNetworkTransactionRequest {
    fn as_mut(&mut self) -> &mut EthTransactionRequest {
        self.inner_mut()
    }
}

impl ScrollNetworkTransactionRequest {
    fn inner(&self) -> &EthTransactionRequest {
        self.0.as_ref()
    }

    fn inner_mut(&mut self) -> &mut EthTransactionRequest {
        self.0.as_mut()
    }
}

fn scroll_tx_type(tx_type: TxType) -> ScrollTxType {
    match tx_type {
        TxType::Eip1559 | TxType::Eip4844 => ScrollTxType::Eip1559,
        TxType::Eip2930 => ScrollTxType::Eip2930,
        TxType::Legacy => ScrollTxType::Legacy,
        TxType::Eip7702 => ScrollTxType::Eip7702,
    }
}

impl From<ScrollTxEnvelope> for ScrollNetworkTransactionRequest {
    fn from(value: ScrollTxEnvelope) -> Self {
        Self(value.into())
    }
}

impl From<ScrollTypedTransaction> for ScrollNetworkTransactionRequest {
    fn from(value: ScrollTypedTransaction) -> Self {
        Self(value.into())
    }
}

impl From<Transaction> for ScrollNetworkTransactionRequest {
    fn from(value: Transaction) -> Self {
        Self(value.inner.into_inner().into())
    }
}

impl TransactionBuilder for ScrollNetworkTransactionRequest {
    fn chain_id(&self) -> Option<ChainId> {
        self.inner().chain_id()
    }

    fn set_chain_id(&mut self, chain_id: ChainId) {
        self.inner_mut().set_chain_id(chain_id);
    }

    fn nonce(&self) -> Option<u64> {
        self.inner().nonce()
    }

    fn set_nonce(&mut self, nonce: u64) {
        self.inner_mut().set_nonce(nonce);
    }

    fn take_nonce(&mut self) -> Option<u64> {
        self.inner_mut().take_nonce()
    }

    fn input(&self) -> Option<&Bytes> {
        self.inner().input()
    }

    fn set_input<T: Into<Bytes>>(&mut self, input: T) {
        self.inner_mut().set_input(input);
    }

    fn set_input_kind<T: Into<Bytes>>(&mut self, input: T, kind: TransactionInputKind) {
        self.inner_mut().set_input_kind(input, kind);
    }

    fn from(&self) -> Option<Address> {
        self.inner().from()
    }

    fn set_from(&mut self, from: Address) {
        self.inner_mut().set_from(from);
    }

    fn kind(&self) -> Option<TxKind> {
        self.inner().kind()
    }

    fn clear_kind(&mut self) {
        self.inner_mut().clear_kind();
    }

    fn set_kind(&mut self, kind: TxKind) {
        self.inner_mut().set_kind(kind);
    }

    fn value(&self) -> Option<U256> {
        self.inner().value()
    }

    fn set_value(&mut self, value: U256) {
        self.inner_mut().set_value(value);
    }

    fn gas_price(&self) -> Option<u128> {
        self.inner().gas_price()
    }

    fn set_gas_price(&mut self, gas_price: u128) {
        self.inner_mut().set_gas_price(gas_price);
    }

    fn max_fee_per_gas(&self) -> Option<u128> {
        self.inner().max_fee_per_gas()
    }

    fn set_max_fee_per_gas(&mut self, max_fee_per_gas: u128) {
        self.inner_mut().set_max_fee_per_gas(max_fee_per_gas);
    }

    fn max_priority_fee_per_gas(&self) -> Option<u128> {
        self.inner().max_priority_fee_per_gas()
    }

    fn set_max_priority_fee_per_gas(&mut self, max_priority_fee_per_gas: u128) {
        self.inner_mut().set_max_priority_fee_per_gas(max_priority_fee_per_gas);
    }

    fn gas_limit(&self) -> Option<u64> {
        self.inner().gas_limit()
    }

    fn set_gas_limit(&mut self, gas_limit: u64) {
        self.inner_mut().set_gas_limit(gas_limit);
    }

    fn access_list(&self) -> Option<&AccessList> {
        self.inner().access_list()
    }

    fn set_access_list(&mut self, access_list: AccessList) {
        self.inner_mut().set_access_list(access_list);
    }
}

impl NetworkTransactionBuilder<Scroll> for ScrollNetworkTransactionRequest {
    fn complete_type(&self, ty: ScrollTxType) -> Result<(), Vec<&'static str>> {
        match ty {
            ScrollTxType::L1Message => Err(vec!["not implemented for L1 message tx"]),
            ScrollTxType::Legacy => self.inner().complete_legacy(),
            ScrollTxType::Eip2930 => self.inner().complete_2930(),
            ScrollTxType::Eip1559 => self.inner().complete_1559(),
            ScrollTxType::Eip7702 => self.inner().complete_7702(),
        }
    }

    fn can_submit(&self) -> bool {
        self.inner().can_submit()
    }

    fn can_build(&self) -> bool {
        let inner = self.inner();
        let common = inner.gas.is_some() && inner.nonce.is_some();
        let legacy = inner.gas_price.is_some();
        let eip2930 = legacy && inner.access_list().is_some();
        let eip1559 = inner.max_fee_per_gas.is_some() && inner.max_priority_fee_per_gas.is_some();
        let eip7702 = eip1559 && inner.authorization_list().is_some();

        common && (legacy || eip2930 || eip1559 || eip7702)
    }

    #[doc(alias = "output_transaction_type")]
    fn output_tx_type(&self) -> ScrollTxType {
        scroll_tx_type(self.inner().preferred_type())
    }

    #[doc(alias = "output_transaction_type_checked")]
    fn output_tx_type_checked(&self) -> Option<ScrollTxType> {
        self.inner().buildable_type().map(scroll_tx_type)
    }

    fn prep_for_submission(&mut self) {
        self.inner_mut().prep_for_submission();
    }

    fn build_unsigned(self) -> BuildResult<ScrollTypedTransaction, Scroll> {
        if let Err((tx_type, missing)) = self.inner().missing_keys() {
            let tx_type = scroll_tx_type(tx_type);
            return Err(TransactionBuilderError::InvalidTransactionRequest(tx_type, missing)
                .into_unbuilt(self));
        }
        Ok(self.0.build_typed_tx().expect("checked by missing_keys"))
    }

    async fn build<W: NetworkWallet<Scroll>>(
        self,
        wallet: &W,
    ) -> Result<<Scroll as Network>::TxEnvelope, TransactionBuilderError<Scroll>> {
        Ok(wallet.sign_request(self).await?)
    }
}

impl RecommendedFillers for Scroll {
    type RecommendedFillers = JoinFill<GasFiller, JoinFill<NonceFiller, ChainIdFiller>>;

    fn recommended_fillers() -> Self::RecommendedFillers {
        Default::default()
    }
}
