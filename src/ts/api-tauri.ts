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

export function getVersionApp(): Promise<string> {
	return invoke('get_version_app');
}

export function registration(nickname: string): Promise<void> {
	return invoke('registration', { nickname: nickname });
}

export function haveAccount(): Promise<boolean> {
	return invoke('have_account');
}

export function getCryptosForAccept(): Promise<string[]> {
	return invoke('get_cryptos_for_accept');
}

export function sendCrypto(nickname: string): Promise<void> {
	return invoke('send_crypto', { nickname: nickname });
}

export function getMyNickname(): Promise<string> {
	return invoke('get_nickname');
}

export function getOrderAddingCrypto(): Promise<string[]> {
	return invoke('get_order_adding_crypto');
}
