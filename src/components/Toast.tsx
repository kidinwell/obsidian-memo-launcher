import React from "react";

export interface ToastProps {
  message: string;
  type?: "success" | "error" | "info";
  visible: boolean;
}

export const Toast: React.FC<ToastProps> = ({ message, type = "success", visible }) => {
  if (!visible) return null;

  const bgColors = {
    success: "bg-emerald-600 text-white shadow-emerald-900/40",
    error: "bg-rose-600 text-white shadow-rose-900/40",
    info: "bg-indigo-600 text-white shadow-indigo-900/40",
  };

  return (
    <div className="absolute top-3 left-1/2 -translate-x-1/2 z-50 pointer-events-none transition-all duration-200">
      <div
        className={`px-4 py-2 rounded-lg text-xs font-medium shadow-lg flex items-center gap-2 ${bgColors[type]}`}
      >
        {type === "success" && (
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M5 13l4 4L19 7" />
          </svg>
        )}
        {type === "error" && (
          <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M6 18L18 6M6 6l12 12" />
          </svg>
        )}
        <span>{message}</span>
      </div>
    </div>
  );
};
