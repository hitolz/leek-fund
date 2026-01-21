import React, { useMemo } from "react";
import { FundTrend } from "../types";

interface FundTrendChartProps {
  trend: FundTrend | null;
}

const CHART_WIDTH = 260;
const CHART_HEIGHT = 120;

export const FundTrendChart: React.FC<FundTrendChartProps> = ({ trend }) => {
  const points = trend?.points ?? [];

  const pathD = useMemo(() => {
    if (points.length === 0) return "";
    const values = points.map((point) => point.value);
    const min = Math.min(...values);
    const max = Math.max(...values);
    const range = max - min || 1;

    return points
      .map((point, index) => {
        const x = (index / (points.length - 1 || 1)) * CHART_WIDTH;
        const y = CHART_HEIGHT - ((point.value - min) / range) * CHART_HEIGHT;
        return `${index === 0 ? "M" : "L"}${x.toFixed(2)},${y.toFixed(2)}`;
      })
      .join(" ");
  }, [points]);

  if (!trend || points.length === 0) {
    return (
      <div className="fund-trend empty">
        <p>暂无走势数据</p>
      </div>
    );
  }

  return (
    <div className="fund-trend">
      <div className="fund-trend-header">
        <span>走势</span>
        <span className="fund-trend-window">{trend.window}</span>
      </div>
      <svg
        width={CHART_WIDTH}
        height={CHART_HEIGHT}
        viewBox={`0 0 ${CHART_WIDTH} ${CHART_HEIGHT}`}
        aria-label="Fund trend chart"
      >
        <path
          d={pathD}
          fill="none"
          stroke="#1890ff"
          strokeWidth="2"
        />
      </svg>
    </div>
  );
};
