import React from "react";
import { FundSummary } from "../types";
import {
  formatChangePercent,
  formatTimestamp,
  getChangeClass,
} from "../utils/formatters";

interface ListDetailProps {
  fund: FundSummary;
  selected: boolean;
  onSelect: (code: string) => void;
}

export const ListDetail: React.FC<ListDetailProps> = ({
  fund,
  selected,
  onSelect,
}) => {
  const changeClass = getChangeClass(fund.daily_change_percent);
  const timeLabel = formatTimestamp(fund.update_time);

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
          {formatChangePercent(fund.daily_change_percent)}
        </span>
      </div>
      {timeLabel && <div className="fund-time">{timeLabel}</div>}
    </button>
  );
};
