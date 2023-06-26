#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod erc20 {

    use ink::{storage::Mapping, primitives::AccountId};

    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<AccountId, AccountId>,
    }

    #[ink(event)]
    pub struct Transfer {
        ///name
        ///symbol
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        InsufficientAllowance,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
        #[ink(constructor)]
        pub fn new() -> self {}

        #[ink(message)]
        pub fn balance_of(&self, owner:AccountId) -> Balance {
            self.balance_of_impl(&owner)
        }
        #[incline]
        pub fn balance_of_impl(&self, owner:AccountId) -> Balance{
            self.get(owner).unwrap_or_default()
        }

        #[ink(message)]
        pub fn allowance(&mut self,owner:AccountId,spender:AccountId) -> Balance{
            self.allowance_impl((&owner,&spender))
        }
        #[incline]
        pub fn allowance_impl(&mut self,owner:AccountId, spender:AccountId)-> Balance {
            self.get((owner,spender)).unwrap_or_default()
        }



        #[ink(message)]
        pub fn total_supply() {}
        #[ink(message)]
        pub fn transfer() {}
        #[ink(message)]
        pub fn transfer_from_to() {}
        #[ink(message)]
        pub fn transfer_from() {}
        #[ink(message)]
        pub fn approve() {}
    }
}
