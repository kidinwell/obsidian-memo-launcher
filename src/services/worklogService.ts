import { invoke } from "@tauri-apps/api/core";
import { AppendWorklogResult, WorklogInput } from "../types/worklog";

export async function appendWorklog(input: WorklogInput): Promise<AppendWorklogResult> {
  return await invoke<AppendWorklogResult>("append_worklog", { input });
}
