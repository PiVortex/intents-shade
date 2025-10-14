use crate::*;

use near_sdk::ext_contract;

#[allow(dead_code)]
#[ext_contract(intents_contract)]
trait IntentsContract {
    fn ft_transfer_call(receiver_id: AccountId, amount: U128, msg: String);
    fn mt_transfer(receiver_id: AccountId, token_id: String, amount: U128);
    fn ft_withdraw(token: AccountId, receiver_id: AccountId, amount: U128);
}