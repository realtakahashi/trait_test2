#![cfg_attr(not(feature = "std"), no_std)]

// use ink_lang as ink;

//#[ink::contract]
#[openbrush::contract]
mod flipper {
    use contract_helper::common::common_logics;
    use contract_helper::traits::*;
    use contract_helper::traits::contract_base::contract_base::*;
    use contract_helper::traits::errors::contract_error::*;
    use ink_prelude::string::{String, ToString};
    use ink_prelude::vec::Vec;
    use ink_storage::traits::{PackedLayout, SpreadAllocate, SpreadLayout, StorageLayout};
    use openbrush::{contracts::ownable::*, modifiers, storage::Mapping, traits::Storage};

    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Flipper {
        /// Stores a single `bool` value on the storage.
        value: bool,
    }
    
    const COMMAND_LIST: [&str; 3] = ["set_dao_address", "add_proposal", "change_proposal_status"];

    impl ContractBase for Flipper {
        #[ink(message)]
        fn execute_interface(
            &mut self,
            _target_contract: String,
            command: String,
            parameters_csv: String,
        ) -> core::result::Result<(), ContractBaseError> {
            if COMMAND_LIST
                .iter()
                .filter(|item| *item == &command)
                .collect::<Vec<&&str>>()
                .len()
                == 0
            {
                return Err(ContractBaseError::CommnadNotFound);
            };
            self._execute_interface(command, parameters_csv)
        }

        #[inline]
        fn _execute_interface(
            &mut self,
            command: String,
            parameters_csv: String,
        ) -> core::result::Result<(), ContractBaseError> {
            let vec_of_parameters: Vec<String> = match parameters_csv.find(',') {
                Some(_index) => parameters_csv
                    .split(',')
                    .map(|col| col.to_string())
                    .collect(),
                None => {
                    let mut rec: Vec<String> = Vec::new();
                    rec.push(parameters_csv);
                    rec
                }
            };
            self._function_calling_switch(command, vec_of_parameters)
        }

        #[inline]
        fn _function_calling_switch(
            &mut self,
            command: String,
            vec_of_parameters: Vec<String>,
        ) -> core::result::Result<(), ContractBaseError> {
            // match command.as_str() {
            //     "set_dao_address" => self._set_dao_address(vec_of_parameters), // dao_coreへのインストールコマンドで必ず呼ぶ
            //     "add_proposal" => self._add_proposal(vec_of_parameters),
            //     "change_proposal_status" => self._change_proposal_status(vec_of_parameters),
            //     _ => Err(ContractBaseError::CommnadNotFound),
            // }
            Ok(())
        }
    }

    impl Flipper {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: bool) -> Self {
            Self { value: init_value }
        }

        /// Constructor that initializes the `bool` value to `false`.
        ///
        /// Constructors can delegate to other constructors.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// A message that can be called on instantiated contracts.
        /// This one flips the value of the stored `bool` from `true`
        /// to `false` and vice versa.
        #[ink(message)]
        pub fn flip(&mut self) -> bool {
            self.value = !self.value;
            self.value
        }

        /// Simply returns the current value of our `bool`.
        #[ink(message)]
        pub fn get(&self) -> bool {
            self.value
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// Imports `ink_lang` so we can use `#[ink::test]`.
        use ink_lang as ink;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let flipper = Flipper::default();
            assert_eq!(flipper.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut flipper = Flipper::new(false);
            assert_eq!(flipper.get(), false);
            flipper.flip();
            assert_eq!(flipper.get(), true);
        }
    }
}
