import React, { useEffect, useMemo, useState } from "react";
import { formatCurrency } from "../utils/formatters";

interface HoldingFormProps {
  holdingAmount: number | null | undefined;
  holdingShares: number | null | undefined;
  dailyChangeAmount: number | null | undefined;
  loading: boolean;
  error: string | null;
  onSave: (amount: number, shares: number) => void;
  onClear: () => void;
}

export const HoldingForm: React.FC<HoldingFormProps> = ({
  holdingAmount,
  holdingShares,
  dailyChangeAmount,
  loading,
  error,
  onSave,
  onClear,
}) => {
  const [amountInput, setAmountInput] = useState("");
  const [sharesInput, setSharesInput] = useState("");

  useEffect(() => {
    setAmountInput(
      holdingAmount !== null && holdingAmount !== undefined
        ? holdingAmount.toFixed(2)
        : ""
    );
  }, [holdingAmount]);

  useEffect(() => {
    setSharesInput(
      holdingShares !== null && holdingShares !== undefined
        ? holdingShares.toFixed(2)
        : ""
    );
  }, [holdingShares]);

  const amountValue = parseInput(amountInput);
  const sharesValue = parseInput(sharesInput);

  const costPrice = useMemo(() => {
    if (sharesValue === null || sharesValue === 0) {
      return null;
    }
    if (amountValue === null) {
      return null;
    }
    return amountValue / sharesValue;
  }, [amountValue, sharesValue]);

  const hasZeroShares = sharesValue !== null && sharesValue === 0;

  const handleSave = () => {
    if (amountValue === null || sharesValue === null) {
      return;
    }
    onSave(round2(amountValue), round2(sharesValue));
  };

  return (
    <div className="holding-section">
      <div className="holding-header">
        <h4>持仓设置</h4>
        {error && <span className="holding-error">{error}</span>}
      </div>

      <div className="holding-fields">
        <label>
          持仓金额
          <input
            type="number"
            inputMode="decimal"
            step="0.01"
            min="0"
            value={amountInput}
            onChange={(e) => setAmountInput(e.target.value)}
            placeholder="0.00"
            disabled={loading}
          />
        </label>
        <label>
          持仓份额
          <input
            type="number"
            inputMode="decimal"
            step="0.01"
            min="0"
            value={sharesInput}
            onChange={(e) => setSharesInput(e.target.value)}
            placeholder="0.00"
            disabled={loading}
          />
        </label>
        <label>
          成本价
          <div className="holding-readonly">
            {hasZeroShares ? "--" : formatCurrency(costPrice)}
          </div>
          {hasZeroShares && (
            <span className="holding-hint">份额为 0，无法计算</span>
          )}
        </label>
      </div>

      <div className="holding-actions">
        <button type="button" onClick={onClear} disabled={loading}>
          清空
        </button>
        <button
          type="button"
          className="primary"
          onClick={handleSave}
          disabled={
            loading || amountValue === null || sharesValue === null
          }
        >
          保存
        </button>
      </div>

      <div className="holding-summary">
        <div>
          <span>持仓金额</span>
          <strong>{formatCurrency(holdingAmount ?? null)}</strong>
        </div>
        <div>
          <span>当日涨跌金额</span>
          <strong>{formatCurrency(dailyChangeAmount ?? null)}</strong>
        </div>
      </div>
    </div>
  );
};

function parseInput(value: string): number | null {
  if (!value.trim()) return null;
  const parsed = Number(value);
  if (!Number.isFinite(parsed)) return null;
  return parsed;
}

function round2(value: number) {
  return Math.round(value * 100) / 100;
}
