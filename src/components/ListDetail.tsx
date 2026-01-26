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
  sortKey: "holding_amount" | "daily_change_percent" | "daily_change_amount";
}

export const ListDetail: React.FC<ListDetailProps> = ({
  fund,
  selected,
  onSelect,
  sortKey,
}) => {
  const displayValue =
    sortKey === "holding_amount"
      ? formatCurrency(fund.holding_amount ?? null)
      : sortKey === "daily_change_amount"
        ? formatSignedCurrency(fund.daily_change_amount ?? null)
        : formatChangePercent(fund.daily_change_percent);

  const changeClass =
    sortKey === "daily_change_amount"
      ? getChangeClassFromNumber(fund.daily_change_amount ?? null)
      : sortKey === "holding_amount"
        ? "neutral"
        : getChangeClass(fund.daily_change_percent);

  return (
    <button
      type="button"
      className={`fund-row ${selected ? "selected" : ""}`}
      onClick={() => onSelect(fund.code)}
    >
      <div className="fund-row-main">
        <div className="fund-row-title">
          <span className="fund-code">{fund.code}</span>
          <span className="fund-name">{fund.name}</span>
        </div>
        <span className={`fund-change ${changeClass}`}>
          {displayValue}
        </span>
      </div>
    </button>
  );
};
