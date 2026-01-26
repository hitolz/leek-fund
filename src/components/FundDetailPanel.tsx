import React from "react";
import { FundDetail, FundTrend, Holding } from "../types";
import { FundTrendChart } from "./FundTrendChart";
import { HoldingForm } from "./HoldingForm";

interface FundDetailPanelProps {
  detail: FundDetail | null;
  trend: FundTrend | null;
  accumTrend: FundTrend | null;
  loading: boolean;
  error: string | null;
  holding: Holding | null;
  holdingLoading: boolean;
  holdingError: string | null;
  onSaveHolding: (amount: number, shares: number) => void;
  onClearHolding: () => void;
}

export const FundDetailPanel: React.FC<FundDetailPanelProps> = ({
  detail,
  trend,
  accumTrend,
  loading,
  error,
  holding,
  holdingLoading,
  holdingError,
  onSaveHolding,
  onClearHolding,
}) => {
  if (loading) {
    return (
      <div className="fund-detail-panel empty">
        <p>加载中...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="fund-detail-panel empty">
        <p>{error}</p>
      </div>
    );
  }

  if (!detail) {
    return (
      <div className="fund-detail-panel empty">
        <p>👉 请选择一只基金查看详情</p>
      </div>
    );
  }

  return (
    <div className="fund-detail-panel">
      <div className="fund-detail-header">
        <div>
          <h3>{detail.name}</h3>
          <span className="fund-code">{detail.code}</span>
        </div>
        {detail.update_time && (
          <span className="fund-detail-time">{detail.update_time}</span>
        )}
      </div>

      <div className="fund-detail-metrics">
        <div className="fund-metric">
          <span className="label">最新净值</span>
          <span className="value">
            {detail.net_value !== null && detail.net_value !== undefined
              ? detail.net_value.toFixed(4)
              : "--"}
          </span>
        </div>
        <div className="fund-metric">
          <span className="label">当日涨跌</span>
          <span className="value">
            {detail.change_percent ? `${detail.change_percent}%` : "--"}
          </span>
        </div>
      </div>

      <HoldingForm
        holdingAmount={holding?.holding_amount ?? detail.holding_amount ?? null}
        holdingShares={holding?.holding_shares ?? detail.holding_shares ?? null}
        dailyChangeAmount={detail.daily_change_amount ?? null}
        loading={holdingLoading}
        error={holdingError}
        onSave={onSaveHolding}
        onClear={onClearHolding}
      />

      <div className="fund-trend-stack">
        <FundTrendChart
          trend={trend}
          title="单位净值走势"
          valueFormatter={(value) => value.toFixed(4)}
        />
        <FundTrendChart
          trend={accumTrend}
          title="累计收益率走势"
          stroke="#fa8c16"
          valueFormatter={(value) => `${value.toFixed(2)}%`}
        />
      </div>
    </div>
  );
};
