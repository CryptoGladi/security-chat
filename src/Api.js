import { invoke } from "@tauri-apps/api/tauri";

export function open_that(path) {
  invoke("open", { path: path });
}

export function registration(nickname) {
  invoke("registration", { nickname: nickname });
}

export async function fuzzy_search_command(command) {
  return await invoke("fuzzy_search_vim_command", { command: command });
}

export async function run_command(command) {
  await invoke("run_command", { command: command });
}