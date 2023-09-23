import { invoke } from '@tauri-apps/api';

export function openThat(path: string) {
	invoke('open', { path: path });
}

export function getRandomNickname(): Promise<string> {
	return invoke('get_random_nickname');
}

export function nicknameIsTaken(nickname: string): Promise<string> {
	return invoke('nickname_is_taken', { nickname: nickname });
}
