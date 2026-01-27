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
  sortKey: "holding_amount" | "daily_change_percent" | "daily_change_amount";
}

export const ListDetail: React.FC<ListDetailProps> = ({
  fund,
  selected,
  onSelect,
  onRemove,
  sortKey,
}) => {
  const [dragOffset, setDragOffset] = React.useState(0);
  const [revealed, setRevealed] = React.useState(false);
  const dragStartRef = React.useRef<number | null>(null);
  const movedRef = React.useRef(false);
  const revealWidth = 72;

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
  const revealRatio = Math.min(dragOffset / revealWidth, 1);

  const handlePointerDown = (event: React.PointerEvent) => {
    dragStartRef.current = event.clientX;
    movedRef.current = false;
  };

  const handlePointerMove = (event: React.PointerEvent) => {
    if (dragStartRef.current === null) return;
    const delta = event.clientX - dragStartRef.current;
    if (Math.abs(delta) > 4) {
      movedRef.current = true;
    }
    if (delta <= 0) {
      setDragOffset(0);
      return;
    }
    setDragOffset(Math.min(delta, revealWidth));
  };

  const handlePointerEnd = () => {
    if (dragStartRef.current === null) return;
    dragStartRef.current = null;
    if (dragOffset > revealWidth * 0.5) {
      setRevealed(true);
      setDragOffset(revealWidth);
    } else {
      setRevealed(false);
      setDragOffset(0);
    }
  };

  const handleSelect = () => {
    if (movedRef.current) {
      movedRef.current = false;
      return;
    }
    if (revealed) {
      setRevealed(false);
      setDragOffset(0);
      return;
    }
    onSelect(fund.code);
  };

  return (
    <div className={`fund-row-swipe ${revealed ? "revealed" : ""}`}>
      <button
        type="button"
        className={`fund-row ${selected ? "selected" : ""}`}
        onClick={handleSelect}
        onPointerDown={handlePointerDown}
        onPointerMove={handlePointerMove}
        onPointerUp={handlePointerEnd}
        onPointerLeave={handlePointerEnd}
        onPointerCancel={handlePointerEnd}
        style={{ transform: `translateX(${dragOffset}px)` }}
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
      <button
        type="button"
        className="fund-row-delete"
        onClick={() => onRemove(fund.code)}
        style={{
          opacity: revealed ? 1 : revealRatio,
          transform: `translateX(${revealed ? 0 : -12 + 12 * revealRatio}px)`,
          pointerEvents: revealed ? "auto" : "none",
        }}
      >
        删除
      </button>
    </div>
  );
};
