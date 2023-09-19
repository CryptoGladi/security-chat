import { Component } from 'solid-js';
import { Title, useNavigate } from 'solid-start';
import { open_that } from '~/api-tauri';

const Logo: Component = () => {
	return (
		<div class="flex flex-col items-center">
			<img src="index/lock.svg" class="h-64 w-64 py-4" />

			<h1 class="text-5xl font-bold">Security chat</h1>
		</div>
	);
};

const BadgeVersion: Component<{ version: string }> = (props) => {
	return (
		<div class="left-0 top-0 flex space-x-2 px-1 py-1" style="position: absolute;">
			<div class="badge badge-secondary">{props.version}</div>
		</div>
	);
};

export default function Home() {
	const navigate = useNavigate();

	return (
		<main>
			<Title>Добро пожаловать!</Title>

			<div class="hero min-h-screen text-center">
				<div class="max-w-md">
					<Logo />

					<p class="py-6">
						Универсальный чат который даёт вам <strong>приватность</strong>,{' '}
						<strong>анонимность</strong> c{' '}
						<a
							class="inline-flex cursor-pointer items-center font-medium text-accent-focus hover:underline"
							onclick={() => {
								open_that('https://github.com/CryptoGladi/security-chat');
							}}
						>
							открытым исходным кодом
						</a>
					</p>
					<button
						class="btn btn-primary"
						onClick={() => {
							navigate('/registration'); // TODO
						}}
					>
						Начать использование
					</button>
				</div>
			</div>

			<BadgeVersion version="0.1.0-alpha.2" />
		</main>
	);
}
