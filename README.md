StellarWill: Blockchain Tabanlı Dijital Vasiyet Sistemi
Hakkımda
İsim: Mustafa Şenlik
Arka Plan: Blockchain teknolojisine ilgi duyan ve merkeziyetsiz uygulamalar (DApp) geliştirmeye odaklanan bir yazılım geliştiricisiyim. Bu proje, Stellar Bootcamp sürecinde geliştirdiğim bir projedir.
Tutku: Blockchain tabanlı çözümler geliştirerek günlük hayatta şeffaflık, güvenlik ve gelecek garantisi sağlayan hizmetler yaratma konusunda tutkuluyum.

Proje Açıklaması
StellarWill, kullanıcıların Stellar blockchain'i üzerinde dijital vasiyetler oluşturmasına olanak tanıyan merkeziyetsiz bir platformdur. Bu sistem sayesinde kullanıcılar, varlıklarını güvenli bir şekilde depolayabilir, yönetebilir ve belirli bir zaman geldiğinde otomatik olarak vasiyetlerini açabilir. Akıllı sözleşmeler, vasiyetlerin şeffaf ve manuel müdahaleye gerek kalmadan yerine getirilmesini sağlar, böylece dijital varlık yönetimi sağlanır.

Vizyon
StellarWill, insanların vasiyet ve miras yönetimi şekillerini dönüştürmeyi hedefliyor. Blockchain teknolojisinin gücüyle, bu proje güvenli ve otomatik varlık transferleri sağlar ve aracı kurumlara olan ihtiyacı azaltır. StellarWill, gelecekte dijital miras yönetiminde devrim yaratmayı hedefliyor.

Yazılım Geliştirme Planı
Akıllı Sözleşme Geliştirme:
Fonksiyonlar: Vasiyet oluşturma, vasiyeti açma, bakiyeyi sorgulama
Değişkenler: Sahip, mirasçı, varlık miktarı, mesaj, açılma zamanı
Veri Depolama:
Kalıcı depolama için gerekli anahtarlar ve veri yapılarını oluşturma.
Ödeme İşlemleri:
Vasiyet açıldığında otomatik XLM transferi.
Frontend Geliştirme:
Kullanıcı dostu bir arayüz geliştirerek vasiyet oluşturma ve açma işlemlerinin kolayca yapılmasını sağlama.
Testler:
Akıllı sözleşme fonksiyonlarını ve frontend'i test etme.
Dağıtım:
Stellar Testnet üzerinde projeyi dağıtma ve kullanıma hazır hale getirme.
Kişisel Hikaye
Blockchain teknolojisi ile insanların günlük yaşantılarındaki temel sorunlara çözümler üretmek benim için büyük bir tutku. StellarWill projesi ile bu vizyonu gerçekleştirmek için büyük bir adım attım. Dijital vasiyet sistemiyle, insanların miras süreçlerini daha güvenli ve modern bir şekilde yönetmesini sağlamak amacıyla bu projeyi geliştirdim.

Kurulum Talimatları
Gerekli Araçları Yükleyin:

Rust ve Stellar CLI kurun.
Hedef platformu kurun: wasm32-unknown-unknown
Depoyu Klonlayın:

bash
Kodu kopyala
git clone https://github.com/MustafaSenliik/StellarWill-Blockchain-Tabanli-Dijital-Vasiyet-Sistemi.git
cd StellarWill-Blockchain-Tabanli-Dijital-Vasiyet-Sistemi
Bağımlılıkları Yükleyin:

bash
Kodu kopyala
cargo build --target wasm32-unknown-unknown --release
Testnet Üzerinde Dağıtın:
Stellar CLI ve soroban SDK kullanarak projenizi Stellar Testnet üzerinde dağıtabilirsiniz.
