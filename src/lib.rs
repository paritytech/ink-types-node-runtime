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

#![cfg_attr(not(test), no_std)]

use core::{array::TryFromSliceError, convert::TryFrom};
use parity_codec::{Decode, Encode};

pub mod calls;

/// Contract environment types defined in substrate node-runtime
#[cfg_attr(feature = "std", derive(Debug, Clone, PartialEq, Eq))]
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

/// The default SRML blocknumber type.
pub type BlockNumber = u64;

/// The default SRML AccountIndex type.
pub type AccountIndex = u32;

/// The default SRML call type.
#[derive(Encode)]
#[cfg_attr(feature = "std", derive(Decode, Debug, Clone, PartialEq, Eq))]
pub enum Call {
	#[codec(index = "5")]
	Balances(calls::Balances<NodeRuntimeTypes, AccountIndex>),
}

impl From<calls::Balances<NodeRuntimeTypes, AccountIndex>> for Call {
	fn from(balances_call: calls::Balances<NodeRuntimeTypes, AccountIndex>) -> Call {
		Call::Balances(balances_call)
	}
}

impl ink_core::env::EnvTypes for NodeRuntimeTypes {
    type AccountId = AccountId;
    type Balance = Balance;
    type Hash = Hash;
    type Moment = Moment;
    type BlockNumber = BlockNumber;
    type Call = Call;
}

#[cfg(test)]
mod tests {
    use super::*;
    use node_runtime::Runtime;
    use parity_codec::{Codec, Decode, Encode};
    use quickcheck_macros::quickcheck;
    use std::fmt::Debug;

    pub type AccountIdOf<T> = <T as srml_system::Trait>::AccountId;
    pub type MomentOf<T> = <T as srml_timestamp::Trait>::Moment;
    pub type SeedOf<T> = <T as srml_system::Trait>::Hash;

    macro_rules! impl_hash_quickcheck_arb_wrapper {
        ($inner:ident, $wrapper:ident) => {
            #[derive(Debug, Clone)]
            struct $wrapper($inner);

            impl quickcheck::Arbitrary for $wrapper {
                fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
                    let mut res = [0u8; core::mem::size_of::<Self>()];
                    g.fill_bytes(&mut res[..]);
                    $wrapper($inner(res))
                }
            }

            impl From<$wrapper> for $inner {
                fn from(x: $wrapper) -> Self {
                    x.0
                }
            }
        };
    }

    impl_hash_quickcheck_arb_wrapper!(AccountId, ContractAccountId);
    impl_hash_quickcheck_arb_wrapper!(Hash, ContractHash);

    /// Ensure that a type is compatible with its equivalent runtime type
    fn runtime_codec_roundtrip<ContractT, WrapperT, RuntimeT>(value: WrapperT)
    where
        ContractT: Codec + Debug + Eq + From<WrapperT>,
        RuntimeT: Codec,
    {
        let contract_value: ContractT = value.into();
        let contract_encoded = Encode::encode(&contract_value);
        let runtime_decoded: RuntimeT = Decode::decode(&mut contract_encoded.as_slice())
            .expect("Should be decodable into node_runtime type");
        let runtime_encoded = Encode::encode(&runtime_decoded);
        let contract_decoded: ContractT = Decode::decode(&mut runtime_encoded.as_slice())
            .expect("Should be decodable into contract env type");
        assert_eq!(contract_value, contract_decoded)
    }

    #[quickcheck]
    fn account_id(value: ContractAccountId) {
        runtime_codec_roundtrip::<AccountId, ContractAccountId, AccountIdOf<Runtime>>(value);
    }

    #[quickcheck]
    fn balance(value: Balance) {
        runtime_codec_roundtrip::<Balance, Balance, srml_contract::BalanceOf<Runtime>>(value);
    }

    #[quickcheck]
    fn hash(value: ContractHash) {
        runtime_codec_roundtrip::<Hash, ContractHash, SeedOf<Runtime>>(value);
    }

    #[quickcheck]
    pub fn moment(value: Moment) {
        runtime_codec_roundtrip::<Moment, Moment, MomentOf<Runtime>>(value);
    }
}
