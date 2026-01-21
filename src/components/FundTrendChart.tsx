import React, { useMemo, useRef, useState } from "react";
import { FundTrend } from "../types";

interface FundTrendChartProps {
  trend: FundTrend | null;
  title?: string;
  stroke?: string;
  valueFormatter?: (value: number) => string;
}

const CHART_WIDTH = 360;
const CHART_HEIGHT = 200;
const PADDING_LEFT = 44;
const PADDING_RIGHT = 10;
const PADDING_TOP = 10;
const PADDING_BOTTOM = 34;
const TICK_COUNT = 4;
const X_TICK_COUNT = 4;

export const FundTrendChart: React.FC<FundTrendChartProps> = ({
  trend,
  title = "走势",
  stroke = "#1890ff",
  valueFormatter = (value) => value.toFixed(4),
}) => {
  const points = trend?.points ?? [];
  const [hoverIndex, setHoverIndex] = useState<number | null>(null);
  const [hoverPos, setHoverPos] = useState<{ x: number; y: number } | null>(
    null
  );
  const svgRef = useRef<SVGSVGElement | null>(null);

  const metrics = useMemo(() => {
    if (points.length === 0) {
      return null;
    }
    const values = points.map((point) => point.value);
    const min = Math.min(...values);
    const max = Math.max(...values);
    const range = max - min || 1;
    const innerWidth = CHART_WIDTH - PADDING_LEFT - PADDING_RIGHT;
    const innerHeight = CHART_HEIGHT - PADDING_TOP - PADDING_BOTTOM;
    return { min, max, range, innerWidth, innerHeight };
  }, [points]);

  const pathD = useMemo(() => {
    if (!metrics || points.length === 0) return "";
    return points
      .map((point, index) => {
        const x =
          PADDING_LEFT +
          (index / (points.length - 1 || 1)) * metrics.innerWidth;
        const y =
          PADDING_TOP +
          metrics.innerHeight -
          ((point.value - metrics.min) / metrics.range) *
            metrics.innerHeight;
        return `${index === 0 ? "M" : "L"}${x.toFixed(2)},${y.toFixed(2)}`;
      })
      .join(" ");
  }, [metrics, points]);

  const yTicks = useMemo(() => {
    if (!metrics) return [];
    const values = [];
    for (let i = 0; i < TICK_COUNT; i += 1) {
      const value =
        metrics.min + (metrics.range * i) / (TICK_COUNT - 1 || 1);
      const y =
        PADDING_TOP +
        metrics.innerHeight -
        ((value - metrics.min) / metrics.range) * metrics.innerHeight;
      values.push({ value, y });
    }
    return values;
  }, [metrics]);

  const xTicks = useMemo(() => {
    if (!metrics || points.length === 0) return [];
    const ticks = [];
    for (let i = 0; i < X_TICK_COUNT; i += 1) {
      const ratio = X_TICK_COUNT === 1 ? 0 : i / (X_TICK_COUNT - 1);
      const index = Math.round(ratio * (points.length - 1));
      const x =
        PADDING_LEFT + ratio * metrics.innerWidth;
      const date = points[index]?.date ?? "";
      ticks.push({ x, date });
    }
    return ticks;
  }, [metrics, points]);

  const hoverPoint =
    hoverIndex !== null && hoverIndex >= 0 && hoverIndex < points.length
      ? points[hoverIndex]
      : null;

  const handleMouseMove = (event: React.MouseEvent<SVGSVGElement>) => {
    if (!metrics || points.length === 0) return;
    const rect = svgRef.current?.getBoundingClientRect();
    if (!rect) return;
    const x = event.clientX - rect.left;
    const clamped = Math.min(
      CHART_WIDTH - PADDING_RIGHT,
      Math.max(PADDING_LEFT, x)
    );
    const ratio = (clamped - PADDING_LEFT) / metrics.innerWidth;
    const index = Math.round(ratio * (points.length - 1));
    setHoverIndex(index);

    const hoverX =
      PADDING_LEFT +
      (index / (points.length - 1 || 1)) * metrics.innerWidth;
    const value = points[index].value;
    const hoverY =
      PADDING_TOP +
      metrics.innerHeight -
      ((value - metrics.min) / metrics.range) * metrics.innerHeight;

    const tooltipWidth = 140;
    const tooltipHeight = 38;
    const left = Math.min(
      CHART_WIDTH - tooltipWidth,
      Math.max(0, hoverX + 8)
    );
    const top = Math.min(
      CHART_HEIGHT - tooltipHeight,
      Math.max(0, hoverY - tooltipHeight - 6)
    );
    setHoverPos({ x: left, y: top });
  };

  const handleMouseLeave = () => {
    setHoverIndex(null);
    setHoverPos(null);
  };

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
        <span>{title}</span>
      </div>
      <div className="fund-trend-window-abs">{trend.window}</div>
      {hoverPoint && hoverPos && (
        <div
          className="fund-trend-tooltip"
          style={{ left: `${hoverPos.x}px`, top: `${hoverPos.y}px` }}
        >
          <div className="fund-trend-tooltip-date">{hoverPoint.date}</div>
          <div className="fund-trend-tooltip-value">
            {valueFormatter(hoverPoint.value)}
          </div>
        </div>
      )}
      <svg
        width={CHART_WIDTH}
        height={CHART_HEIGHT}
        viewBox={`0 0 ${CHART_WIDTH} ${CHART_HEIGHT}`}
        aria-label="Fund trend chart"
        ref={svgRef}
        onMouseMove={handleMouseMove}
        onMouseLeave={handleMouseLeave}
      >
        {yTicks.map((tick, index) => (
          <g key={`${tick.value}-${index}`}>
            <line
              x1={PADDING_LEFT}
              x2={CHART_WIDTH - PADDING_RIGHT}
              y1={tick.y}
              y2={tick.y}
              className="fund-trend-grid"
            />
            <text
              x={PADDING_LEFT - 6}
              y={tick.y + 3}
              textAnchor="end"
              className="fund-trend-axis"
            >
              {valueFormatter(tick.value)}
            </text>
          </g>
        ))}
        {xTicks.map((tick, index) => (
          <text
            key={`${tick.x}-${index}`}
            x={tick.x}
            y={CHART_HEIGHT - 10}
            textAnchor={index === 0 ? "start" : index === xTicks.length - 1 ? "end" : "middle"}
            className="fund-trend-axis"
          >
            {formatDateLabel(tick.date)}
          </text>
        ))}
        <path
          d={pathD}
          fill="none"
          stroke={stroke}
          strokeWidth="2"
        />
        {hoverPoint && metrics && (
          <g className="fund-trend-hover">
            <line
              x1={
                PADDING_LEFT +
                (hoverIndex! / (points.length - 1 || 1)) * metrics.innerWidth
              }
              x2={
                PADDING_LEFT +
                (hoverIndex! / (points.length - 1 || 1)) * metrics.innerWidth
              }
              y1={PADDING_TOP}
              y2={PADDING_TOP + metrics.innerHeight}
              className="fund-trend-hover-line"
            />
            <circle
              cx={
                PADDING_LEFT +
                (hoverIndex! / (points.length - 1 || 1)) * metrics.innerWidth
              }
              cy={
                PADDING_TOP +
                metrics.innerHeight -
                ((hoverPoint.value - metrics.min) / metrics.range) *
                  metrics.innerHeight
              }
              r="3"
              className="fund-trend-hover-dot"
            />
          </g>
        )}
      </svg>
    </div>
  );
};

function formatDateLabel(value: string) {
  if (value.length >= 10) {
    return value.slice(5, 10);
  }
  return value;
}
