import { Component } from 'solid-js';
import { Title, useNavigate } from 'solid-start';
import { Link } from '~/components/small/link';
import { BadgeVersion } from '~/components/small/badge_version';

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
							link="https://github.com/CryptoGladi/security-chat"
						>открытым исходным кодом</Link>
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
