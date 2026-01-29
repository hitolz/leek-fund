import React, { useEffect, useMemo, useState } from "react";
import { FundSummary } from "../types";
import { useTauriCommands } from "../hooks/useTauriCommands";
import { ListDetail } from "./ListDetail";
import { isFundInList } from "../utils/fundFilter";
import {
  formatSignedCurrency,
  getChangeClassFromNumber,
} from "../utils/formatters";

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
  const {
    getListFundSummaries,
    addFundToList,
    removeFundFromList,
    syncFundPingzhong,
  } = useTauriCommands();

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

  const handleRemoveFund = async (code: string) => {
    if (listId === null) return;
    try {
      await removeFundFromList(listId, code);
      if (selectedFundCode === code) {
        onSelectFund("");
      }
      showToast?.("已从列表移除", "success");
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

  const dailyChangeTotal = useMemo(() => {
    let total = 0;
    let hasValue = false;
    funds.forEach((fund) => {
      const value = fund.daily_change_amount;
      if (value === null || value === undefined || !Number.isFinite(value)) {
        return;
      }
      total += value;
      hasValue = true;
    });
    return hasValue ? total : null;
  }, [funds]);

  if (listId === null) {
    return (
      <div className="list-detail-view empty">
        <div className="empty-state">
          <div className="panel-title">请选择一个列表</div>
          <div className="panel-subtitle">从左侧选择列表后查看基金详情</div>
        </div>
      </div>
    );
  }

  if (loading) {
    return (
      <div className="list-detail-view loading">
        <div className="empty-state">
          <div className="panel-title">正在加载</div>
          <div className="panel-subtitle">请稍候，正在同步基金列表</div>
        </div>
      </div>
    );
  }

  if (funds.length === 0) {
    return (
      <div className="list-detail-view empty">
        <div className="panel-title">{listName}</div>
        <div className="toolbar add-form">
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
            className="input"
          />
          <button onClick={handleAddFund} className="button primary small">
            添加
          </button>
        </div>
        <div className="empty-state">
          <div className="panel-subtitle">列表为空，添加一些基金吧</div>
        </div>
      </div>
    );
  }

  const sortLabel = getSortLabel(sortKey);

  return (
    <div className="list-detail-view">
      <div className="panel-header">
        <div>
          <div className="panel-title">{listName}</div>
          <div className="panel-subtitle">
            {funds.length} 只基金 · 当前排序：{sortLabel}
          </div>
        </div>
      </div>

      <div className="toolbar">
        <div className="toolbar-group">
          <span className="toolbar-label">排序字段</span>
          {renderSortButton(
            "daily_change_percent",
            "当日涨跌幅",
            sortKey,
            sortOrder,
            onSortKeyChange,
            onSortOrderChange
          )}
          {renderSortButton(
            "daily_change_amount",
            "当日涨跌额",
            sortKey,
            sortOrder,
            onSortKeyChange,
            onSortOrderChange
          )}
          {renderSortButton(
            "holding_amount",
            "持仓金额",
            sortKey,
            sortOrder,
            onSortKeyChange,
            onSortOrderChange
          )}
        </div>
        <div className="toolbar-group add-form">
          <span className="toolbar-label">添加基金</span>
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
            placeholder="输入基金代码"
            maxLength={6}
            className="input"
          />
          <button onClick={handleAddFund} className="button primary small">
            添加
          </button>
        </div>
      </div>

      <div className="fund-table">
        <div className="fund-head">
          <span>基金</span>
          <span className="metric-head">{sortLabel}</span>
          <span style={{ textAlign: "right" }}>操作</span>
        </div>
        <div className="fund-body">
          {sortedFunds.map((fund) => (
            <ListDetail
              key={fund.code}
              fund={fund}
              selected={selectedFundCode === fund.code}
              onSelect={onSelectFund}
              onRemove={handleRemoveFund}
              metricKey={sortKey}
            />
          ))}
        </div>
      </div>

      <div className="fund-summary">
        <span>当日涨跌金额汇总</span>
        <strong
          className={`fund-change ${getChangeClassFromNumber(
            dailyChangeTotal ?? null
          )}`}
        >
          {formatSignedCurrency(dailyChangeTotal ?? null)}
        </strong>
      </div>
    </div>
  );
};

function renderSortButton(
  field: "holding_amount" | "daily_change_percent" | "daily_change_amount",
  label: string,
  currentKey: "holding_amount" | "daily_change_percent" | "daily_change_amount",
  currentOrder: "none" | "asc" | "desc",
  onSortKeyChange: (
    key: "holding_amount" | "daily_change_percent" | "daily_change_amount"
  ) => void,
  onSortOrderChange: (order: "none" | "asc" | "desc") => void
) {
  const isActive = currentKey === field && currentOrder !== "none";
  const orderLabel =
    currentKey !== field || currentOrder === "none"
      ? "未排序"
      : currentOrder === "desc"
        ? "降序"
        : "升序";

  const handleClick = () => {
    if (currentKey !== field) {
      onSortKeyChange(field);
      onSortOrderChange("desc");
      return;
    }
    if (currentOrder === "desc") {
      onSortOrderChange("asc");
      return;
    }
    if (currentOrder === "asc") {
      onSortOrderChange("none");
      return;
    }
    onSortOrderChange("desc");
  };

  return (
    <button
      type="button"
      className={`sort-btn ${isActive ? "active" : ""}`}
      onClick={handleClick}
    >
      {label} <span className="sort-icon">{orderLabel}</span>
    </button>
  );
}

function getSortLabel(
  key: "holding_amount" | "daily_change_percent" | "daily_change_amount"
) {
  if (key === "holding_amount") return "持仓金额";
  if (key === "daily_change_amount") return "当日涨跌额";
  return "当日涨跌幅";
}

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
