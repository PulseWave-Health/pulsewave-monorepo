#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Customer(Address),
    ValidationCount,
}

#[contracttype]
#[derive(Clone)]
pub struct CustomerRecord {
    pub address: Address,
    pub status: String,   // "pending" | "validated" | "rejected"
    pub data_hash: String, // hash of off-chain customer data
    pub timestamp: u64,
}

#[contract]
pub struct PulsewaveContract;

#[contractimpl]
impl PulsewaveContract {
    /// Submit a customer for validation with a hash of their data
    pub fn submit(env: Env, customer: Address, data_hash: String) {
        customer.require_auth();
        let record = CustomerRecord {
            address: customer.clone(),
            status: String::from_str(&env, "pending"),
            data_hash,
            timestamp: env.ledger().timestamp(),
        };
        env.storage().persistent().set(&DataKey::Customer(customer), &record);
        let count: u32 = env.storage().instance().get(&DataKey::ValidationCount).unwrap_or(0);
        env.storage().instance().set(&DataKey::ValidationCount, &(count + 1));
    }

    /// Validate or reject a customer (admin only — caller must be contract deployer)
    pub fn validate(env: Env, admin: Address, customer: Address, approved: bool) {
        admin.require_auth();
        let mut record: CustomerRecord = env
            .storage()
            .persistent()
            .get(&DataKey::Customer(customer.clone()))
            .expect("customer not found");
        record.status = if approved {
            String::from_str(&env, "validated")
        } else {
            String::from_str(&env, "rejected")
        };
        env.storage().persistent().set(&DataKey::Customer(customer), &record);
    }

    /// Get a customer record
    pub fn get(env: Env, customer: Address) -> Option<CustomerRecord> {
        env.storage().persistent().get(&DataKey::Customer(customer))
    }

    /// Total submissions count
    pub fn count(env: Env) -> u32 {
        env.storage().instance().get(&DataKey::ValidationCount).unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env};

    #[test]
    fn test_submit_and_validate() {
        let env = Env::default();
        env.mock_all_auths();
        let contract_id = env.register_contract(None, PulsewaveContract);
        let client = PulsewaveContractClient::new(&env, &contract_id);

        let customer = Address::generate(&env);
        let admin = Address::generate(&env);
        let hash = String::from_str(&env, "sha256:abc123");

        client.submit(&customer, &hash);
        assert_eq!(client.count(), 1);

        let record = client.get(&customer).unwrap();
        assert_eq!(record.status, String::from_str(&env, "pending"));

        client.validate(&admin, &customer, &true);
        let record = client.get(&customer).unwrap();
        assert_eq!(record.status, String::from_str(&env, "validated"));
    }
}
