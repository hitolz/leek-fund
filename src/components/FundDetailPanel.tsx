import React from "react";
import { FundDetail, FundTrend, Holding } from "../types";
import { FundTrendChart } from "./FundTrendChart";
import { HoldingForm } from "./HoldingForm";
import { formatChangePercent, getChangeClass } from "../utils/formatters";

interface FundDetailPanelProps {
  detail: FundDetail | null;
  trend: FundTrend | null;
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
        <div className="empty-state">
          <div className="panel-title">正在加载详情</div>
          <div className="panel-subtitle">请稍候，正在获取基金信息</div>
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="fund-detail-panel empty">
        <div className="empty-state">
          <div className="panel-title">详情加载失败</div>
          <div className="panel-subtitle">{error}</div>
        </div>
      </div>
    );
  }

  if (!detail) {
    return (
      <div className="fund-detail-panel empty">
        <div className="empty-state">
          <div className="panel-title">请选择基金</div>
          <div className="panel-subtitle">选择基金后查看详情与走势</div>
        </div>
      </div>
    );
  }

  const changeClass = getChangeClass(detail.change_percent);
  const changePercentLabel = formatChangePercent(detail.change_percent);

  return (
    <div className="fund-detail-panel">
      <div className="detail-hero">
        <div className="panel-title">{detail.name}</div>
        <div className="panel-subtitle">基金代码：{detail.code}</div>
        {detail.update_time && (
          <div className="panel-subtitle">更新时间：{detail.update_time}</div>
        )}
      </div>

      <div className="detail-grid">
        <div className="metric-card">
          <div className="metric-label">最新净值</div>
          <div className="metric-value">
            {detail.net_value !== null && detail.net_value !== undefined
              ? detail.net_value.toFixed(4)
              : "--"}
          </div>
        </div>
        <div className="metric-card">
          <div className="metric-label">当日涨跌幅</div>
          <div className={`metric-value fund-change ${changeClass}`}>
            {changePercentLabel}
          </div>
        </div>
        <div className="metric-card">
          <div className="metric-label">当日涨跌额</div>
          <div className="metric-value">
            {detail.daily_change_amount !== null &&
            detail.daily_change_amount !== undefined
              ? detail.daily_change_amount.toFixed(2)
              : "--"}
          </div>
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
      </div>
    </div>
  );
};
