import React from "react";
import { FundSummary } from "../types";
import {
  formatChangePercent,
  formatCurrency,
  formatSignedCurrency,
  getChangeClass,
  getChangeClassFromNumber,
} from "../utils/formatters";

interface ListDetailProps {
  fund: FundSummary;
  selected: boolean;
  onSelect: (code: string) => void;
  onRemove: (code: string) => void;
  metricKey: "holding_amount" | "daily_change_percent" | "daily_change_amount";
  rowRef?: (node: HTMLDivElement | null) => void;
}

export const ListDetail: React.FC<ListDetailProps> = ({
  fund,
  selected,
  onSelect,
  onRemove,
  metricKey,
  rowRef,
}) => {
  const changePercentClass = getChangeClass(fund.daily_change_percent);
  const changeAmountClass = getChangeClassFromNumber(
    fund.daily_change_amount ?? null
  );

  const holdingAmount = formatCurrency(fund.holding_amount ?? null);
  const dailyChangePercent = formatChangePercent(fund.daily_change_percent);
  const dailyChangeAmount = formatSignedCurrency(
    fund.daily_change_amount ?? null
  );

  const metricValue =
    metricKey === "holding_amount"
      ? holdingAmount
      : metricKey === "daily_change_amount"
        ? dailyChangeAmount
        : dailyChangePercent;
  const metricClass =
    metricKey === "daily_change_amount"
      ? `metric-cell delta ${changeAmountClass}`
      : metricKey === "daily_change_percent"
        ? `metric-cell delta ${changePercentClass}`
        : "metric-cell";

  return (
    <div
      className={`fund-row ${selected ? "active" : ""}`}
      ref={rowRef}
      onClick={() => onSelect(fund.code)}
      role="button"
      tabIndex={0}
      onKeyDown={(event) => {
        if (event.key === "Enter") {
          onSelect(fund.code);
        }
      }}
    >
      <div>
        <div className="fund-name">{fund.name}</div>
        <div className="fund-code">代码：{fund.code}</div>
      </div>
      <div className={metricClass}>{metricValue}</div>
      <div style={{ textAlign: "right" }}>
        <button
          type="button"
          className="icon-btn"
          onClick={(event) => {
            event.stopPropagation();
            onRemove(fund.code);
          }}
        >
          删除
        </button>
      </div>
    </div>
  );
};
