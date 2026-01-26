-- group_fund_positions
CREATE TABLE IF NOT EXISTS group_fund_positions (
  group_id INTEGER NOT NULL,
  fund_code VARCHAR(64) NOT NULL,
  holding_amount REAL NOT NULL,
  holding_shares REAL NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  PRIMARY KEY (group_id, fund_code)
);

CREATE INDEX IF NOT EXISTS idx_group_fund_positions_group_id
  ON group_fund_positions (group_id);
CREATE INDEX IF NOT EXISTS idx_group_fund_positions_fund_code
  ON group_fund_positions (fund_code);
