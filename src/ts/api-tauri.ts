import { invoke } from '@tauri-apps/api';

export function open_that(path: string) {
	invoke('open', { path: path });
}

export async function get_random_nickname(): Promise<string> {
	return invoke('get_random_nickname');
}
