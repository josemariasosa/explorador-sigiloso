#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,                  // internal primary key
    pub created_at: DateTime<Utc>,
    pub display_name: Option<String>, // optional ENS or human label
    pub notes: Option<String>,        // "This wallet appears in tx with X"
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserIdentity {
    pub id: Uuid,
    pub user_id: Uuid,                   // FK to User
    pub identity_type: IdentityType,
    pub identifier: String,             // e.g. bc1..., 0x..., 03abc..., sigiloso.eth
    pub first_seen: Option<DateTime<Utc>>,
    pub last_seen: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "identity_type", rename_all = "lowercase")] // if you use Postgres enums
pub enum IdentityType {
    BitcoinAddress,
    EthereumAddress,
    PublicKey,
    ENS,
}

