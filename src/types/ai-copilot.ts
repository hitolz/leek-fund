export type AssetCategory = "fund" | "stock" | "crypto" | "gold";

export interface AssetSnapshot {
  code: string;
  name: string;
  category: AssetCategory;
  cost_amount: number;
  holding_amount: number;
  holding_quantity: number;
  current_price: number | null;
  change_percent: number | null;
  daily_change_amount: number | null;
  valuation_basis: "quote" | "cost_fallback";
  group_name: string | null;
  update_time: string | null;
  data_complete: boolean;
}

export interface AssetAllocation {
  category: AssetCategory;
  label: string;
  total_value: number;
  percent: number;
  daily_change: number;
  count: number;
}

export interface AssetMover {
  code: string;
  name: string;
  category: AssetCategory;
  holding_amount: number;
  daily_change_amount: number;
  change_percent: number | null;
}

export interface ConcentrationMetrics {
  max_single_percent: number;
  max_single_name: string;
  top5_percent: number;
  top5_names: string[];
}

export interface DataQuality {
  total_assets: number;
  complete_assets: number;
  missing_holding: number;
  missing_quote: number;
  quote_coverage_percent: number;
  freshness: string;
  gaps: string[];
}

export interface PortfolioSnapshot {
  id: string;
  snapshot_at: number;
  total_value: number;
  daily_change_amount: number;
  daily_change_percent: number;
  daily_change_coverage_percent: number;
  assets: AssetSnapshot[];
  allocation: AssetAllocation[];
  top_movers: AssetMover[];
  concentration: ConcentrationMetrics;
  data_quality: DataQuality;
}

export interface ChatSession {
  session_id: string;
  title?: string | null;
  created_at: number;
  updated_at: number;
}

export interface ChatMessage {
  id: number;
  role: "user" | "assistant";
  content: string;
  created_at: number;
  saved_state?: "saved" | "unsaved";
}

export type LlmProvider =
  | "openai"
  | "claude"
  | "claude_compatible"
  | "openai_compatible";

export interface LlmConfig {
  provider: LlmProvider;
  base_url: string;
  model: string;
  max_tokens: number;
  temperature: number;
  has_api_key: boolean;
  api_key?: string;
}

export interface LlmActionResult {
  success: boolean;
  message: string;
}

export interface StreamEvent {
  event: "chunk" | "saved_state" | "done" | "error" | string;
  data: string;
}
