import React from "react";
import { Tabs, TabsList, TabsTrigger } from "./ui/tabs";
import { Kbd } from "./ui/kbd";
import { Button } from "./ui/button";
import { Settings, X } from "lucide-react";

interface HeaderProps {
  activeTab: "worklog" | "task";
  onTabChange: (tab: "worklog" | "task") => void;
  onOpenSettings: () => void;
  onClose: () => void;
}

export const Header: React.FC<HeaderProps> = ({
  activeTab,
  onTabChange,
  onOpenSettings,
  onClose,
}) => {
  return (
    <div className="flex items-center justify-between pb-3 mb-3 border-b select-none">
      {/* Tabs */}
      <Tabs
        value={activeTab}
        onValueChange={(val) => onTabChange(val as "worklog" | "task")}
      >
        <TabsList>
          <TabsTrigger
            value="worklog"
            data-testid="tab-worklog-btn"
            onMouseDown={(e) => e.preventDefault()}
            onClick={() => onTabChange("worklog")}
          >
            <Kbd>1</Kbd>
            <span>作業実績</span>
          </TabsTrigger>
          <TabsTrigger
            value="task"
            data-testid="tab-task-btn"
            onMouseDown={(e) => e.preventDefault()}
            onClick={() => onTabChange("task")}
          >
            <Kbd>2</Kbd>
            <span>タスク追加</span>
          </TabsTrigger>
        </TabsList>
      </Tabs>

      {/* Action icons */}
      <div className="flex items-center gap-1">
        <Button
          variant="ghost"
          size="icon-sm"
          data-testid="open-settings-btn"
          onClick={onOpenSettings}
          title="設定 (Settings)"
        >
          <Settings />
        </Button>
        <Button
          variant="ghost"
          size="icon-sm"
          data-testid="close-launcher-btn"
          onClick={onClose}
          title="閉じる (Esc)"
        >
          <X />
        </Button>
      </div>
    </div>
  );
};
