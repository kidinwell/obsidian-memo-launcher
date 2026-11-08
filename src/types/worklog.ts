export interface WorklogInput {
  memo: string;
  timestamp?: string | null;
}

export interface AppendWorklogResult {
  success: boolean;
  daily_note_path: string;
  entry_text: string;
}
