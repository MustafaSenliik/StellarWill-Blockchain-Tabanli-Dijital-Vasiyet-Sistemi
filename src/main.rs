// WillContract modülünü dışa aktar
pub use crate::WillContract;

use soroban_sdk::{Env, Address, Symbol, String};

fn main() {
    // Environment oluşturuluyor
    let env = Env::default();

    // Sahip ve mirasçı adresleri oluşturuluyor
    let owner = Address::from_str("GBHHMP5QS33VZVRK3WQM7ZS3XGEMO67RCZYPCBPQD3ZWJHN3IHVGCY5N");
    let heir = Address::from_str("GDAAHBERD3KPZMZWGRMCFOBRAQU6IE34DVUF2QMREAEFB3J5NRWAQ2RH");

    // Varlık olarak XLM kullanılıyor
    let asset = Symbol::short("XLM");

    // Miktar ve açılma zamanı belirleniyor
    let amount: i128 = 1000; // Miktar
    let unlock_time: u64 = 1700000000; // Açılma zamanı

    // Vasiyet mesajı
    let message = String::from_slice(&env, "Son mesaj");

    // Vasiyet oluşturma
    WillContract::create_will(
        env.clone(),
        owner.clone(),
        heir.clone(),
        asset.clone(),
        amount,
        message.clone(),
        unlock_time,
    );

    println!("Vasiyet oluşturuldu!");

    // Vasiyeti açma zamanı kontrolü
    let current_time = env.ledger().timestamp();
    if current_time >= unlock_time {
        let result = WillContract::unlock_will(
            env.clone(),
            owner.clone(),
            heir.clone(),
            asset.clone(),
            unlock_time,
        );

        println!("Miras alındı: {:?}, Mesaj: {:?}", result.0, result.1);
    } else {
        println!(
            "Vasiyet henüz açılmadı. Mevcut zaman: {}, Açılma zamanı: {}",
            current_time, unlock_time
        );
    }

    // Vasiyeti görüntüleme
    if let Some((stored_amount, stored_message)) = WillContract::view_will(
        env.clone(),
        owner.clone(),
        heir.clone(),
        asset.clone(),
        unlock_time,
    ) {
        println!("Vasiyet Bilgileri - Miktar: {}, Mesaj: {}", stored_amount, stored_message);
    } else {
        println!("Bu vasiyet bulunamadı.");
    }
}
