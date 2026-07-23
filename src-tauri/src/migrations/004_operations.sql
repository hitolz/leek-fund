-- AI 记账操作记录表
CREATE TABLE IF NOT EXISTS operations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    op_type TEXT NOT NULL CHECK(op_type IN ('buy', 'sell', 'dividend', 'transfer')),
    asset_type TEXT NOT NULL CHECK(asset_type IN ('fund', 'stock', 'crypto', 'gold')),
    asset_code TEXT NOT NULL,
    asset_name TEXT,
    amount REAL,
    shares REAL,
    price REAL,
    note TEXT,
    op_date TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_operations_date ON operations(op_date);
CREATE INDEX IF NOT EXISTS idx_operations_asset ON operations(asset_type, asset_code);
