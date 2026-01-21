import React, { useState } from "react";
import { FundInfo, FundList } from "../types";
import { useTauriCommands } from "../hooks/useTauriCommands";

interface FundInfoCardProps {
  fund: FundInfo | null;
  lists: FundList[];
  showToast?: (message: string, type: "success" | "error") => void;
  onListsChange: () => void;
}

export const FundInfoCard: React.FC<FundInfoCardProps> = ({
  fund,
  lists,
  showToast,
  onListsChange,
}) => {
  const [selectedListId, setSelectedListId] = useState<number | null>(null);
  const [adding, setAdding] = useState(false);
  const { addFundToList } = useTauriCommands();

  if (!fund) {
    return (
      <div className="fund-info-card empty">
        <p>请输入基金代码查询</p>
      </div>
    );
  }

  const handleAddToList = async () => {
    if (selectedListId === null) {
      if (showToast) {
        showToast("请选择一个列表", "error");
      }
      return;
    }

    setAdding(true);
    try {
      await addFundToList(selectedListId, fund.code);
      if (showToast) {
        showToast("已添加到列表", "success");
      }
      setSelectedListId(null);
      onListsChange();
    } catch (error) {
      if (showToast) {
        showToast(String(error), "error");
      }
    } finally {
      setAdding(false);
    }
  };

  return (
    <div className="fund-info-card">
      <div className="fund-header">
        <h2>{fund.name}</h2>
        <span className="fund-code">{fund.code}</span>
      </div>

      <div className="fund-details">
        {fund.net_value !== null && (
          <div className="fund-value">
            <span className="label">净值：</span>
            <span className="value">{fund.net_value.toFixed(4)}</span>
          </div>
        )}
        {fund.change_percent && (
          <div className="fund-change">
            <span className="label">涨跌幅：</span>
            <span className="value">{fund.change_percent}%</span>
          </div>
        )}
        {fund.update_time && (
          <div className="fund-time">
            <span className="label">更新时间：</span>
            <span className="value">{fund.update_time}</span>
          </div>
        )}
      </div>

      <div className="add-to-list-section">
        <select
          value={selectedListId ?? ""}
          onChange={(e) =>
            setSelectedListId(e.target.value ? Number(e.target.value) : null)
          }
          className="list-selector"
          disabled={adding || lists.length === 0}
        >
          <option value="">选择列表</option>
          {lists.map((list) => (
            <option key={list.id} value={list.id}>
              {list.name} ({list.fund_codes.length}个基金)
            </option>
          ))}
        </select>

        <button
          onClick={handleAddToList}
          disabled={selectedListId === null || adding}
          className="btn-add"
        >
          {adding ? "添加中..." : "添加到列表"}
        </button>
      </div>

      {lists.length === 0 && (
        <p className="hint">请先创建一个列表</p>
      )}
    </div>
  );
};
