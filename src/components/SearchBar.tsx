import React, { useState, useEffect } from "react";
import { useTauriCommands } from "../hooks/useTauriCommands";
import { FundInfo } from "../types";

interface SearchBarProps {
  onFundFound: (fund: FundInfo) => void;
  showToast?: (message: string, type: "success" | "error") => void;
}

export const SearchBar: React.FC<SearchBarProps> = ({
  onFundFound,
  showToast,
}) => {
  const [code, setCode] = useState("");
  const [loading, setLoading] = useState(false);
  const { searchFund } = useTauriCommands();

  // 防抖搜索
  useEffect(() => {
    if (code.length !== 6) return;

    const timer = setTimeout(async () => {
      setLoading(true);
      try {
        const fund = await searchFund(code);
        onFundFound(fund);
      } catch (error) {
        if (showToast) {
          showToast(String(error), "error");
        }
      } finally {
        setLoading(false);
      }
    }, 300);

    return () => clearTimeout(timer);
  }, [code]);

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const value = e.target.value.replace(/\D/g, "").slice(0, 6);
    setCode(value);
  };

  return (
    <div className="search-bar">
      <input
        type="text"
        value={code}
        onChange={handleInputChange}
        placeholder="输入6位基金代码"
        maxLength={6}
        className="search-input"
        disabled={loading}
      />
      {loading && <span className="loading-spinner">🔄</span>}
    </div>
  );
};

