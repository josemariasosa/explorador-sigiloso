use bip39::{Mnemonic, Seed};
use bip32::{XPrv, DerivationPath};
use bitcoin::util::address::Address;
use bitcoin::network::constants::Network;
use ethers::prelude::*;

fn generate_keys(mnemonic: &str, passphrase: &str) {
    let mnemonic = Mnemonic::parse(mnemonic).unwrap();
    let seed = Seed::new(&mnemonic, passphrase);
    
    // Derive BIP44 paths
    let btc_path = DerivationPath::from_str("m/44'/0'/0'/0/0").unwrap();
    let eth_path = DerivationPath::from_str("m/44'/60'/0'/0/0").unwrap();

    // Master key
    let xprv = XPrv::new(&seed).unwrap();

    let btc_key = xprv.derive_child(&btc_path).unwrap();
    let eth_key = xprv.derive_child(&eth_path).unwrap();

    // BTC address
    let btc_pubkey = bitcoin::PublicKey::from_private_key(&btc_key.to_secp256k1(), &btc_key.to_priv().to_key(Network::Bitcoin));
    let btc_addr = Address::p2wpkh(&btc_pubkey, Network::Bitcoin).unwrap();
    println!("BTC Address: {}", btc_addr);

    // ETH address
    let eth_wallet = LocalWallet::from_bytes(eth_key.to_bytes()).unwrap();
    println!("ETH Address: {:?}", eth_wallet.address());
}
