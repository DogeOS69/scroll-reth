use alloc::boxed::Box;
use alloy_primitives::{Address, U256};
use revm::{
    state::{Account, AccountInfo, AccountStatus, EvmState, EvmStorage, EvmStorageSlot},
    Database, DatabaseCommit,
};

/// Applies a protocol-level account change through the public revm database interfaces.
///
/// Alloy EVM 0.30 no longer exposes the concrete revm state cache to block executors, so Scroll
/// hardfork migrations must be expressed as an ordinary state commit.
pub(super) fn apply_account_change<DB>(
    db: &mut DB,
    address: Address,
    new_info: impl FnOnce(AccountInfo) -> AccountInfo,
    storage_updates: &[(U256, U256)],
) -> Result<(), DB::Error>
where
    DB: Database + DatabaseCommit,
{
    let original_info = db.basic(address)?.unwrap_or_default();
    let info = new_info(original_info.clone());
    let mut storage = EvmStorage::default();

    for &(slot, present_value) in storage_updates {
        let original_value = db.storage(address, slot)?;
        storage.insert(slot, EvmStorageSlot::new_changed(original_value, present_value, 0));
    }

    let account = Account {
        info,
        original_info: Box::new(original_info),
        storage,
        status: AccountStatus::Touched,
        ..Default::default()
    };
    let mut changes = EvmState::default();
    changes.insert(address, account);
    db.commit(changes);

    Ok(())
}
