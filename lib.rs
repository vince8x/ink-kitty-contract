#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod nft {

    use ink_prelude::{
        string::{
            String,
        },
        vec::Vec,
    };

    use ink_storage::{
        traits::SpreadAllocate,
        traits::{PackedLayout, SpreadLayout},
        Mapping,
    };

    use scale::{
        Decode,
        Encode,
    };

    use ink_env::{
        hash_encoded, hash::{Sha2x256, HashOutput}
    };

    #[derive(
        scale::Encode,
        scale::Decode,
        Eq,
        PartialEq,
        Debug,
        Clone,
        SpreadLayout,
        PackedLayout,
    )]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout,)
    )]
	pub enum Gender {
		Male,
		Female,
	}

    #[derive(
        scale::Encode,
        scale::Decode,
        Eq,
        PartialEq,
        Debug,
        Clone,
        SpreadLayout,
        PackedLayout,
    )]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink_storage::traits::StorageLayout,)
    )]
	pub struct Kitty {
		pub dna: Hash,
		pub owner: AccountId,
        pub gender: Gender,
	}

    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct NftKitties {
        kitties: Mapping<Hash, Kitty>,
    }

    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        DuplicateKitty,
		TooManyOwned,
		NoKitty,
		NotOwner,
		TransferToSelf,
		CannotConvert,
		ExceedKittyNumber,
    }

    /// Event emitted when a kitty creation occurs.
    #[ink(event)]
    pub struct Created {
        #[ink(topic)]
        kitty: Hash,
        #[ink(topic)]
        owner: AccountId,
    }

    /// Event emitted when a kitty transfer occurs.
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        #[ink(topic)]
        kitty: Hash,
    }

    impl NftKitties {
        /// Creates a new ERC-721 token contract.
        #[ink(constructor)]
        pub fn new() -> Self {
            // This call is required in order to correctly initialize the
            // `Mapping`s of our contract.
            ink_lang::utils::initialize_contract(|_| {})
        }

        /// Prints the specified string into node's debug log.
        #[ink(message)]
        pub fn debug_log(&mut self, _message: String) {
            ink_env::debug_println!("debug_log: {}", _message);
        }

        fn gen_gender(dna: &Vec<u8>) -> Result<Gender, Error> {
            let mut res = Gender::Female;
            if dna.len() % 2 == 0 {
                res = Gender::Male;
            }
            Ok(res)
        }

        fn gen_dna(dna: Vec<u8>, block_number: u32) -> Hash {
            let encodable = (block_number, dna.clone()); // Implements `scale::Encode`
            let mut output = <Sha2x256 as HashOutput>::Type::default(); // 256-bit buffer
            ink_env::hash_encoded::<Sha2x256, _>(&encodable, &mut output);
            Hash::from(output)

        }


        #[ink(message)]
        pub fn create_kitty(&mut self, owner: AccountId, dna: Vec<u8>) -> Result<(), Error> {


            let caller = self.env().caller();

            if owner != caller {
                return Err(Error::NotOwner)
            };

            let block_number = self.env().block_number();

            let gender = Self::gen_gender(&dna)?;

            let dna_hash = Self::gen_dna(dna, block_number);

            let kitty: Kitty =
				Kitty { 
                    dna: dna_hash.clone(),
                    gender: gender,
                    owner: owner.clone()
                };

            // Check if the kitty does not already exist 
            if self.kitties.contains(&kitty.dna) {
                return Err(Error::DuplicateKitty)
            }


            self.kitties.insert(kitty.dna.clone(), &kitty);


            Ok(())
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
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
        }
    }
}
