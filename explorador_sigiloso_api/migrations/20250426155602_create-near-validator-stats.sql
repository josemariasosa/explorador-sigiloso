-- This table was created with the `nodosigiloso.pool.near` upgradeable contract
CREATE TABLE IF NOT EXISTS near_validator_status (
    id SERIAL PRIMARY KEY,
    validator_id TEXT NOT NULL,
    owner_id TEXT NOT NULL,
    total_staked_balance INTEGER NOT NULL,
    reward_fee_bp INTEGER NOT NULL,
    next_reward_fee_bp INTEGER NOT NULL,
    burn_fee_bp INTEGER NOT NULL,
    farms JSONB NOT NULL,
    snapshot_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);