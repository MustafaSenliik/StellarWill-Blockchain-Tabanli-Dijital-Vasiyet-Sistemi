#![no_std]

use soroban_sdk::{
    contract, contractimpl, Address, Env, Symbol, String,
};

#[derive(Clone)]
#[contract]
pub struct WillContract;

#[contractimpl]
impl WillContract {
    /// Vasiyet oluşturma fonksiyonu
    pub fn create_will(
        env: Env,
        owner: Address,
        heir: Address,
        asset: Symbol,
        amount: i128,
        message: String,
        unlock_time: u64,
    ) {
        owner.require_auth();
        assert!(amount > 0, "Miktar pozitif olmalı");
        assert!(
            unlock_time > env.ledger().timestamp(),
            "Açılma zamanı gelecekte olmalı"
        );

        // Anahtar (storage_key) oluşturuluyor
        let storage_key = (owner.clone(), heir.clone(), asset.clone(), unlock_time);

        // Veriyi kalıcı depolama alanına kaydediyoruz
        env.storage().persistent().set(&storage_key, &(amount, message));
    }

    /// Vasiyeti açma fonksiyonu
    pub fn unlock_will(
        env: Env,
        owner: Address,
        heir: Address,
        asset: Symbol,
        unlock_time: u64,
    ) -> (i128, String) {
        let current_time = env.ledger().timestamp();
        assert!(current_time >= unlock_time, "Vasiyet henüz açılamaz");

        // Anahtar oluşturuluyor
        let storage_key = (owner.clone(), heir.clone(), asset.clone(), unlock_time);

        // Vasiyet verilerini alıyoruz
        let (amount, message): (i128, String) = env
            .storage()
            .persistent()
            .get(&storage_key)
            .expect("Vasiyet bulunamadı");

        // Depolama alanından vasiyet siliniyor
        env.storage().persistent().remove(&storage_key);

        (amount, message)
    }

    /// Vasiyeti görüntüleme fonksiyonu
    pub fn view_will(
        env: Env,
        owner: Address,
        heir: Address,
        asset: Symbol,
        unlock_time: u64,
    ) -> Option<(i128, String)> {
        let storage_key = (owner.clone(), heir.clone(), asset.clone(), unlock_time);
        env.storage().persistent().get(&storage_key)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Env as TestEnv, String, Symbol};

    #[test]
    fn test_will_contract() {
        let env = TestEnv::default();
        let contract_id = env.register_contract(None, WillContract);
        let client = WillContractClient::new(&env, &contract_id);

        let owner = Address::random(&env);
        let heir = Address::random(&env);
        let asset = Symbol::short("XLM");
        let amount = 1000_i128;
        let message = String::from_slice(&env, "Vasiyet Mesajı");
        let unlock_time = 1700000000_u64;

        // Vasiyet oluşturma testi
        client.create_will(&owner, &heir, &asset, &amount, &message, &unlock_time);

        // Vasiyet görüntüleme testi
        let (stored_amount, stored_message) = client
            .view_will(&owner, &heir, &asset, &unlock_time)
            .expect("Vasiyet bulunmalı");
        assert_eq!(stored_amount, amount);
        assert_eq!(stored_message, message);
    }
}
