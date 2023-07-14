#![cfg_attr(not(feature = "std"), no_std, no_main)]


/*
    Smart contract de un token ERC20
*/

#[ink::contract]
mod erc20 {

    use ink::storage::Mapping;

    //Estructura basica de ERC20
    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
        ///name
        ///symbol
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }


    //Evento de transferencias
    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        value: Balance,
    }

    //Evento de aprobacion
    #[ink(event)]
    pub struct Approval {
        #[ink(topic)]
        owner: AccountId,
        #[ink(topic)]
        spender: AccountId,
        value: Balance,
    }


    //Representar errores basicos
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        InsufficientAllowance,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    //Implementaciones de las funciones basicas del smart contract token ERC20
    impl Erc20 {

         // Constructor del contrato (Se puede agregar mas de un constructor)
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let mut balances = Mapping::default();
            let caller = Self::env().caller();

            balances.insert(caller, &total_supply);

            Self::env().emit_event(Transfer {
                from: None,
                to: Some(caller),
                value: total_supply,
            });

            Self {
                total_supply,
                balances,
                allowances: Default::default(),
            }
        }

        // Obtiene el saldo de tokens de una cuenta
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balance_of_impl(owner)
        }

        #[inline]
        pub fn balance_of_impl(&self, owner: AccountId) -> Balance {
            self.balances.get(owner).unwrap_or_default()
        }

        // Obtiene la asignación de gasto aprobada de una cuenta a otra cuenta
        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowance_impl(&owner, &spender)
        }

        #[inline]
        fn allowance_impl(&self, owner: &AccountId, spender: &AccountId) -> Balance {
            self.allowances.get((owner, spender)).unwrap_or_default()
        }

        // Obtiene el suministro total de tokens
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        // Transfiere tokens de una cuenta a otra
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(from, to,value)
        }

        // Aprueba una asignación de gasto de tokens para otra cuenta
        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((&owner, &spender), &value);
            self.env().emit_event(Approval {
                owner,
                spender,
                value,
            });
            Ok(())
        }

        // Transfiere tokens de una cuenta a otra en nombre de la cuenta llamante
        #[ink(message)]
        pub fn transfer_from(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let caller = self.env().caller();
            let allowance = self.allowance_impl(&from, &caller);
            if allowance < value {
                return Err(Error::InsufficientAllowance);
            }
            self.transfer_from_to(from, to, value)?;
            self.allowances.insert((from, caller), &(allowance - value));
            Ok(())
        }   

        // Realiza la transferencia de tokens de una cuenta a otra
        #[ink(message)]
        pub fn transfer_from_to(
            &mut self,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> Result<()> {
            let from_balance = self.balance_of_impl(from);
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }
            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of_impl(to);
            self.balances.insert(to, &(to_balance + value));
            self.env().emit_event(Transfer {
                from: Some(from),
                to: Some(to),
                value,
            });
            Ok(())
        }
    }
}

