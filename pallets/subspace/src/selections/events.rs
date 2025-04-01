use frame_support::pallet_macros::pallet_section;

#[pallet_section]
pub mod events {
    #[pallet::event]
    #[pallet::generate_deposit(pub fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event created when a new network is added
        NetworkAdded(T::AccountId),
        /// Event created when a network is removed
        NetworkRemoved(u16),
        /// Event created when stake has been transferred from the coldkey account onto the key
        /// staking account
        StakeAdded(T::AccountId, T::AccountId, u64),
        /// Event created when stake has been removed from the key staking account onto the coldkey
        /// account
        StakeRemoved(T::AccountId, T::AccountId, u64),
        /// Event created when a caller successfully sets their weights on a subnetwork
        WeightsSet(u16, u16),
        /// Event created when a new module account has been registered to the chain
        ModuleRegistered(T::AccountId, T::AccountId),
        /// Event created when a module account has been deregistered from the chain
        ModuleDeregistered(T::AccountId, T::AccountId),
        /// Event created when the module's updated information is added to the network
        ModuleUpdated(T::AccountId, T::AccountId),
        // Parameter Updates
        /// Event created when global parameters are updated
        GlobalParamsUpdated(GlobalParams<T>),
        /// Event created when subnet parameters are updated
        SubnetParamsUpdated(T::AccountId),
    }
}
