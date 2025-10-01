use crate::*;

use near_sdk::ext_contract;

#[allow(dead_code)]
#[ext_contract(intents_contract)]
trait IntentsContract {
    fn mt_transfer(&self, receiver_id: AccountId, token_id: String, amount: U128);
}