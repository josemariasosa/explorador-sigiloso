-- Add migration script here
CREATE TABLE IF NOT EXISTS follower_snapshots (
  id             BIGSERIAL      PRIMARY KEY,
  follower_id    BIGINT         NOT NULL REFERENCES followers(id) ON DELETE CASCADE,
  data           JSONB          NOT NULL,
  taken_at       TIMESTAMPTZ    NOT NULL DEFAULT now()
);