#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod lucky_aid {
    use ink::storage::Mapping;
    use ink::env::DefaultEnvironment;
    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;

    #[ink(storage)]
    pub struct LuckyAid {
        asset_contract: AccountId,
        claimed: Mapping<AccountId, bool>,
        base_amount: Balance,
    }

    impl LuckyAid {
        #[ink(constructor)]
        pub fn new(asset_contract: AccountId, base_amount: Balance) -> Self {
            Self {
                asset_contract,
                claimed: Mapping::default(),
                base_amount,
            }
        }

        #[ink(message)]
        pub fn claim(&mut self) -> Result<String, String> {
            let caller = self.env().caller();
            if self.claimed.get(caller).unwrap_or(false) {
                return Err(String::from("Already claimed"      .exec_input(
                    ink::env::call::ExecutionInput::new(ink::env::call::Selector::new([0x0f, 0x12, 0xdc, 0x65]))
                        .push_arg(caller)
                        .push_arg(amount_to_transfer),
                )
                .returns::<()>();

            builder
                .fire()
                .map_err(|_| String::from("Transfer failed"))?;

            if lucky {
                Ok(String::from("í¾‰ Bonus! You won 5x the reward!"))
            } else {
                Ok(String::from("í¾ˆ Claimed successfully!"))
            }
        }
    }
}
));
            }

            self.claimed.insert(caller, &true);

            let randomness = self.env().block_timestamp();
            let lucky = randomness % 10 == 0;

            let amount_to_transfer = if lucky {
                self.base_amount * 5
            } else {
                self.base_amount
            };

            let builder = ink::env::call::build_call::<DefaultEnvironment>()
                .call(self.asset_contract)
          
