import { invoke } from "@tauri-apps/api/tauri";
import {
  CryptoHolding,
  CryptoQuote,
  FundDetail,
  FundInfo,
  FundList,
  FundSummary,
  FundTrend,
  GoldQuote,
  Holding,
  StockHolding,
  StockQuote,
  StockSearchResult,
} from "../types";

export const searchFund = (code: string): Promise<FundInfo> => {
  return invoke("search_fund", { code });
};

export const getAllLists = (): Promise<FundList[]> => {
  return invoke("get_all_lists");
};

export const createList = (name: string): Promise<FundList> => {
  return invoke("create_list", { name });
};

export const renameList = (id: number, newName: string): Promise<void> => {
  return invoke("rename_list", { id, newName });
};

export const deleteList = (id: number): Promise<void> => {
  return invoke("delete_list", { id });
};

export const addFundToList = (listId: number, fundCode: string): Promise<void> => {
  return invoke("add_fund_to_list", { listId, fundCode });
};

export const removeFundFromList = (
  listId: number,
  fundCode: string
): Promise<void> => {
  return invoke("remove_fund_from_list", { listId, fundCode });
};

export const getListFunds = (listId: number): Promise<FundInfo[]> => {
  return invoke("get_list_funds", { listId });
};

export const getListFundSummaries = (listId: number): Promise<FundSummary[]> => {
  return invoke("get_list_fund_summaries", { listId });
};

export const getFundDetail = (
  listId: number,
  fundCode: string
): Promise<FundDetail> => {
  return invoke("get_list_fund_detail", { listId, fundCode });
};

export const getFundTrend = (code: string): Promise<FundTrend> => {
  return invoke("get_fund_trend", { code });
};

export const getFundAccumTrend = (code: string): Promise<FundTrend> => {
  return invoke("get_fund_accum_trend", { code });
};

export const syncFundPingzhong = (code: string): Promise<void> => {
  return invoke("sync_fund_pingzhong", { code });
};

export const getStorageWarning = (): Promise<string | null> => {
  return invoke("get_storage_warning");
};

export const reorderLists = (listIds: number[]): Promise<void> => {
  return invoke("reorder_lists", { listIds });
};

export const getHolding = (
  listId: number,
  fundCode: string
): Promise<Holding | null> => {
  return invoke("get_holding", { listId, fundCode });
};

export const setHolding = (
  listId: number,
  fundCode: string,
  holdingAmount: number,
  holdingShares: number
): Promise<Holding> => {
  return invoke("set_holding", {
    listId,
    fundCode,
    holdingAmount,
    holdingShares,
  });
};

export const clearHolding = (
  listId: number,
  fundCode: string
): Promise<void> => {
  return invoke("clear_holding", { listId, fundCode });
};

export const setRefreshInterval = (intervalMs: number): Promise<void> => {
  return invoke("set_refresh_interval", { intervalMs });
};

// ============================================================================
// 股票相关
// ============================================================================

export const searchStock = (keyword: string): Promise<StockSearchResult[]> => {
  return invoke("search_stock", { keyword });
};

export const getStockQuote = (code: string): Promise<StockQuote> => {
  return invoke("get_stock_quote", { code });
};

// ============================================================================
// 加密货币相关
// ============================================================================

export const getCryptoQuote = (symbol: string): Promise<CryptoQuote> => {
  return invoke("get_crypto_quote", { symbol });
};

export const getCryptoQuotes = (symbols: string[]): Promise<CryptoQuote[]> => {
  return invoke("get_crypto_quotes", { symbols });
};

export const getPopularCryptos = (): Promise<[string, string][]> => {
  return invoke("get_popular_cryptos");
};

// ============================================================================
// 股票持仓
// ============================================================================

export const getStockHolding = (code: string): Promise<StockHolding | null> => {
  return invoke("get_stock_holding", { code });
};

export const setStockHolding = (
  code: string,
  holdingAmount: number,
  holdingShares: number
): Promise<StockHolding> => {
  return invoke("set_stock_holding", { code, holdingAmount, holdingShares });
};

export const clearStockHolding = (code: string): Promise<void> => {
  return invoke("clear_stock_holding", { code });
};

// ============================================================================
// 加密货币持仓
// ============================================================================

export const getCryptoHolding = (symbol: string): Promise<CryptoHolding | null> => {
  return invoke("get_crypto_holding", { symbol });
};

export const setCryptoHolding = (
  symbol: string,
  holdingAmount: number,
  holdingQuantity: number
): Promise<CryptoHolding> => {
  return invoke("set_crypto_holding", { symbol, holdingAmount, holdingQuantity });
};

export const clearCryptoHolding = (symbol: string): Promise<void> => {
  return invoke("clear_crypto_holding", { symbol });
};

// ============================================================================
// 黄金相关
// ============================================================================

export const getGoldQuote = (): Promise<GoldQuote> => {
  return invoke("get_gold_quote");
};

export const getGoldHolding = (): Promise<CryptoHolding | null> => {
  return invoke("get_gold_holding");
};

export const setGoldHolding = (
  holdingAmount: number,
  holdingQuantity: number
): Promise<CryptoHolding> => {
  return invoke("set_gold_holding", { holdingAmount, holdingQuantity });
};

export const clearGoldHolding = (): Promise<void> => {
  return invoke("clear_gold_holding");
};
