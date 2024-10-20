🌟 StellarWill: Blockchain Tabanlı Dijital Vasiyet Sistemi

StellarWill, dijital varlıklarınızın güvenli ve otomatik bir şekilde yönetilmesini sağlayan blockchain tabanlı bir vasiyet sistemidir. Stellar blockchain'inde çalışan bu platform, kullanıcıların varlıklarını gelecekte belirli kişilere güvenle aktarabilmeleri için dijital vasiyetler oluşturmalarına olanak tanır.

🚀 Proje Özeti

StellarWill, kullanıcıların Stellar blockchain’i üzerinde dijital vasiyet oluşturmalarını sağlar. Bu akıllı sözleşme tabanlı sistemde, kullanıcılar vasiyetlerinde yer alan varlıkları (örneğin XLM) güvenli bir şekilde belirlenen mirasçılara aktarabilir. Varlık transferi, belirlenen açılma zamanında otomatik olarak gerçekleşir ve vasiyet açılana kadar blockchain'de güvenle saklanır.

🎯 Vizyon

Dijital dünyanın hızla büyüdüğü bir çağda, StellarWill ile miras ve vasiyet yönetimi daha güvenilir ve modern hale geliyor. Blockchain'in sağladığı güvenlik ve şeffaflık sayesinde, kullanıcılar varlıklarını gelecekteki kuşaklara güvenle aktarabilir. StellarWill, dijital varlık yönetiminde küresel ölçekte güvenilir bir çözüm haline gelmeyi ve milyonlarca insana ulaşarak dijital miras yönetimini devrim niteliğinde dönüştürmeyi hedefliyor.

💡 Öne Çıkan Özellikler

🔐 Güvenli Varlık Aktarımı: Varlıklarınız Stellar blockchain üzerinde güvenli bir şekilde saklanır ve belirlenen tarihte otomatik olarak mirasçıya aktarılır.
🕒 Zaman Kilidi: Varlık transferi, yalnızca belirlediğiniz açılma zamanında gerçekleşir, böylece kontrol tamamen sizde olur.
📜 Şeffaflık: Tüm vasiyet işlemleri blockchain üzerinde kaydedilir ve herkes tarafından denetlenebilir.
📊 Bakiye Sorgulama: Kullanıcılar bakiyelerini sorgulayabilir ve mevcut varlıklarını kontrol edebilir.
🛠 Yazılım Geliştirme Planı
Akıllı Sözleşme Geliştirme:

Fonksiyonlar: Vasiyet oluşturma, vasiyeti açma, bakiye sorgulama.
Değişkenler: Sahip, mirasçı, varlık miktarı, mesaj, açılma zamanı.
Veri Depolama:

Kalıcı depolama sistemi ile vasiyet ve varlık bilgilerinin güvenli bir şekilde saklanması.
Frontend Geliştirme:

Kullanıcı dostu bir arayüz ile vasiyet oluşturma ve açma işlemlerinin gerçekleştirilmesi.
Testler:

Akıllı sözleşme fonksiyonlarının ve frontend entegrasyonunun test edilmesi.
Dağıtım:

Stellar Testnet üzerinde projeyi yayınlama ve kullanıma sunma.
🛠 Kurulum Rehberi

Gerekli Araçlar:
Rust kurulumu.
Stellar CLI kurulumu.
Hedef platform: wasm32-unknown-unknown

Adımlar:
Projeyi Klonlayın:


git clone https://github.com/MustafaSenliik/StellarWill-Blockchain-Tabanli-Dijital-Vasiyet-Sistemi.git
cd StellarWill-Blockchain-Tabanli-Dijital-Vasiyet-Sistemi
Bağımlılıkları Yükleyin:


cargo build --target wasm32-unknown-unknown --release
Testnet’e Dağıtım:



soroban deploy --source ACCOUNT --network testnet
Frontend’i Yükleyin ve Çalıştırın:



cd frontend
npm install
npm start
Proje Testleri:


cargo test
