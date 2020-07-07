#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs
use frame_support::{
    debug, decl_module, decl_error,
};
use frame_system::{
    self as system,
};
use sp_core::crypto::KeyTypeId;
use sp_std::prelude::*;

use core::convert::TryInto;
use sp_runtime::{
    offchain::{storage::StorageValueRef},
};
use codec::{Decode, Encode};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");

#[derive(Debug, Encode, Decode)]
struct CurrentNumber {
    number: u64,
}

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
}

decl_error!(
    pub enum Error for Module<T: Trait> {
        AlreadyFetched
    }
);

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		fn offchain_worker(block_number: T::BlockNumber) {
			debug::info!("Entering off-chain workers");

	        let result = Self::save_local_number(block_number);
	        if let Err(e) = result {
	            debug::error!("Error: {:?}", e);
	        }
		}

	}
}

impl<T: Trait> Module<T> {
    fn save_local_number(block_number: T::BlockNumber) -> Result<(), Error<T>> {
        let height: u64 = block_number.try_into().ok().unwrap() as u64;

        let num_store = StorageValueRef::persistent(b"local-storage::numbers");
        let lock = StorageValueRef::persistent(b"local-storage::lock");

        if let Some(Some(number)) = num_store.get::<CurrentNumber>() {
            debug::info!("~~ Current number is {}", number.number);
        }

        let res: Result<Result<bool, bool>, Error<T>> = lock.mutate(|s:Option<Option<bool>>| {
            match s {
                // `s` can be one of the following:
                //   `None`: the lock has never been set. Treated as the lock is free
                //   `Some(None)`: unexpected case, treated it as AlreadyFetch
                //   `Some(Some(false))`: the lock is free
                //   `Some(Some(true))`: the lock is held
                None | Some(Some(false)) => Ok(true),
                _ => Err(Error::<T>::AlreadyFetched),
            }
        });

        if let Ok(Ok(true)) = res {
            let new_num = CurrentNumber {number: height};
            num_store.set(&new_num);

            lock.set(&false);
        }

        Ok(())
    }
}

