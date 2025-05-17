// pub fn parse_near_address(address: &str) -> Result<String, String> {
//     if address.len() != 64 {
//         return Err("Invalid NEAR address length".to_string());
//     }
//     if !address.chars().all(|c| c.is_ascii_hexdigit()) {
//         return Err("Invalid NEAR address format".to_string());
//     }
//     Ok(address.to_string())
// }

pub fn is_near_mainnet(address: &str) -> bool {
    match near_sdk::AccountId::validate(&address) {
        Ok(_) => true,
        Err(_) => false,
    }
}
