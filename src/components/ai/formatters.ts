export function formatMoney(value: number, compact = false): string {
  const absolute = Math.abs(value);
  const sign = value < 0 ? "-" : "";
  if (compact && absolute >= 10000) {
    return `${sign}¥${(absolute / 10000).toFixed(2)}万`;
  }
  return `${sign}¥${absolute.toLocaleString("zh-CN", {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  })}`;
}

export function formatSignedMoney(value: number, compact = false): string {
  return `${value > 0 ? "+" : ""}${formatMoney(value, compact)}`;
}

export function formatPercent(value: number): string {
  return `${value > 0 ? "+" : ""}${value.toFixed(2)}%`;
}

export function formatSnapshotTime(timestamp: number): string {
  return new Date(timestamp * 1000).toLocaleString("zh-CN", {
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
}
