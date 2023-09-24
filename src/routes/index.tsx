import { Component } from 'solid-js';
import { Title, useNavigate } from 'solid-start';
import { Link } from '~/components/link';
import { BadgeVersion } from '~/components/badge_version';
import { have_account } from '~/ts/api-tauri';

const Logo: Component = () => {
	return (
		<div class="flex flex-col items-center">
			<img src="index/lock.svg" class="h-64 w-64 py-4" />

			<h1 class="text-5xl font-bold">Security chat</h1>
		</div>
	);
};

export default function Index() {
	const navigate = useNavigate();

	have_account().then((have_account) => {
		if (have_account) {
			navigate('/main');
		}
	})

	return (
		<main>
			<Title>Добро пожаловать!</Title>

			<div class="hero min-h-screen text-center">
				<div class="max-w-md">
					<Logo />

					<p class="py-6">
						Универсальный чат который даёт вам <strong>приватность</strong>,{' '}
						<strong>анонимность</strong> c{' '}
						<Link
							text="открытым исходным кодом"
							link="https://github.com/CryptoGladi/security-chat"
						/>
					</p>
					<button
						class="btn btn-primary"
						onClick={() => {
							navigate('/registration');
						}}
					>
						Начать использование
					</button>
				</div>
			</div>

			<BadgeVersion />
		</main>
	);
}
