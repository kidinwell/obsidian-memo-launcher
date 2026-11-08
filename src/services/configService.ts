import { invoke } from "@tauri-apps/api/core";
import { AppConfig, VaultValidationResult } from "../types/config";

export async function getConfig(): Promise<AppConfig> {
  return await invoke<AppConfig>("get_config");
}

export async function setConfig(config: AppConfig): Promise<void> {
  await invoke("set_config", { config });
}

export async function validateVault(vaultPath: string): Promise<VaultValidationResult> {
  return await invoke<VaultValidationResult>("validate_vault", { vaultPath });
}
