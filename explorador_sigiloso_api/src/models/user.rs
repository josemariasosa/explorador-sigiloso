#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,                  // internal primary key
    pub created_at: DateTime<Utc>,
    pub display_name: Option<String>, // optional ENS or human label
    pub notes: Option<String>,        // "This wallet appears in tx with X"
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "identity_type", rename_all = "lowercase")] // if you use Postgres enums
pub enum IdentityType {
    BitcoinAddress,
    EthereumAddress,
    PublicKey,
    ENS,
}

