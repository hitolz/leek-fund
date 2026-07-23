import {
  addFundToList,
  createList,
  deleteList,
  getHolding,
  getFundDetail,
  getFundTrend,
  getFundAccumTrend,
  syncFundPingzhong,
  getStorageWarning,
  getAllLists,
  getListFundSummaries,
  getListFunds,
  renameList,
  reorderLists,
  removeFundFromList,
  searchFund,
  setHolding,
  clearHolding,
  setRefreshInterval,
  // 股票
  searchStock,
  getStockQuote,
  // 加密货币
  getCryptoQuote,
  getCryptoQuotes,
  getPopularCryptos,
  // 股票持仓
  getStockHolding,
  setStockHolding,
  clearStockHolding,
  // 加密货币持仓
  getCryptoHolding,
  setCryptoHolding,
  clearCryptoHolding,
  // 黄金
  getGoldQuote,
  getGoldHolding,
  setGoldHolding,
  clearGoldHolding,
} from "./useTauriApi";

export function useTauriCommands() {
  return {
    searchFund,
    getAllLists,
    createList,
    renameList,
    deleteList,
    addFundToList,
    removeFundFromList,
    getListFunds,
    getListFundSummaries,
    getFundDetail,
    getFundTrend,
    getFundAccumTrend,
    syncFundPingzhong,
    getStorageWarning,
    reorderLists,
    getHolding,
    setHolding,
    clearHolding,
    setRefreshInterval,
    // 股票
    searchStock,
    getStockQuote,
    // 加密货币
    getCryptoQuote,
    getCryptoQuotes,
    getPopularCryptos,
    // 股票持仓
    getStockHolding,
    setStockHolding,
    clearStockHolding,
    // 加密货币持仓
    getCryptoHolding,
    setCryptoHolding,
    clearCryptoHolding,
    // 黄金
    getGoldQuote,
    getGoldHolding,
    setGoldHolding,
    clearGoldHolding,
  };
}
