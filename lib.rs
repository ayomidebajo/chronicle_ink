#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
pub mod chronicle_contracts {
    use ink::prelude::vec::Vec;
    use scale::{Decode, Encode};

    #[ink(storage)]
    pub struct ChronicleContracts {
        cars: Vec<CarData>,
        owners: Vec<AccountId>,
    }

    #[derive(Encode, Decode, Debug, PartialEq, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct CarData {
        model: String,
        vin: String,
        log: Vec<(String, String)>,
        owner: AccountId,
    }

    impl ChronicleContracts {
        #[ink(constructor)]
        pub fn new() -> Self {
            let cars: Vec<CarData> = Vec::new();
            let owners: Vec<AccountId> = Vec::new();
            Self { cars, owners }
        }

        #[ink(message)]
        pub fn get_owners(&self) -> Vec<AccountId> {
            self.owners.clone()
        }
    }
}
