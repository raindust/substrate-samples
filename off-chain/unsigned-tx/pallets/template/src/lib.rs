#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs
use frame_support::{
    debug, decl_module, decl_error, decl_storage, decl_event,
    dispatch,
};
use frame_system::{
    self as system, ensure_none,
    offchain::{
        CreateSignedTransaction,
    },
};
use sp_core::crypto::KeyTypeId;
use sp_std::prelude::*;

use core::convert::TryInto;
use frame_system::offchain::SubmitTransaction;
use sp_runtime::transaction_validity::{TransactionSource, TransactionValidity, InvalidTransaction, ValidTransaction, TransactionPriority};
use frame_support::traits::Get;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");

// todo use SendUnsignedTransaction instead

/// The pallet's configuration trait.
pub trait Trait: system::Trait + CreateSignedTransaction<Call<Self>> {
    /// The overarching event type.
    type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;

    type Call: From<Call<Self>>;
    /// The type to sign and send transactions.
    type UnsignedPriority: Get<TransactionPriority>;
}

// This pallet's storage items.
decl_storage! {
	// It is important to update your storage name so that your pallet's
	// storage items are isolated from other pallets.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {
		Numbers get(fn numbers): map hasher(blake2_128_concat) u64 => u64;
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		/// Just a dummy event.
		/// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		/// To emit this event, we call the deposit function, from our runtime functions
		NumberAppended(Option<AccountId>, u64, u64),
	}
);

decl_error!(
    pub enum Error for Module<T: Trait> {
        SendUnsignedError
    }
);

// The pallet's dispatchable functions.
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

		#[weight = 10_000]
		pub fn save_number(origin, height: u64, number: u64) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let _ = ensure_none(origin)?;

			Numbers::insert(height, number);
			Self::deposit_event(RawEvent::NumberAppended(None, height, number));

			Ok(())
		}

		fn offchain_worker(block_number: T::BlockNumber) {
			debug::info!("Entering off-chain workers");

	        let result = Self::submit_number(block_number);
	        if let Err(e) = result {
	            debug::error!("Error: {:?}", e);
	        }
		}

	}
}

impl<T: Trait> Module<T> {
    fn submit_number(block_number: T::BlockNumber) -> Result<(), Error<T>> {
        let height: u64 = block_number.try_into().ok().unwrap() as u64;
        let latest = if height > 0 {
            Self::numbers((height - 1) as u64)
        } else {
            0
        };

        let new: u64 = latest.saturating_add((height + 1).saturating_pow(2));

        let call = Call::save_number(height, new);
        SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into()).map_err(|e| {
            debug::error!("failed in submit_number: {:?}", e);
            <Error<T>>::SendUnsignedError
        })
    }
}

impl<T: Trait> frame_support::unsigned::ValidateUnsigned for Module<T> {
    type Call = Call<T>;

    fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
        #[allow(unused_variables)]
        if let Call::save_number(height, number) = call {
            debug::native::info!("off-chain send unsigned at height {} with number {}",
                height, number);

            ValidTransaction::with_tag_prefix("offchain-demo")
                .priority(T::UnsignedPriority::get())
                .and_provides([b"save_number"])
                .longevity(3)
                .propagate(true)
                .build()
        } else {
            InvalidTransaction::Call.into()
        }
    }
}
