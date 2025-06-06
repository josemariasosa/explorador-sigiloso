pub fn sign_btc_psbt(path: &str, psbt_base64: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Decrypt seed, derive key, parse PSBT, sign
    // Example only – real impl needs error handling and safety
    use base64;
    use bitcoin::{util::psbt::PartiallySignedTransaction, consensus::encode};
    use bip39::{Mnemonic, Seed};
    use bip32::{XPrv, DerivationPath};

    let seed = get_seed_securely(); // ← your encrypted storage access
    let xprv = XPrv::new(seed.as_bytes())?;
    let derivation = DerivationPath::from_str(path)?;
    let child = xprv.derive_child(&derivation)?;

    let psbt_bytes = base64::decode(psbt_base64)?;
    let mut psbt: PartiallySignedTransaction = encode::deserialize(&psbt_bytes)?;
    
    // Sign here using `bitcoin::util::bip32` + psbt finalizer...
    // For now, fake signature
    Ok("signed_psbt_base64_string".into())
}

use ethers::prelude::*;
use ethers::types::{TransactionRequest, Signature};
use bip32::{XPrv, DerivationPath};
use bip39::{Mnemonic, Seed};

pub fn sign_eth_tx(path: &str, tx_data: &TransactionRequest, chain_id: u64) -> Result<String, Box<dyn std::error::Error>> {
    let seed = get_seed_securely();
    let xprv = XPrv::new(seed.as_bytes())?;
    let derivation = DerivationPath::from_str(path)?;
    let derived = xprv.derive_child(&derivation)?;

    let wallet: LocalWallet = derived.private_key().to_bytes().into();
    let wallet = wallet.with_chain_id(chain_id);

    let sig = wallet.sign_transaction(&tx_data.clone()).await?;
    let signed = tx_data.rlp_signed(&sig);

    Ok(format!("0x{}", hex::encode(signed)))
}
