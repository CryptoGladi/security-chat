import { invoke } from "@tauri-apps/api/tauri";

export function open_that(path) {
  invoke("open", { path: path });
}