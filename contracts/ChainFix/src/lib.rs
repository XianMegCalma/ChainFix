#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Symbol, String,
};

#[contracttype]
#[derive(Clone)]
pub struct MaintenanceRequest {
    pub tenant: Address,
    pub landlord: Address,
    pub worker: Address,
    pub issue: String,
    pub payment_amount: i128,
    pub completed: bool,
    pub confirmed: bool,
}

#[contracttype]
pub enum RequestKey {
    Request(u32),
}

#[contract]
pub struct ChainFixDormContract;

#[contractimpl]
impl ChainFixDormContract {

    // Create a maintenance request
    pub fn create_request(
        env: Env,
        request_id: u32,
        tenant: Address,
        landlord: Address,
        worker: Address,
        issue: String,
        payment_amount: i128,
    ) {

        tenant.require_auth();

        let request = MaintenanceRequest {
            tenant,
            landlord,
            worker,
            issue,
            payment_amount,
            completed: false,
            confirmed: false,
        };

        env.storage().instance().set(
            &RequestKey::Request(request_id),
            &request,
        );
    }

    // Worker marks repair as completed
    pub fn mark_completed(
        env: Env,
        request_id: u32,
        worker: Address,
    ) {

        worker.require_auth();

        let key = RequestKey::Request(request_id);

        let mut request: MaintenanceRequest =
            env.storage().instance().get(&key).unwrap();

        if request.worker != worker {
            panic!("Unauthorized worker");
        }

        request.completed = true;

        env.storage().instance().set(&key, &request);
    }

    // Tenant confirms repair completion
    pub fn confirm_completion(
        env: Env,
        request_id: u32,
        tenant: Address,
    ) {

        tenant.require_auth();

        let key = RequestKey::Request(request_id);

        let mut request: MaintenanceRequest =
            env.storage().instance().get(&key).unwrap();

        if request.tenant != tenant {
            panic!("Unauthorized tenant");
        }

        if !request.completed {
            panic!("Repair not completed yet");
        }

        request.confirmed = true;

        env.storage().instance().set(&key, &request);

        // Simulated escrow release event
        env.events().publish(
            (symbol_short!("PAYMENT"),),
            request.payment_amount,
        );
    }

    // View request details
    pub fn get_request(
        env: Env,
        request_id: u32,
    ) -> MaintenanceRequest {

        env.storage()
            .instance()
            .get(&RequestKey::Request(request_id))
            .unwrap()
    }
}