//! Tsuki fork transition for Dogeos.
//!
//! On the first block of the Tsuki fork, Dogeos performed a transition to the Tsuki fork state,
//! changes to the protocol:
//!   1. Set the code of address `0x0x530000000000000000000000000000000000d09e` to NativeDogeToken
//!      bytecode.

use alloc::vec;
use alloy_primitives::bytes;
use revm::{
    bytecode::Bytecode,
    database::{
        bal::EvmDatabaseError,
        states::{plain_account::StorageWithOriginalValues, State},
    },
    primitives::Bytes,
    state::AccountInfo,
    Database,
};
use revm_scroll::precompile::transfer::NATIVE_DOGE_TOKEN_ADDRESS;

const TSUKI_NATIVE_DOGE_TOKEN_BYTECODE: Bytes = bytes!("");

/// Applies the Tsuki hard fork to the state by reserving the NativeDogeToken address.
///
/// The token account remains ordinary EVM state. The transfer precompile hardcodes this address as
/// its only allowed caller, and this migration only creates the account if it is still empty. This
/// makes the transition compatible with mainnet genesis predeploys: if genesis already contains
/// code at the same address, the migration is a no-op and does not overwrite it.
pub(super) fn apply_tsuki_hard_fork<DB: Database>(
    state: &mut State<DB>,
) -> Result<(), <State<DB> as Database>::Error> {
    let token =
        state.load_cache_account(NATIVE_DOGE_TOKEN_ADDRESS).map_err(EvmDatabaseError::Database)?;

    let old_info = token.account_info().unwrap_or_default();
    if old_info.nonce != 0 {
        return Ok(());
    }

    let bytecode = Bytecode::new_raw(TSUKI_NATIVE_DOGE_TOKEN_BYTECODE);
    let code_hash = bytecode.hash_slow();
    let new_info = AccountInfo { nonce: 1, code_hash, code: Some(bytecode), ..old_info };

    let transition = token.change(new_info, StorageWithOriginalValues::default());

    if let Some(s) = state.transition_state.as_mut() {
        s.add_transitions(vec![(NATIVE_DOGE_TOKEN_ADDRESS, transition)])
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use revm::{
        database::{states::bundle_state::BundleRetention, CacheDB, EmptyDB, State},
        primitives::{bytes, U256},
        state::{AccountInfo, Bytecode},
    };

    #[test]
    fn test_apply_tsuki_fork_inserts_native_doge_token() -> eyre::Result<()> {
        let db = EmptyDB::new();
        let mut state =
            State::builder().with_database(db).with_bundle_update().without_state_clear().build();

        apply_tsuki_hard_fork(&mut state)?;

        state.merge_transitions(BundleRetention::Reverts);
        let bundle = state.take_bundle();
        let token = bundle.state.get(&NATIVE_DOGE_TOKEN_ADDRESS).unwrap();

        let bytecode = Bytecode::new_raw(TSUKI_NATIVE_DOGE_TOKEN_BYTECODE);
        let expected_info = AccountInfo {
            nonce: 1,
            balance: U256::ZERO,
            code_hash: bytecode.hash_slow(),
            code: Some(bytecode),
            ..Default::default()
        };

        assert_eq!(token.info.as_ref().unwrap(), &expected_info);

        Ok(())
    }

    #[test]
    fn test_apply_tsuki_fork_does_not_overwrite_existing_predeploy() -> eyre::Result<()> {
        let bytecode = Bytecode::new_raw(bytes!("00"));
        let predeploy_info = AccountInfo::from_bytecode(bytecode);

        let mut db = CacheDB::new(EmptyDB::default());
        db.insert_account_info(NATIVE_DOGE_TOKEN_ADDRESS, predeploy_info);
        let mut state =
            State::builder().with_database(db).with_bundle_update().without_state_clear().build();

        apply_tsuki_hard_fork(&mut state)?;

        state.merge_transitions(BundleRetention::Reverts);
        let bundle = state.take_bundle();

        assert_eq!(bundle.state.get(&NATIVE_DOGE_TOKEN_ADDRESS), None);

        Ok(())
    }

    #[test]
    fn test_apply_tsuki_fork_does_not_run_twice() -> eyre::Result<()> {
        let bytecode = Bytecode::new_raw(TSUKI_NATIVE_DOGE_TOKEN_BYTECODE);
        let token_info = AccountInfo {
            nonce: 1,
            code_hash: bytecode.hash_slow(),
            code: Some(bytecode),
            ..Default::default()
        };

        let mut db = CacheDB::new(EmptyDB::default());
        db.insert_account_info(NATIVE_DOGE_TOKEN_ADDRESS, token_info);
        let mut state =
            State::builder().with_database(db).with_bundle_update().without_state_clear().build();

        apply_tsuki_hard_fork(&mut state)?;

        state.merge_transitions(BundleRetention::Reverts);
        let bundle = state.take_bundle();

        assert_eq!(bundle.state.get(&NATIVE_DOGE_TOKEN_ADDRESS), None);

        Ok(())
    }
}
