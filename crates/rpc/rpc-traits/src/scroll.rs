use crate::{FromConsensusTx, SignTxRequestError, SignableTxRequest, TryIntoSimTx};

use alloy_consensus::{error::ValueError, transaction::Recovered, SignableTransaction};
use alloy_network::TxSigner;
use alloy_primitives::{Address, Signature};
use core::convert::Infallible;
use scroll_alloy_consensus::{ScrollTransactionInfo, ScrollTxEnvelope, ScrollTypedTransaction};
use scroll_alloy_rpc_types::{ScrollTransactionRequest, Transaction};

impl SignableTxRequest<ScrollTxEnvelope> for ScrollTransactionRequest {
    async fn try_build_and_sign(
        self,
        signer: impl TxSigner<Signature> + Send,
    ) -> Result<ScrollTxEnvelope, SignTxRequestError> {
        let mut tx =
            self.build_typed_tx().map_err(|_| SignTxRequestError::InvalidTransactionRequest)?;
        let signature = signer.sign_transaction(&mut tx).await?;

        let signed = match tx {
            ScrollTypedTransaction::Legacy(tx) => {
                ScrollTxEnvelope::Legacy(tx.into_signed(signature))
            }
            ScrollTypedTransaction::Eip2930(tx) => {
                ScrollTxEnvelope::Eip2930(tx.into_signed(signature))
            }
            ScrollTypedTransaction::Eip1559(tx) => {
                ScrollTxEnvelope::Eip1559(tx.into_signed(signature))
            }
            ScrollTypedTransaction::Eip7702(tx) => {
                ScrollTxEnvelope::Eip7702(tx.into_signed(signature))
            }
            ScrollTypedTransaction::L1Message(_) => {
                return Err(SignTxRequestError::InvalidTransactionRequest);
            }
        };

        Ok(signed)
    }
}

impl FromConsensusTx<ScrollTxEnvelope> for Transaction {
    type TxInfo = ScrollTransactionInfo;
    type Err = Infallible;

    fn from_consensus_tx(
        tx: ScrollTxEnvelope,
        signer: Address,
        tx_info: Self::TxInfo,
    ) -> Result<Self, Self::Err> {
        Ok(Self::from_transaction(Recovered::new_unchecked(tx, signer), tx_info))
    }
}

impl TryIntoSimTx<ScrollTxEnvelope> for ScrollTransactionRequest {
    fn try_into_sim_tx(self) -> Result<ScrollTxEnvelope, ValueError<Self>> {
        let tx = self
            .build_typed_tx()
            .map_err(|request| ValueError::new(request, "Required fields missing"))?;

        let signature = Signature::new(Default::default(), Default::default(), false);

        Ok(tx.into_signed(signature).into())
    }
}
