import { invoke } from "@tauri-apps/api/core";
import { CreateTaskResult, TaskInput } from "../types/task";

export async function createTask(input: TaskInput): Promise<CreateTaskResult> {
  return await invoke<CreateTaskResult>("create_task", { input });
}
