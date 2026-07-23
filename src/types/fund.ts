/**
 * 基金信息
 */
export interface FundInfo {
  code: string;
  name: string;
  net_value: number | null;
  change_percent: string | null;
  update_time: string | null;
}

export interface FundSummary {
  code: string;
  name: string;
  daily_change_percent: string | null;
  daily_change_amount?: number | null;
  holding_amount?: number | null;
  update_time: string | null;
}

export interface FundDetail {
  code: string;
  name: string;
  net_value: number | null;
  change_percent: string | null;
  update_time: string | null;
  daily_change_amount?: number | null;
  holding_amount?: number | null;
  holding_shares?: number | null;
  cost_price?: number | null;
}

export interface TrendPoint {
  date: string;
  value: number;
}

export interface FundTrend {
  code: string;
  window: string;
  points: TrendPoint[];
}

export interface Holding {
  list_id: number;
  fund_code: string;
  holding_amount: number;
  holding_shares: number;
  created_at: number;
  updated_at: number;
}

/**
 * 基金列表
 */
export interface FundList {
  id: number;
  name: string;
  fund_codes: string[];
  created_at: number;
  updated_at: number;
  position: number;
}

/**
 * 用户数据
 */
export interface UserData {
  schema_version: number;
  lists: FundList[];
  last_migrated_at: number | null;
  preferences?: Record<string, unknown>;
}

/**
 * 股票搜索结果
 */
export interface StockSearchResult {
  code: string;
  name: string;
  market: string;
}

/**
 * 股票行情信息
 */
export interface StockQuote {
  code: string;
  name: string;
  price: number | null;
  change_percent: number | null;
  change_amount: number | null;
  open: number | null;
  high: number | null;
  low: number | null;
  yesterday_close: number | null;
  volume: number | null;
  update_time: string | null;
}

/**
 * 加密货币行情信息
 */
export interface CryptoQuote {
  symbol: string;
  name: string;
  price: number | null;
  change_percent: number | null;
  high_24h: number | null;
  low_24h: number | null;
  volume_24h: number | null;
  update_time: string | null;
}

/**
 * 股票持仓信息
 */
export interface StockHolding {
  code: string;
  holding_amount: number;
  holding_shares: number;
  cost_price: number;
  created_at: number;
  updated_at: number;
}

/**
 * 加密货币持仓信息
 */
export interface CryptoHolding {
  symbol: string;
  holding_amount: number;
  holding_quantity: number;
  cost_price: number;
  created_at: number;
  updated_at: number;
}

/**
 * 黄金行情信息
 */
export interface GoldQuote {
  code: string;
  name: string;
  price: number | null;
  change_percent: number | null;
  change_amount: number | null;
  open: number | null;
  high: number | null;
  low: number | null;
  yesterday_close: number | null;
  volume: number | null;
  update_time: string | null;
}
