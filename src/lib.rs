// Copyright 2018-2019 Parity Technologies (UK) Ltd.
// This file is part of ink!.
//
// ink! is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// ink! is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with ink!.  If not, see <http://www.gnu.org/licenses/>.

//! Definitions for environment types for contracts targeted at a
//! substrate chain with the default `node-runtime` configuration.

#![cfg_attr(not(any(test, feature = "test-env")), no_std)]

use core::{
    array::TryFromSliceError,
    convert::TryFrom,
};

use parity_codec::{Encode, Decode};

/// Contract environment types defined in substrate node-runtime
pub enum NodeRuntimeTypes {}

/// The default SRML address type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Encode, Decode)]
pub struct AccountId([u8; 32]);

impl From<[u8; 32]> for AccountId {
    fn from(address: [u8; 32]) -> AccountId {
        AccountId(address)
    }
}

impl<'a> TryFrom<&'a [u8]> for AccountId {
    type Error = TryFromSliceError;

    fn try_from(bytes: &'a [u8]) -> Result<AccountId, TryFromSliceError> {
        let address = <[u8; 32]>::try_from(bytes)?;
        Ok(AccountId(address))
    }
}

/// The default SRML balance type.
pub type Balance = u128;

/// The default SRML hash type.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Encode, Decode)]
pub struct Hash([u8; 32]);

impl From<[u8; 32]> for Hash {
    fn from(hash: [u8; 32]) -> Hash {
        Hash(hash)
    }
}

impl<'a> TryFrom<&'a [u8]> for Hash {
    type Error = TryFromSliceError;

    fn try_from(bytes: &'a [u8]) -> Result<Hash, TryFromSliceError> {
        let hash = <[u8; 32]>::try_from(bytes)?;
        Ok(Hash(hash))
    }
}

/// The default SRML moment type.
pub type Moment = u64;

impl ink_core::env::EnvTypes for NodeRuntimeTypes {
    type AccountId = AccountId;
    type Balance = Balance;
    type Hash = Hash;
    type Moment = Moment;
}

#[cfg(test)]
mod tests {
    use super::*;
    use srml_contract::AccountIdOf;
    use node_runtime::Runtime;
    use parity_codec::{Encode, Decode};
    use quickcheck_macros::quickcheck;

    #[derive(Debug, Clone)]
    struct ContractAccountId(AccountId);

    impl quickcheck::Arbitrary for ContractAccountId {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let mut res = [0u8; core::mem::size_of::<Self>()];
			g.fill_bytes(&mut res[..]);
            ContractAccountId(AccountId(res)) 
        }
    }

    #[quickcheck]
    fn account_id(value: ContractAccountId) {
        let contract_encoded = Encode::encode(&value.0);
        let runtime_decoded: AccountIdOf<Runtime> = Decode::decode(&mut contract_encoded.as_slice())
            .expect("Should be decodable into node_runtime type");
        let runtime_encoded = Encode::encode(&runtime_decoded);
        let contract_decoded = Decode::decode(&mut runtime_encoded.as_slice())
            .expect("Should be decodable into contract env type");
        assert_eq!(value.0, contract_decoded)
    }

    // #[test]
    // pub fn balance() {
    //     let x: <NodeRuntimeTypes as ink_core::env::EnvTypes>::Balance = ();
    // }

    // #[test]
    // pub fn hash() {
    //     let x: <NodeRuntimeTypes as ink_core::env::EnvTypes>::Hash = ();
    // }

    // #[test]
    // pub fn moment() {
    //     let x: <NodeRuntimeTypes as ink_core::env::EnvTypes>::Moment = ();
    // }
}
