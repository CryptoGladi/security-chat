import { invoke } from "@tauri-apps/api/tauri";

export function open_that(path) {
  invoke("open", { path: path });
}

export function nickname_is_taken(nickname) {
  invoke("nickname_is_taken", { nickname: nickname });
}