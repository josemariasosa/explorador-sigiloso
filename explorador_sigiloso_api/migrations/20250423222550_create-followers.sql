-- If you follow some data, to generate historical data,
-- then you become a follower of that data.
CREATE TABLE IF NOT EXISTS followers (
  id            BIGSERIAL PRIMARY KEY,   -- auto-incrementing 64-bit integer
  user_id       BIGINT NOT NULL,         -- references your users table (add FK if desired)
  label         TEXT NOT NULL,           -- e.g. "NEAR Validator"
  kind          TEXT NOT NULL,           -- e.g. "near_validator"
  target        JSONB NOT NULL,          -- config payload for this follower
  public        BOOLEAN NOT NULL DEFAULT FALSE,
  created_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);