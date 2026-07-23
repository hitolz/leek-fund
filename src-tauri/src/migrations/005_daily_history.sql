-- 股票每日行情历史
CREATE TABLE IF NOT EXISTS stock_daily_quotes (
    code TEXT NOT NULL,
    quote_date TEXT NOT NULL,
    price REAL,
    change_percent REAL,
    change_amount REAL,
    open REAL,
    high REAL,
    low REAL,
    yesterday_close REAL,
    volume REAL,
    created_at INTEGER NOT NULL,
    PRIMARY KEY (code, quote_date)
);

-- 加密货币/黄金每日行情历史
CREATE TABLE IF NOT EXISTS crypto_daily_quotes (
    symbol TEXT NOT NULL,
    quote_date TEXT NOT NULL,
    price REAL,
    change_percent REAL,
    high_24h REAL,
    low_24h REAL,
    volume_24h REAL,
    created_at INTEGER NOT NULL,
    PRIMARY KEY (symbol, quote_date)
);

-- 每日持仓快照
CREATE TABLE IF NOT EXISTS daily_portfolio_snapshot (
    snapshot_date TEXT PRIMARY KEY,
    total_value REAL NOT NULL,
    total_cost REAL,
    total_profit REAL,
    total_profit_percent REAL,
    fund_value REAL,
    stock_value REAL,
    crypto_value REAL,
    gold_value REAL,
    fund_count INTEGER,
    stock_count INTEGER,
    crypto_count INTEGER,
    gold_count INTEGER,
    details TEXT,
    created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_stock_daily_date ON stock_daily_quotes(quote_date);
CREATE INDEX IF NOT EXISTS idx_crypto_daily_date ON crypto_daily_quotes(quote_date);
