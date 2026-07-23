-- AI 投资驾驶舱相关表
-- Migration 003: AI Copilot tables

-- 组合快照（不可变，绑定每次分析）
CREATE TABLE IF NOT EXISTS portfolio_snapshots (
    id TEXT PRIMARY KEY,
    snapshot_at INTEGER NOT NULL,
    payload TEXT NOT NULL,
    data_quality TEXT NOT NULL,
    created_at INTEGER NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_portfolio_snapshots_time
    ON portfolio_snapshots(snapshot_at DESC, created_at DESC);

-- AI 报告
CREATE TABLE IF NOT EXISTS ai_reports (
    id TEXT PRIMARY KEY,
    type TEXT NOT NULL,
    title TEXT NOT NULL,
    snapshot_id TEXT NOT NULL,
    content TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'completed',
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (snapshot_id) REFERENCES portfolio_snapshots(id)
);

-- AI 结论（可追溯）
CREATE TABLE IF NOT EXISTS ai_findings (
    id TEXT PRIMARY KEY,
    report_id TEXT NOT NULL,
    kind TEXT NOT NULL,
    severity TEXT NOT NULL,
    title TEXT NOT NULL,
    detail TEXT NOT NULL,
    evidence_json TEXT,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (report_id) REFERENCES ai_reports(id)
);

-- 会话表
CREATE TABLE IF NOT EXISTS sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL UNIQUE,
    title TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- 消息表
CREATE TABLE IF NOT EXISTS session_chat_messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id TEXT NOT NULL,
    role TEXT NOT NULL,
    content TEXT NOT NULL,
    saved_state TEXT,
    snapshot_id TEXT,
    context_json TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL,
    FOREIGN KEY (session_id) REFERENCES sessions(session_id),
    FOREIGN KEY (snapshot_id) REFERENCES portfolio_snapshots(id)
);

-- Agent 表
CREATE TABLE IF NOT EXISTS agents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    description TEXT,
    created_at INTEGER NOT NULL,
    updated_at INTEGER NOT NULL
);

-- 云模型出站字段审计，不记录 API Key 和消息正文
CREATE TABLE IF NOT EXISTS ai_request_audits (
    id TEXT PRIMARY KEY,
    session_id TEXT NOT NULL,
    snapshot_id TEXT,
    provider TEXT NOT NULL,
    data_mode TEXT NOT NULL,
    fields_sent TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    FOREIGN KEY (session_id) REFERENCES sessions(session_id),
    FOREIGN KEY (snapshot_id) REFERENCES portfolio_snapshots(id)
);

-- 索引
CREATE INDEX IF NOT EXISTS idx_ai_reports_snapshot ON ai_reports(snapshot_id);
CREATE INDEX IF NOT EXISTS idx_ai_reports_type ON ai_reports(type);
CREATE INDEX IF NOT EXISTS idx_ai_reports_created ON ai_reports(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_ai_findings_report ON ai_findings(report_id);
CREATE INDEX IF NOT EXISTS idx_session_messages_session ON session_chat_messages(session_id);
CREATE INDEX IF NOT EXISTS idx_ai_request_audits_session ON ai_request_audits(session_id, created_at DESC);
