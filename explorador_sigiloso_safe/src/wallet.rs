use aes_gcm::{Aes256Gcm, KeyInit, Nonce}; // AES-GCM encryption
use aes_gcm::aead::{Aead, OsRng, generic_array::GenericArray};
use bip39::{Mnemonic, Seed};
use bip32::{XPrv, DerivationPath};
use rand::RngCore;
use sled::{Db};
use serde::{Serialize, Deserialize};
use zeroize::Zeroize;

const ROOT_KEY: &str = "root_seed";
const NONCE_SIZE: usize = 12; // For AES-GCM

#[derive(Serialize, Deserialize)]
struct EncryptedSeed {
    nonce: [u8; NONCE_SIZE],
    ciphertext: Vec<u8>,
}

pub struct SecureWallet {
    db: Db,
    aes_key: [u8; 32],
}


// Then store under key: "key:/m/44'/0'/0'/0/2" → KeyEntry.
#[derive(Serialize, Deserialize)]
struct KeyEntry {
    path: String,
    label: String,
    access_level: String, // e.g., "root", "admin", "employee"
    created_at: u64,
}


impl SecureWallet {
    pub fn new(db_path: &str, aes_key: [u8; 32]) -> Self {
        let db = sled::open(db_path).expect("failed to open DB");
        Self { db, aes_key }
    }

    pub fn init_seed(&self, mnemonic: &Mnemonic, passphrase: &str) {
        let seed = Seed::new(mnemonic, passphrase);
        let mut nonce = [0u8; NONCE_SIZE];
        OsRng.fill_bytes(&mut nonce);
        let cipher = Aes256Gcm::new(GenericArray::from_slice(&self.aes_key));
        let ciphertext = cipher.encrypt(
            GenericArray::from_slice(&nonce),
            seed.as_bytes()
        ).expect("encryption failed");

        let encrypted = EncryptedSeed { nonce, ciphertext };
        let bytes = serde_json::to_vec(&encrypted).unwrap();
        self.db.insert(ROOT_KEY, bytes).unwrap();
        self.db.flush().unwrap();
    }

    pub fn load_seed(&self, passphrase: &str) -> Seed {
        let encrypted = self.db.get(ROOT_KEY).unwrap().expect("no root seed stored");
        let enc_seed: EncryptedSeed = serde_json::from_slice(&encrypted).unwrap();
        let cipher = Aes256Gcm::new(GenericArray::from_slice(&self.aes_key));
        let decrypted = cipher.decrypt(
            GenericArray::from_slice(&enc_seed.nonce),
            enc_seed.ciphertext.as_ref()
        ).expect("decryption failed");
        Seed::new(&Mnemonic::from_entropy(&decrypted).unwrap(), passphrase)
    }

    pub fn derive_key(&self, derivation_path: &str, passphrase: &str) -> XPrv {
        let seed = self.load_seed(passphrase);
        let xprv = XPrv::new(seed.as_bytes()).unwrap();
        let path = DerivationPath::from_str(derivation_path).unwrap();
        xprv.derive_child(&path).unwrap()
    }
}
