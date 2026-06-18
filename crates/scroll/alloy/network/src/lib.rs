#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), warn(unused_crate_dependencies))]
#![cfg_attr(docsrs, feature(doc_cfg))]

use alloy_consensus::TxType;
pub use alloy_network::*;
use alloy_provider::fillers::{
    ChainIdFiller, GasFiller, JoinFill, NonceFiller, RecommendedFillers,
};
use scroll_alloy_consensus::{self, ScrollTxType, ScrollTypedTransaction};
use scroll_alloy_rpc_types::ScrollTransactionRequest;

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

    type TransactionRequest = scroll_alloy_rpc_types::ScrollTransactionRequest;

    type TransactionResponse = scroll_alloy_rpc_types::Transaction;

    type ReceiptResponse = scroll_alloy_rpc_types::ScrollTransactionReceipt;

    type HeaderResponse = alloy_rpc_types_eth::Header;

    type BlockResponse =
        alloy_rpc_types_eth::Block<Self::TransactionResponse, Self::HeaderResponse>;
}

impl NetworkTransactionBuilder<Scroll> for ScrollTransactionRequest {
    fn complete_type(&self, ty: ScrollTxType) -> Result<(), Vec<&'static str>> {
        match ty {
            ScrollTxType::L1Message => Err(vec!["not implemented for L1 message tx"]),
            _ => {
                let ty = TxType::try_from(ty as u8).unwrap();
                self.as_ref().complete_type(ty)
            }
        }
    }

    fn can_submit(&self) -> bool {
        self.as_ref().can_submit()
    }

    fn can_build(&self) -> bool {
        self.as_ref().can_build()
    }

    #[doc(alias = "output_transaction_type")]
    fn output_tx_type(&self) -> ScrollTxType {
        match self.as_ref().preferred_type() {
            TxType::Eip1559 | TxType::Eip4844 => ScrollTxType::Eip1559,
            TxType::Eip2930 => ScrollTxType::Eip2930,
            TxType::Legacy => ScrollTxType::Legacy,
            TxType::Eip7702 => ScrollTxType::Eip7702,
        }
    }

    #[doc(alias = "output_transaction_type_checked")]
    fn output_tx_type_checked(&self) -> Option<ScrollTxType> {
        self.as_ref().buildable_type().map(|tx_ty| match tx_ty {
            TxType::Eip1559 | TxType::Eip4844 => ScrollTxType::Eip1559,
            TxType::Eip2930 => ScrollTxType::Eip2930,
            TxType::Legacy => ScrollTxType::Legacy,
            TxType::Eip7702 => ScrollTxType::Eip7702,
        })
    }

    fn prep_for_submission(&mut self) {
        self.as_mut().prep_for_submission();
    }

    fn build_unsigned(self) -> BuildResult<ScrollTypedTransaction, Scroll> {
        if let Err((tx_type, missing)) = self.as_ref().missing_keys() {
            let tx_type = ScrollTxType::try_from(tx_type as u8).unwrap();
            return Err(TransactionBuilderError::InvalidTransactionRequest(tx_type, missing)
                .into_unbuilt(self));
        }
        Ok(self.build_typed_tx().expect("checked by missing_keys"))
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
