import React, { useEffect, useMemo, useState } from "react";
import { FundSummary } from "../types";
import { useTauriCommands } from "../hooks/useTauriCommands";
import { ListDetail } from "./ListDetail";
import { isFundInList } from "../utils/fundFilter";

interface ListDetailViewProps {
  listId: number | null;
  listName: string;
  selectedFundCode: string | null;
  onSelectFund: (code: string) => void;
  showToast?: (message: string, type: "success" | "error") => void;
  onListsChange: () => void;
  refreshIntervalMs: number;
  holdingVersion: number;
  sortKey: "holding_amount" | "daily_change_percent" | "daily_change_amount";
  sortOrder: "none" | "asc" | "desc";
  onSortKeyChange: (
    key: "holding_amount" | "daily_change_percent" | "daily_change_amount"
  ) => void;
  onSortOrderChange: (order: "none" | "asc" | "desc") => void;
}

export const ListDetailView: React.FC<ListDetailViewProps> = ({
  listId,
  listName,
  selectedFundCode,
  onSelectFund,
  showToast,
  onListsChange,
  refreshIntervalMs,
  holdingVersion,
  sortKey,
  sortOrder,
  onSortKeyChange,
  onSortOrderChange,
}) => {
  const [funds, setFunds] = useState<FundSummary[]>([]);
  const [loading, setLoading] = useState(false);
  const [newFundCode, setNewFundCode] = useState("");
  const { getListFundSummaries, addFundToList, syncFundPingzhong } =
    useTauriCommands();

  useEffect(() => {
    if (listId !== null) {
      loadFunds(false);
    } else {
      setFunds([]);
    }
  }, [listId, holdingVersion]);

  useEffect(() => {
    if (listId === null) return;
    const timer = setInterval(() => {
      loadFunds(true);
    }, refreshIntervalMs);
    return () => clearInterval(timer);
  }, [listId, refreshIntervalMs]);

  const loadFunds = async (silent = false) => {
    if (listId === null) return;

    if (!silent) {
      setLoading(true);
    }
    try {
      const fundList = await getListFundSummaries(listId);
      setFunds(fundList);
      if (silent && fundList.length > 0) {
        void Promise.allSettled(
          fundList.map((fund) => syncFundPingzhong(fund.code))
        );
      }
    } catch (error) {
      if (!silent && showToast) {
        showToast(String(error), "error");
      }
    } finally {
      if (!silent) {
        setLoading(false);
      }
    }
  };

  const handleAddFund = async () => {
    if (listId === null) return;
    const code = newFundCode.trim();
    if (!/^[0-9]{6}$/.test(code)) {
      showToast?.("请输入6位基金代码", "error");
      return;
    }
    if (isFundInList(funds, code)) {
      onSelectFund(code);
      showToast?.("基金已在列表中", "success");
      return;
    }

    try {
      await addFundToList(listId, code);
      showToast?.("已添加到列表", "success");
      setNewFundCode("");
      loadFunds(false);
      onListsChange();
    } catch (error) {
      showToast?.(String(error), "error");
    }
  };

  const sortedFunds = useMemo(() => {
    if (sortOrder === "none") {
      return funds;
    }
    const withIndex = funds.map((fund, index) => ({ fund, index }));
    withIndex.sort((a, b) => {
      const aValue = getSortValue(a.fund, sortKey);
      const bValue = getSortValue(b.fund, sortKey);
      if (aValue === null && bValue === null) {
        return a.index - b.index;
      }
      if (aValue === null) {
        return 1;
      }
      if (bValue === null) {
        return -1;
      }
      return sortOrder === "asc" ? aValue - bValue : bValue - aValue;
    });
    return withIndex.map((item) => item.fund);
  }, [funds, sortKey, sortOrder]);

  if (listId === null) {
    return (
      <div className="list-detail-view empty">
        <p>👈 请选择一个列表查看详情</p>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="list-detail-view loading">
        <p>加载中...</p>
      </div>
    );
  }

  if (funds.length === 0) {
    return (
      <div className="list-detail-view empty">
        <h3>{listName}</h3>
        <div className="list-add-fund">
          <input
            type="text"
            value={newFundCode}
            onChange={(e) =>
              setNewFundCode(e.target.value.replace(/\D/g, "").slice(0, 6))
            }
            onKeyDown={(e) => {
              if (e.key === "Enter") {
                handleAddFund();
              }
            }}
            placeholder="输入基金代码添加"
            maxLength={6}
            className="list-add-input"
          />
          <button onClick={handleAddFund} className="btn-add-inline">
            添加
          </button>
        </div>
        <p>列表为空，添加一些基金吧</p>
      </div>
    );
  }

  return (
    <div className="list-detail-view">
      <div className="detail-header">
        <h3>{listName}</h3>
        <span className="fund-count">{funds.length}只基金</span>
      </div>

      <div className="list-add-fund">
        <input
          type="text"
          value={newFundCode}
          onChange={(e) =>
            setNewFundCode(e.target.value.replace(/\D/g, "").slice(0, 6))
          }
          onKeyDown={(e) => {
            if (e.key === "Enter") {
              handleAddFund();
            }
          }}
          placeholder="输入基金代码添加"
          maxLength={6}
          className="list-add-input"
        />
        <button onClick={handleAddFund} className="btn-add-inline">
          添加
        </button>
      </div>

      <div className="fund-sort">
        <span className="fund-sort-label">排序字段</span>
        <select
          className="fund-sort-select"
          value={sortKey}
          onChange={(event) =>
            onSortKeyChange(
              event.target.value as
                | "holding_amount"
                | "daily_change_percent"
                | "daily_change_amount"
            )
          }
        >
          <option value="daily_change_amount">当日涨跌额</option>
          <option value="daily_change_percent">当日涨跌幅</option>
          <option value="holding_amount">持仓金额</option>
        </select>
        <span className="fund-sort-label">排序方式</span>
        <select
          className="fund-sort-select"
          value={sortOrder}
          onChange={(event) =>
            onSortOrderChange(event.target.value as "none" | "asc" | "desc")
          }
        >
          <option value="none">不排序</option>
          <option value="asc">升序</option>
          <option value="desc">降序</option>
        </select>
      </div>

      <div className="funds-list">
        {sortedFunds.map((fund) => (
          <ListDetail
            key={fund.code}
            fund={fund}
            selected={selectedFundCode === fund.code}
            onSelect={onSelectFund}
            sortKey={sortKey}
          />
        ))}
      </div>
    </div>
  );
};

function parseChangePercent(value: string | null) {
  if (!value) return null;
  const parsed = Number.parseFloat(value);
  return Number.isFinite(parsed) ? parsed : null;
}

function getSortValue(
  fund: FundSummary,
  key: "holding_amount" | "daily_change_percent" | "daily_change_amount"
) {
  if (key === "holding_amount") {
    return fund.holding_amount ?? null;
  }
  if (key === "daily_change_amount") {
    return fund.daily_change_amount ?? null;
  }
  return parseChangePercent(fund.daily_change_percent);
}
