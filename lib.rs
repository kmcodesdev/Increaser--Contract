#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod voter {
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Increaser {
        increase_count: i32,
        id: ink_storage::Mapping<AccountId, i32>,
    }

    impl Increaser {
        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.increase_count = Default::default();
            })
        }

        #[ink(message)]
        pub fn increase_my_number(&mut self) {
            let caller = self.env().caller();
            let id = self.get_my_increase();
            let increase = id + 1;
            self.increase_number();
            self.id.insert(caller, &(increase));
        }

    

        #[ink(message)]
        pub fn get_my_increase(&self) -> i32 {
            self.id.get(&self.env().caller()).unwrap_or_default()
        }

        #[ink(message)]
        pub fn get_total_increase(&self) -> i32 {
            self.increase_count
        }

        fn increase_number(&mut self) {
            self.increase_count = self.increase_count + 1;
        }
        
   
    }
    
}