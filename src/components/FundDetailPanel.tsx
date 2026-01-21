import React from "react";
import { FundDetail, FundTrend } from "../types";
import { FundTrendChart } from "./FundTrendChart";

interface FundDetailPanelProps {
  detail: FundDetail | null;
  trend: FundTrend | null;
  loading: boolean;
  error: string | null;
}

export const FundDetailPanel: React.FC<FundDetailPanelProps> = ({
  detail,
  trend,
  loading,
  error,
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

      <FundTrendChart trend={trend} />
    </div>
  );
};
