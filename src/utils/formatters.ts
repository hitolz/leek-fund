export const formatChangePercent = (value: string | null) => {
  if (!value) return "--";
  const numeric = Number(value);
  if (Number.isNaN(numeric)) return value;
  return `${numeric.toFixed(2)}%`;
};

export const formatTimestamp = (value: string | null) => {
  if (!value) return "";
  return value;
};

export const getChangeClass = (value: string | null) => {
  if (!value) return "neutral";
  const numeric = Number(value);
  if (Number.isNaN(numeric)) return "neutral";
  if (numeric > 0) return "positive";
  if (numeric < 0) return "negative";
  return "neutral";
};

export const getChangeClassFromNumber = (value: number | null | undefined) => {
  if (value === null || value === undefined || !Number.isFinite(value)) {
    return "neutral";
  }
  if (value > 0) return "positive";
  if (value < 0) return "negative";
  return "neutral";
};

export const formatCurrency = (value: number | null | undefined) => {
  if (value === null || value === undefined || !Number.isFinite(value)) {
    return "--";
  }
  return value.toFixed(2);
};

export const formatFixedNumber = (
  value: number | null | undefined,
  decimals: number
) => {
  if (value === null || value === undefined || !Number.isFinite(value)) {
    return "--";
  }
  return value.toFixed(decimals);
};

export const formatSignedCurrency = (value: number | null | undefined) => {
  if (value === null || value === undefined || !Number.isFinite(value)) {
    return "--";
  }
  const sign = value > 0 ? "+" : "";
  return `${sign}${value.toFixed(2)}`;
};

export const formatPercentNumber = (value: number | null | undefined) => {
  if (value === null || value === undefined || !Number.isFinite(value)) {
    return "--";
  }
  return `${value.toFixed(2)}%`;
};
