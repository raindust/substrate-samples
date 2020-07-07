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
use sp_std::str;

use core::{convert::TryInto, fmt};
use sp_runtime::{
    offchain::{storage::StorageValueRef},
};
use codec::{Decode, Encode};
use alt_serde::{Deserialize, Deserializer};
use sp_std::fmt::Formatter;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"demo");

#[serde(crate = "alt_serde")]
#[derive(Encode, Decode, Deserialize)]
struct GithubInfo {
    #[serde(deserialize_with = "de_string_to_bytes")]
    login: Vec<u8>,
    #[serde(deserialize_with = "de_string_to_bytes")]
    blog: Vec<u8>,
    public_repos: u32,
}

pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
    where D: Deserializer<'de> {
    let s: &str = Deserialize::deserialize(de)?;
    Ok(s.as_bytes().to_vec())
}

impl fmt::Debug for GithubInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ login: {}, blog: {}, public_repos: {} }}",
            str::from_utf8(&self.login).map_err(|_| fmt::Error)?,
            str::from_utf8(&self.blog).map_err(|_| fmt::Error)?,
            &self.public_repos,
        )
    }
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

	        let result = Self::fetch_github_info(block_number);
	        if let Err(e) = result {
	            debug::error!("Error: {:?}", e);
	        }
		}

	}
}

impl<T: Trait> Module<T> {
    fn fetch_github_info(block_number: T::BlockNumber) -> Result<(), Error<T>> {

        Ok(())
    }
}

