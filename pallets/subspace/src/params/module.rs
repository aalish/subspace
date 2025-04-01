use crate::*;
use scale_info::TypeInfo;

#[derive(Decode, Encode, PartialEq, Eq, Clone, Debug, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct ModuleParams<T: Config> {
    pub fees: ValidatorFees,
    pub metadata: Option<Vec<u8>>,
    pub _pd: PhantomData<T>,
}

#[derive(Debug)]
pub struct ModuleChangeset<T: Config> {
    pub fees: Option<ValidatorFees>,
    pub metadata: Option<Vec<u8>>,
    pub _pd: PhantomData<T>,
}

impl<T: Config> ModuleChangeset<T> {
    #[must_use]
    pub fn new(fees: ValidatorFees, metadata: Option<Vec<u8>>) -> Self {
        Self {
            fees: Some(fees),
            metadata,
            _pd: PhantomData,
        }
    }

    #[deny(unused_variables)]
    #[must_use]
    pub fn update(
        params: &ModuleParams<T>,
        fees: Option<ValidatorFees>,
        metadata: Option<Vec<u8>>,
    ) -> Self {
        let ModuleParams {
            fees: _,
            metadata: _,
            _pd: _,
        } = params;

        Self {
            fees,
            metadata,
            _pd: PhantomData,
        }
    }

    #[deny(unused_variables)]
    pub fn validate(&self) -> Result<(), sp_runtime::DispatchError> {
        let Self {
            fees,
            metadata,
            _pd: _,
        } = self;
        if let Some(fees) = fees {
            ModuleValidator::validate_fees::<T>(fees)?;
        }

        if let Some(metadata) = metadata {
            ModuleValidator::validate_metadata::<T>(metadata)?;
        }

        Ok(())
    }

    #[deny(unused_variables)]
    pub fn apply(
        self,
        net_key: &T::AccountId,
        mod_key: T::AccountId,
    ) -> Result<(), sp_runtime::DispatchError> {
        self.validate()?;

        let Self {
            fees,
            metadata,
            _pd: _,
        } = self;

        if let Some(new_fees) = fees {
            ValidatorFeeConfig::<T>::insert(&mod_key, new_fees);
        }

        if let Some(new_metadata) = metadata {
            Metadata::<T>::insert(net_key, &mod_key, new_metadata);
        }

        Pallet::<T>::deposit_event(Event::ModuleUpdated(net_key.clone(), mod_key));
        Ok(())
    }
}

pub struct ModuleValidator;

impl ModuleValidator {
    pub fn validate_metadata<T: Config>(metadata: &[u8]) -> Result<(), sp_runtime::DispatchError> {
        ensure!(!metadata.is_empty(), Error::<T>::InvalidModuleMetadata);
        ensure!(metadata.len() <= 120, Error::<T>::ModuleMetadataTooLong);
        core::str::from_utf8(metadata).map_err(|_| Error::<T>::InvalidModuleMetadata)?;
        Ok(())
    }

    pub fn validate_fees<T: Config>(fees: &ValidatorFees) -> Result<(), sp_runtime::DispatchError> {
        fees.validate::<T>().map_err(|_| Error::<T>::InvalidMinDelegationFee)?;
        Ok(())
    }
}

impl<T: Config> Pallet<T> {
    pub fn module_params(net_key: &T::AccountId, key: &T::AccountId) -> ModuleParams<T> {
        ModuleParams {
            fees: ValidatorFeeConfig::<T>::get(key),
            metadata: Metadata::<T>::get(net_key, key),
            _pd: PhantomData,
        }
    }
}
