import React from "react";
import { Bitcoin, Coins, Landmark, LineChart } from "lucide-react";
import { AssetCategory } from "../../types/ai-copilot";

interface AssetCategoryIconProps {
  category: AssetCategory;
  size?: number;
}

export const AssetCategoryIcon: React.FC<AssetCategoryIconProps> = ({
  category,
  size = 16,
}) => {
  if (category === "fund") return <Landmark size={size} aria-hidden="true" />;
  if (category === "stock") return <LineChart size={size} aria-hidden="true" />;
  if (category === "crypto") return <Bitcoin size={size} aria-hidden="true" />;
  return <Coins size={size} aria-hidden="true" />;
};
