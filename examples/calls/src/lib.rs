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

#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang2 as ink;
use ink_types_node_runtime::{calls, AccountIndex, NodeRuntimeTypes};

#[ink::contract(version = "0.1.0", env = NodeRuntimeTypes)]
mod runtime_calls {
    /// This simple dummy contract dispatches substrate runtime calls
    #[ink(storage)]
    struct RuntimeCalls {}

    impl RuntimeCalls {
        #[ink(constructor)]
        fn new(&mut self) {}

        /// Dispatches a `transfer` call to the Balances srml module
        #[ink(message)]
        fn balance_transfer(&self, dest: AccountId, value: Balance) {
            let dest_addr = calls::Address::Id(dest);
            let transfer_call =
                calls::Balances::<NodeRuntimeTypes, AccountIndex>::transfer(dest_addr, value);
            self.env().invoke_runtime(&transfer_call);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn dispatches_balances_call() {
            let calls = RuntimeCalls::new();
            let alice = AccountId::from([0x0; 32]);
            // assert_eq!(calls.env().dispatched_calls().into_iter().count(), 0);
            calls.balance_transfer(alice, 10000);
            // assert_eq!(calls.env().dispatched_calls().into_iter().count(), 1);
        }
    }
}
