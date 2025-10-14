use crate::*;

#[near]
impl Contract {
    pub fn deposit_to_intents(
        &mut self,
        amount: U128
    ) -> Promise {
        // self.require_approved_codehash();

        intents_contract::ext(self.intents_contract_id.clone())
            .with_static_gas(INTENTS_GAS)
            .with_attached_deposit(INTENTS_ATTACHED_DEPOSIT)
            .ft_transfer_call(self.intents_contract_id.clone(), amount, "".to_string())
    }

    pub fn trade(
        &mut self,
        receiver_id: AccountId,
        token_id: String,
        amount: U128,
    ) -> Promise {
        // self.require_approved_codehash();

        // Is there a way to restrict in the contract the token being swapped into
        // Possible with signing the intent to swap but would rather not use chain signatures here 
        // Maybe there is a function that takes a quote as an argument

        intents_contract::ext(self.intents_contract_id.clone())
            .with_static_gas(INTENTS_GAS)
            .with_attached_deposit(INTENTS_ATTACHED_DEPOSIT)
            .mt_transfer(receiver_id, token_id, amount)
    }

    pub fn withdraw_from_intents(
        &mut self,
        amount: U128
    ) -> Promise {
        // self.require_approved_codehash();

        intents_contract::ext(self.intents_contract_id.clone())
            .with_static_gas(INTENTS_GAS)
            .with_attached_deposit(INTENTS_ATTACHED_DEPOSIT)
            .ft_withdraw(self.base_token_id.clone(), self.intents_contract_id.clone(), amount)
    }

}