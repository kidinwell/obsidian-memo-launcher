import React from "react";
import { CheckCircle2, AlertCircle, Info } from "lucide-react";

export interface ToastProps {
  message: string;
  type?: "success" | "error" | "info";
  visible: boolean;
}

export const Toast: React.FC<ToastProps> = ({ message, type = "success", visible }) => {
  if (!visible) return null;

  const icons = {
    success: <CheckCircle2 className="size-4 text-emerald-500" />,
    error: <AlertCircle className="size-4 text-destructive" />,
    info: <Info className="size-4 text-primary" />,
  };

  return (
    <div
      data-testid="toast-notification"
      className="absolute top-3 left-1/2 -translate-x-1/2 z-50 pointer-events-none transition-all duration-200 animate-in fade-in slide-in-from-top-2"
    >
      <div className="px-4 py-2 rounded-lg text-xs font-medium shadow-lg flex items-center gap-2 bg-popover text-popover-foreground border">
        {icons[type]}
        <span>{message}</span>
      </div>
    </div>
  );
};
