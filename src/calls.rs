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

use ink_core::env::EnvTypes;
use scale::{Codec, Decode, Encode};
use pallet_indices::address::Address;
use sp_runtime::traits::Member;

#[derive(Encode, Decode, Clone, PartialEq, Eq)]
pub enum Balances<T, AccountIndex>
where
    T: EnvTypes,
    T::AccountId: Member + Codec,
    AccountIndex: Member + Codec,
{
    #[allow(non_camel_case_types)]
    transfer(Address<T::AccountId, AccountIndex>, #[codec(compact)] T::Balance),
    #[allow(non_camel_case_types)]
    set_balance(
        Address<T::AccountId, AccountIndex>,
        #[codec(compact)] T::Balance,
        #[codec(compact)] T::Balance,
    ),
}

#[cfg(test)]
mod tests {
    use crate::{calls, AccountIndex, Call, NodeRuntimeTypes};

    use node_runtime::{self, Runtime};
    use pallet_indices::address;
    use scale::{Decode, Encode};


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
