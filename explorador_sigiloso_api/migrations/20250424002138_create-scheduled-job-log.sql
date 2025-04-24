-- I love this.
CREATE TABLE IF NOT EXISTS scheduled_job_logs (
  id             BIGSERIAL      PRIMARY KEY,
  follower_id    BIGINT         NOT NULL REFERENCES followers(id) ON DELETE CASCADE,
  scheduled_by   TEXT           NOT NULL,
  scheduled_at   TIMESTAMPTZ    NOT NULL DEFAULT now()
);
