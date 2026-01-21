-- groups
CREATE TABLE IF NOT EXISTS groups (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name VARCHAR(64) NOT NULL,
  position INTEGER NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);

-- funds
CREATE TABLE IF NOT EXISTS funds (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  code VARCHAR(64) NOT NULL UNIQUE,
  name VARCHAR(64),
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL
);

-- group_funds
CREATE TABLE IF NOT EXISTS group_funds (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  group_id INTEGER NOT NULL,
  fund_code VARCHAR(64) NOT NULL,
  position INTEGER NOT NULL,
  created_at INTEGER NOT NULL,
  updated_at INTEGER NOT NULL,
  UNIQUE (group_id, fund_code)
);

CREATE INDEX IF NOT EXISTS idx_group_funds_group_id ON group_funds (group_id);
CREATE INDEX IF NOT EXISTS idx_group_funds_fund_code ON group_funds (fund_code);

-- fund_pingzhong_raw
CREATE TABLE IF NOT EXISTS fund_pingzhong_raw (
  fund_code VARCHAR(64) NOT NULL,
  fetched_at INTEGER NOT NULL,
  payload TEXT NOT NULL,
  PRIMARY KEY (fund_code, fetched_at)
);

-- fund_pingzhong_kv
CREATE TABLE IF NOT EXISTS fund_pingzhong_kv (
  fund_code VARCHAR(64) NOT NULL,
  fetched_at INTEGER NOT NULL,
  var_name VARCHAR(64) NOT NULL,
  value_type VARCHAR(16) NOT NULL,
  value_text TEXT,
  PRIMARY KEY (fund_code, fetched_at, var_name)
);

CREATE INDEX IF NOT EXISTS idx_fund_pingzhong_kv_name ON fund_pingzhong_kv (var_name);

-- fund_profile
CREATE TABLE IF NOT EXISTS fund_profile (
  fund_code VARCHAR(64) PRIMARY KEY,
  name VARCHAR(128) NOT NULL,
  is_money INTEGER NOT NULL DEFAULT 0,
  source_rate REAL,
  rate REAL,
  min_purchase REAL,
  updated_at INTEGER NOT NULL
);

-- fund_nav_daily
CREATE TABLE IF NOT EXISTS fund_nav_daily (
  fund_code VARCHAR(64) NOT NULL,
  nav_date INTEGER NOT NULL,
  unit_nav REAL,
  accum_nav REAL,
  equity_return REAL,
  unit_money TEXT,
  updated_at INTEGER NOT NULL,
  PRIMARY KEY (fund_code, nav_date)
);

CREATE INDEX IF NOT EXISTS idx_fund_nav_daily_date ON fund_nav_daily (nav_date);

-- fund_rank_daily
CREATE TABLE IF NOT EXISTS fund_rank_daily (
  fund_code VARCHAR(64) NOT NULL,
  rank_date INTEGER NOT NULL,
  rank INTEGER,
  total INTEGER,
  percentile REAL,
  updated_at INTEGER NOT NULL,
  PRIMARY KEY (fund_code, rank_date)
);

CREATE INDEX IF NOT EXISTS idx_fund_rank_daily_date ON fund_rank_daily (rank_date);

-- fund_return_summary
CREATE TABLE IF NOT EXISTS fund_return_summary (
  fund_code VARCHAR(64) NOT NULL,
  period VARCHAR(16) NOT NULL,
  value REAL NOT NULL,
  updated_at INTEGER NOT NULL,
  PRIMARY KEY (fund_code, period)
);

-- fund_asset_allocation
CREATE TABLE IF NOT EXISTS fund_asset_allocation (
  fund_code VARCHAR(64) NOT NULL,
  report_date INTEGER NOT NULL,
  stock_pct REAL,
  bond_pct REAL,
  cash_pct REAL,
  other_pct REAL,
  updated_at INTEGER NOT NULL,
  PRIMARY KEY (fund_code, report_date)
);

-- fund_holder_structure
CREATE TABLE IF NOT EXISTS fund_holder_structure (
  fund_code VARCHAR(64) NOT NULL,
  report_date INTEGER NOT NULL,
  institution_pct REAL,
  individual_pct REAL,
  internal_pct REAL,
  updated_at INTEGER NOT NULL,
  PRIMARY KEY (fund_code, report_date)
);

-- fund_manager
CREATE TABLE IF NOT EXISTS fund_manager (
  manager_id VARCHAR(64) PRIMARY KEY,
  name VARCHAR(128) NOT NULL,
  star INTEGER,
  pic_url TEXT,
  work_time_text TEXT
);

-- fund_manager_rel
CREATE TABLE IF NOT EXISTS fund_manager_rel (
  fund_code VARCHAR(64) NOT NULL,
  manager_id VARCHAR(64) NOT NULL,
  fund_size_text TEXT,
  updated_at INTEGER NOT NULL,
  PRIMARY KEY (fund_code, manager_id)
);
