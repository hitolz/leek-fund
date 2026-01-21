import React, { useState, useEffect } from "react";
import "../styles.css";
import { ToastContext } from "./ToastContext";

interface ToastProps {
  message: string;
  type: "success" | "error";
}

export const Layout: React.FC<{
  children: React.ReactNode;
  headerRight?: React.ReactNode;
}> = ({ children, headerRight }) => {
  const [toast, setToast] = useState<ToastProps | null>(null);

  // 自动关闭 Toast
  useEffect(() => {
    if (toast) {
      const timer = setTimeout(() => setToast(null), 3000);
      return () => clearTimeout(timer);
    }
  }, [toast]);

  // 将 showToast 函数提供给子组件
  const showToast = (message: string, type: "success" | "error" = "success") => {
    setToast({ message, type });
  };

  return (
    <div className="app-layout">
      <header className="app-header">
        <div className="app-header-content">
          <h1>📈 基金查询客户端</h1>
          {headerRight && <div className="app-header-right">{headerRight}</div>}
        </div>
      </header>

      <ToastContext.Provider value={showToast}>
        <main className="app-main app-main-three-column">{children}</main>
      </ToastContext.Provider>

      {toast && (
        <div className={`toast toast-${toast.type}`}>
          {toast.message}
        </div>
      )}
    </div>
  );
};
