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
