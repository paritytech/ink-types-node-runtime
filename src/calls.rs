// Copyright 2019 Parity Technologies (UK) Ltd.
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

use core::convert::TryInto;
use ink_core::env2::EnvTypes;
use scale::{Decode, Encode, Error, Input, Output};

#[cfg_attr(feature = "std", derive(Clone, PartialEq, Eq))]
pub enum Address<T: EnvTypes, AccountIndex> {
    Id(T::AccountId),
    Index(AccountIndex),
}

/// Returns `b` if `b` is greater than `a` and otherwise `None`.
fn greater_than_or_err<T: PartialOrd>(a: T, b: T) -> Result<T, Error> {
    if a < b {
        Ok(b)
    } else {
        Err("Invalid range".into())
    }
}

/// Decode implementation copied over from Substrate `Address` that can be found [here](substrate-address).
///
/// # Note
/// This implementation MUST be kept in sync with substrate, tests below will ensure that.
///
/// [substrate-address]: https://github.com/paritytech/substrate/blob/ec62d24c602912f07bbc416711376d9b8e5782c5/srml/indices/src/address.rs#L61
impl<T, AccountIndex> Decode for Address<T, AccountIndex>
where
    T: EnvTypes,
    AccountIndex: Decode + From<u32> + PartialOrd + Copy + Clone,
{
    fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
        Ok(match input.read_byte()? {
            x @ 0x00..=0xef => Address::Index(AccountIndex::from(x as u32)),
            0xfc => Address::Index(AccountIndex::from(greater_than_or_err(
                0xef,
                u16::decode(input)?,
            )? as u32)),
            0xfd => Address::Index(AccountIndex::from(greater_than_or_err(
                0xffff,
                u32::decode(input)?,
            )?)),
            0xfe => Address::Index(greater_than_or_err(
                0xffffffffu32.into(),
                Decode::decode(input)?,
            )?),
            0xff => Address::Id(Decode::decode(input)?),
            _ => return Err("Invalid address variant".into()),
        })
    }
}

/// Encode implementation copied over from Substrate `Address` that can be found [here](substrate-address).
///
/// # Note
/// This implementation MUST be kept in sync with substrate, tests below will ensure that.
///
/// [substrate-address]: https://github.com/paritytech/substrate/blob/ec62d24c602912f07bbc416711376d9b8e5782c5/srml/indices/src/address.rs#L83
impl<T, AccountIndex> Encode for Address<T, AccountIndex>
where
    T: EnvTypes,
    AccountIndex: Encode + TryInto<u32> + Copy + Clone,
{
    fn encode_to<O: Output>(&self, dest: &mut O) {
        match *self {
            Address::Id(ref i) => {
                dest.push_byte(255);
                dest.push(i);
            }
            Address::Index(i) => {
                let maybe_u32: Result<u32, _> = i.try_into();
                if let Ok(x) = maybe_u32 {
                    if x > 0xffff {
                        dest.push_byte(253);
                        dest.push(&x);
                    } else if x >= 0xf0 {
                        dest.push_byte(252);
                        dest.push(&(x as u16));
                    } else {
                        dest.push_byte(x as u8);
                    }
                } else {
                    dest.push_byte(254);
                    dest.push(&i);
                }
            }
        }
    }
}

#[derive(Encode)]
#[cfg_attr(feature = "std", derive(Decode, Clone, PartialEq, Eq))]
pub enum Balances<T: EnvTypes, AccountIndex> {
    #[allow(non_camel_case_types)]
    transfer(Address<T, AccountIndex>, #[codec(compact)] T::Balance),
    #[allow(non_camel_case_types)]
    set_balance(
        Address<T, AccountIndex>,
        #[codec(compact)] T::Balance,
        #[codec(compact)] T::Balance,
    ),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{calls, AccountIndex, Call, NodeRuntimeTypes};

    use node_runtime::{self, Runtime};
    use pallet_indices::address;
    use scale::{Decode, Encode};

    #[test]
    fn account_index_serialization() {
        let account_index = 0u32;

        let ink_address: Address<NodeRuntimeTypes, u32> = Address::Index(account_index.into());
        let pallet_address: address::Address<[u8; 32], u32> =
            address::Address::Index(account_index);

        let ink_encoded = ink_address.encode();
        let pallet_encoded = pallet_address.encode();

        assert_eq!(pallet_encoded, ink_encoded);

        let srml_decoded: address::Address<[u8; 32], u32> =
            Decode::decode(&mut ink_encoded.as_slice())
                .expect("Account Index decodes to srml Address");
        let srml_encoded = srml_decoded.encode();
        let ink_decoded: Address<NodeRuntimeTypes, u32> =
            Decode::decode(&mut srml_encoded.as_slice())
                .expect("Account Index decodes back to ink type");

        assert!(ink_address == ink_decoded);
    }

    #[test]
    fn account_id_serialization() {
        let account_id = [0u8; 32];

        let ink_address = Address::Id(account_id.into());
        let srml_address: address::Address<[u8; 32], u32> = address::Address::Id(account_id);

        let ink_encoded = ink_address.encode();
        let srml_encoded = srml_address.encode();

        assert_eq!(srml_encoded, ink_encoded);

        let srml_decoded: address::Address<[u8; 32], u32> =
            Decode::decode(&mut ink_encoded.as_slice())
                .expect("Account Id decodes to srml Address");
        let srml_encoded = srml_decoded.encode();
        let ink_decoded: Address<NodeRuntimeTypes, u32> =
            Decode::decode(&mut srml_encoded.as_slice())
                .expect("Account Id decodes decodes back to ink type");

        assert!(ink_address == ink_decoded);
    }

    #[test]
    fn call_balance_transfer() {
        let balance = 10_000;
        let account_index = 0;

        let contract_address = calls::Address::Index(account_index);
        let contract_transfer =
            calls::Balances::<NodeRuntimeTypes, AccountIndex>::transfer(contract_address, balance);
        let contract_call = Call::Balances(contract_transfer);

        let srml_address = address::Address::Index(account_index);
        let srml_transfer = node_runtime::BalancesCall::<Runtime>::transfer(srml_address, balance);
        let srml_call = node_runtime::Call::Balances(srml_transfer);

        let contract_call_encoded = contract_call.encode();
        let srml_call_encoded = srml_call.encode();

        assert_eq!(srml_call_encoded, contract_call_encoded);

        let srml_call_decoded: node_runtime::Call =
            Decode::decode(&mut contract_call_encoded.as_slice())
                .expect("Balances transfer call decodes to srml type");
        let srml_call_encoded = srml_call_decoded.encode();
        let contract_call_decoded: Call = Decode::decode(&mut srml_call_encoded.as_slice())
            .expect("Balances transfer call decodes back to contract type");
        assert!(contract_call == contract_call_decoded);
    }
}
