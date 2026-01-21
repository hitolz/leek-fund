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
  update_time: string | null;
}

export interface FundDetail {
  code: string;
  name: string;
  net_value: number | null;
  change_percent: string | null;
  update_time: string | null;
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

/**
 * 基金列表
 */
export interface FundList {
  id: string;
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
