import React from "react";

type ToastType = "success" | "error";
type ShowToast = (message: string, type?: ToastType) => void;

const noop: ShowToast = () => {};

export const ToastContext = React.createContext<ShowToast>(noop);

export const useToast = () => React.useContext(ToastContext);
