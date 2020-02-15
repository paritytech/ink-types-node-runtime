#![feature(proc_macro_hygiene)]
#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0", env = NodeRuntimeTypes)]
mod calls {
    use ink_types_node_runtime::{calls as runtime_calls, AccountIndex, Call, NodeRuntimeTypes};

    /// This simple dummy contract dispatches substrate runtime calls
    #[ink(storage)]
    struct Calls {}

    impl Calls {
        #[ink(constructor)]
        fn new(&mut self) {}

        /// Dispatches a `transfer` call to the Balances srml module
        #[ink(message)]
        fn balance_transfer(&self, dest: AccountId, value: Balance) {
            let dest_addr = runtime_calls::Address::Id(dest);
            let transfer_call = Call::Balances(runtime_calls::Balances::<
                NodeRuntimeTypes,
                AccountIndex,
            >::transfer(dest_addr, value));
            let _ = self.env().invoke_runtime(&transfer_call);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn dispatches_balances_call() {
            let calls = Calls::new();
            let alice = AccountId::from([0x0; 32]);
            // assert_eq!(calls.env().dispatched_calls().into_iter().count(), 0);
            calls.balance_transfer(alice, 10000);
            // assert_eq!(calls.env().dispatched_calls().into_iter().count(), 1);
        }
    }
}
