declare global { interface Window { __TAURI__?: any; } }
import { invoke as tauriInvoke } from "@tauri-apps/api/core";

// Safe invoke wrapper for browser testing
export const invoke = async <T,>(cmd: string, args?: any): Promise<T> => {
  if (window.__TAURI__) {
    return tauriInvoke<T>(cmd, args);
  }
  
  console.warn(`[Tauri Mock] IPC call blocked in browser: ${cmd}`, args);
  
  // Return dummy data for UI testing
  if (cmd === "get_active_transactions") return [] as any;
  if (cmd === "search_books") return [] as any;
  if (cmd === "get_members") return [] as any;
  
  return null as any;
};
