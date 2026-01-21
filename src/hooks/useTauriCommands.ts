import {
  addFundToList,
  createList,
  deleteList,
  getFundDetail,
  getFundTrend,
  getAllLists,
  getListFundSummaries,
  getListFunds,
  renameList,
  reorderLists,
  removeFundFromList,
  searchFund,
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
    reorderLists,
  };
}
