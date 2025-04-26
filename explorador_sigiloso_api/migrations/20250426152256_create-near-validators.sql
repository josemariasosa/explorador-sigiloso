CREATE TABLE IF NOT EXISTS near_validators (
    id           SERIAL       PRIMARY KEY,
    validator_id TEXT         NOT NULL,
    factory_id   TEXT         NOT NULL,
    is_online    BOOLEAN      NOT NULL DEFAULT TRUE,
    updated_at   TIMESTAMPTZ  NOT NULL DEFAULT now(),
    created_at   TIMESTAMPTZ  NOT NULL DEFAULT now()
);

ALTER TABLE near_validators
  ADD CONSTRAINT uq_near_validators_validator
  UNIQUE (validator_id);