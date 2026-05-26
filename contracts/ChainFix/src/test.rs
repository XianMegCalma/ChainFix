#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

mod tests {

    use super::*;

    // Test 1: Happy Path
    #[test]
    fn test_happy_path() {

        let env = Env::default();

        let tenant = Address::generate(&env);
        let landlord = Address::generate(&env);
        let worker = Address::generate(&env);

        let contract_id = env.register_contract(None, ChainFixDormContract);
        let client = ChainFixDormContractClient::new(&env, &contract_id);

        client.create_request(
            &1,
            &tenant,
            &landlord,
            &worker,
            &String::from_str(&env, "Broken Sink"),
            &500,
        );

        client.mark_completed(&1, &worker);

        client.confirm_completion(&1, &tenant);

        let request = client.get_request(&1);

        assert_eq!(request.confirmed, true);
    }

    // Test 2: Edge Case
    #[test]
    #[should_panic(expected = "Unauthorized worker")]
    fn test_unauthorized_worker() {

        let env = Env::default();

        let tenant = Address::generate(&env);
        let landlord = Address::generate(&env);
        let worker = Address::generate(&env);
        let fake_worker = Address::generate(&env);

        let contract_id = env.register_contract(None, ChainFixDormContract);
        let client = ChainFixDormContractClient::new(&env, &contract_id);

        client.create_request(
            &1,
            &tenant,
            &landlord,
            &worker,
            &String::from_str(&env, "Broken Pipe"),
            &300,
        );

        client.mark_completed(&1, &fake_worker);
    }

    // Test 3: State Verification
    #[test]
    fn test_storage_state() {

        let env = Env::default();

        let tenant = Address::generate(&env);
        let landlord = Address::generate(&env);
        let worker = Address::generate(&env);

        let contract_id = env.register_contract(None, ChainFixDormContract);
        let client = ChainFixDormContractClient::new(&env, &contract_id);

        client.create_request(
            &2,
            &tenant,
            &landlord,
            &worker,
            &String::from_str(&env, "Aircon Repair"),
            &1000,
        );

        let request = client.get_request(&2);

        assert_eq!(request.payment_amount, 1000);
    }

    // Test 4
    #[test]
    fn test_completion_flag() {

        let env = Env::default();

        let tenant = Address::generate(&env);
        let landlord = Address::generate(&env);
        let worker = Address::generate(&env);

        let contract_id = env.register_contract(None, ChainFixDormContract);
        let client = ChainFixDormContractClient::new(&env, &contract_id);

        client.create_request(
            &3,
            &tenant,
            &landlord,
            &worker,
            &String::from_str(&env, "Door Repair"),
            &400,
        );

        client.mark_completed(&3, &worker);

        let request = client.get_request(&3);

        assert_eq!(request.completed, true);
    }

    // Test 5
    #[test]
    fn test_multiple_requests() {

        let env = Env::default();

        let tenant = Address::generate(&env);
        let landlord = Address::generate(&env);
        let worker = Address::generate(&env);

        let contract_id = env.register_contract(None, ChainFixDormContract);
        let client = ChainFixDormContractClient::new(&env, &contract_id);

        client.create_request(
            &10,
            &tenant,
            &landlord,
            &worker,
            &String::from_str(&env, "Light Repair"),
            &200,
        );

        client.create_request(
            &11,
            &tenant,
            &landlord,
            &worker,
            &String::from_str(&env, "Window Repair"),
            &600,
        );

        let req1 = client.get_request(&10);
        let req2 = client.get_request(&11);

        assert_eq!(req1.payment_amount, 200);
        assert_eq!(req2.payment_amount, 600);
    }
}