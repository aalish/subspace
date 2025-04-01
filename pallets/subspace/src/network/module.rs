use crate::*;

use frame_support::pallet_prelude::DispatchResult;
use pallet_subnet_emission_api::SubnetEmissionApi;
pub struct SubnetDistributionParameters;

impl<T: Config> Pallet<T> {
    pub fn do_update_module(
        origin: T::RuntimeOrigin,
        netuid: u16,
        changeset: ModuleChangeset<T>,
    ) -> DispatchResult {
        let key = ensure_signed(origin)?;
        let uid: u16 = Self::get_uid_for_key(netuid, &key).ok_or(Error::<T>::ModuleDoesNotExist)?;
        changeset.apply(netuid, key, uid)?;
        Ok(())
    }

    pub fn append_module(
        net_key: &T::AccountId,
        mod_key: &T::AccountId,
        changeset: ModuleChangeset<T>,
    ) -> Result<u16, sp_runtime::DispatchError> {
        // --- Get The Next Uid ---
        let uid: u16 = N::<T>::get(net_key);
        log::debug!("append_module( net_key: {net_key:?} | mod_key: {mod_key:?})");

        // -- Initialize All Storages ---
        StorageHandler::initialize_all::<T>(net_key, mod_key)?;
        // Make sure this overwrites the defaults (keep it second)
        changeset.apply(net_key, mod_key.clone())?;

        // --- Update The Network Module Size ---
        N::<T>::mutate(net_key, |n| *n = n.saturating_add(1));

        // --- Initilaize Stake Storage ---
        Self::increase_stake(mod_key, mod_key, 0);

        Ok(uid)
    }

    /// Replace the module under this uid.
    pub fn remove_module(
        net_key: &T::AccountId,
        mod_key: &T::AccountId,
        deregister_subnet_if_empty: bool,
    ) -> DispatchResult {
        // 1. Check if network has any modules
        let n = N::<T>::get(mod_key);
        if n == 0 {
            return Ok(());
        }

        log::debug!(
            "remove_module( net_key: {:?} | mod_key : {:?} ) ",
            net_key,
            mod_key
        );

        // --- Remove All Module Related Storage ---
        StorageHandler::remove_all::<T>(net_key, mod_key)?;
        <T as SubnetEmissionApi<T::AccountId>>::clear_module_includes(
            netuid,
            uid,
            replace_uid,
            &module_key,
            &replace_key,
        )?;

        // --- Delete Rate Limit ---
        RootNetWeightCalls::<T>::remove(uid);

        // --- Delete Global-Module Storage ---
        // This will remove storages if the module is only registered on this network.
        // So the values are not "just hanging around" in the storage. Without module actually being
        // registered on any subnet.
        if Uids::<T>::iter().all(|(_, key, _)| key != module_key) {
            ValidatorFeeConfig::<T>::remove(&module_key);
            Self::remove_stake_from_storage(&module_key);
        }

        // 9. Update network size
        let module_count = N::<T>::mutate(netuid, |v| {
            *v = v.saturating_sub(1);
            *v
        });

        // 11. Remove subnet if empty
        if deregister_subnet_if_empty && module_count == 0 {
            Self::remove_subnet(netuid);
        }

        Ok(())
    }
}
