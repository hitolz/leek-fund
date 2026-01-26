import { invoke } from "@tauri-apps/api/tauri";
import {
  FundDetail,
  FundInfo,
  FundList,
  FundSummary,
  FundTrend,
  Holding,
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
