use hex::{decode, encode};
use near_sdk::{
    env::{self, block_timestamp},
    near, require,
    store::{IterableMap, IterableSet},
    AccountId, Gas, NearToken, PanicOnDefault, Promise,
    json_types::U128,
};

use dcap_qvl::{verify, QuoteCollateralV3};
use intents::intents_contract;
mod collateral;
mod intents;

const INTENTS_GAS: Gas = Gas::from_tgas(50);
const INTENTS_ATTACHED_DEPOSIT: NearToken = NearToken::from_yoctonear(1);

#[near(serializers = [json, borsh])]
#[derive(Clone)]
pub struct Worker {
    checksum: String,
    codehash: String,
}

#[near(contract_state)]
#[derive(PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub approved_codehashes: IterableSet<String>,
    pub worker_by_account_id: IterableMap<AccountId, Worker>,
    pub intents_contract_id: AccountId,
}

#[near]
impl Contract {
    #[init]
    #[private]
    pub fn init(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            approved_codehashes: IterableSet::new(b"a"),
            worker_by_account_id: IterableMap::new(b"b"),
            intents_contract_id: "intents.near".parse().unwrap(),
        }
    }

    // helpers for method access control

    pub fn approve_codehash(&mut self, codehash: String) {
        // !!! UPGRADE TO YOUR METHOD OF MANAGING APPROVED WORKER AGENT CODEHASHES !!!
        self.require_owner();
        self.approved_codehashes.insert(codehash);
    }

    // register args see: https://github.com/mattlockyer/based-agent-template/blob/main/pages/api/register.js

    pub fn register_agent(
        &mut self,
        quote_hex: String,
        collateral: String,
        checksum: String,
        tcb_info: String,
    ) -> bool {
        let collateral = collateral::get_collateral(collateral);
        let quote = decode(quote_hex).unwrap();
        let now = block_timestamp() / 1000000000;
        let result = verify::verify(&quote, &collateral, now).expect("report is not verified");
        let report = result.report.as_td10().unwrap();
        let report_data = format!("{}", String::from_utf8_lossy(&report.report_data));

        // verify the predecessor matches the report data
        require!(
            env::predecessor_account_id() == report_data,
            format!("predecessor_account_id != report_data: {}", report_data)
        );

        let rtmr3 = encode(report.rt_mr3.to_vec());
        let (shade_agent_api_image, shade_agent_app_image) =
            collateral::verify_codehash(tcb_info, rtmr3);

        // verify the code hashes are approved
        require!(self.approved_codehashes.contains(&shade_agent_api_image));
        require!(self.approved_codehashes.contains(&shade_agent_app_image));

        let predecessor = env::predecessor_account_id();
        self.worker_by_account_id.insert(
            predecessor,
            Worker {
                checksum,
                codehash: shade_agent_app_image,
            },
        );

        true
    }

    pub fn trade(
        &mut self,
        receiver_id: AccountId,
        token_id: String,
        amount: U128,
    ) -> Promise {
        // self.require_approved_codehash();

        intents_contract::ext(self.intents_contract_id.clone())
            .with_static_gas(INTENTS_GAS)
            .with_attached_deposit(INTENTS_ATTACHED_DEPOSIT)
            .mt_transfer(receiver_id, token_id, amount)
    }

    // views

    pub fn get_agent(&self, account_id: AccountId) -> Worker {
        self.worker_by_account_id
            .get(&account_id)
            .expect("no worker found")
            .to_owned()
    }

    // only for contract methods

    fn require_owner(&mut self) {
        require!(env::predecessor_account_id() == self.owner_id);
    }

    /// will throw on client if worker agent is not registered with a codehash in self.approved_codehashes
    fn require_approved_codehash(&mut self) {
        let worker = self.get_agent(env::predecessor_account_id());
        require!(self.approved_codehashes.contains(&worker.codehash));
    }
}
