-- seed the initial set of validators
INSERT INTO near_validators (validator_id, factory_id) VALUES
  ('nodosigiloso.pool.near', 'pool.near'),
  ('mugglenodes.pool.near',  'pool.near'),
  ('luganodes.pool.near',    'pool.near'),
  ('owa.poolv1.near',        'poolv1.near')
ON CONFLICT (validator_id) DO NOTHING;