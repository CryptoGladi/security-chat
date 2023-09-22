import { invoke } from '@tauri-apps/api';

export function open_that(path: string) {
	invoke('open', { path: path });
}
