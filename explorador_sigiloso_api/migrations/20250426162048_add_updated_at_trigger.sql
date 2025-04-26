-- 1) Trigger function: only bump updated_at when is_online changes
CREATE OR REPLACE FUNCTION update_updated_at()
  RETURNS TRIGGER AS $$
BEGIN
  IF NEW.is_online IS DISTINCT FROM OLD.is_online THEN
    NEW.updated_at = now();
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 2) Attach the trigger to the table
DROP TRIGGER IF EXISTS trigger_update_updated_at
  ON near_validators;

CREATE TRIGGER trigger_update_updated_at
  BEFORE UPDATE OF is_online
  ON near_validators
  FOR EACH ROW
  EXECUTE PROCEDURE update_updated_at();