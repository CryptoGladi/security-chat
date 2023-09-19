import { Component } from 'solid-js';
import { Title, useNavigate } from 'solid-start';
import { hexToCSSFilter } from 'hex-to-css-filter';
import debug from 'debug';

const Logo: Component = () => {
	const cssFilter = hexToCSSFilter('#00a4d6');
	debug('eee' + cssFilter);
	console.warn(cssFilter);
	
	return (
		<div>
			<img src="index/lock.svg" class="filter: invert(47%) sepia(92%) saturate(1908%) hue-rotate(160deg) brightness(92%) contrast(104%);"/>

			<h1 class="text-5xl font-bold">Security chat</h1>
		</div>
	);
};

export default function Home() {
	const navigate = useNavigate();

	return (
		<main>
			<Title>Добро пожаловать!</Title>

			<div class="text-center hero min-h-screen">
				<div class="max-w-md">
						<Logo/>
						
						<p class="py-6">
							Универсальный чат который даёт вам <strong>приватность</strong>,{' '}
							<strong>анонимность</strong> c{' '}
							<a class="inline-flex items-center font-medium text-accent-focus cursor-pointer hover:underline">
								открытым исходным кодом
							</a>
						</p>
						<button
							class="btn btn-primary"
							onClick={() => {
								navigate('/registration'); // TODO
							}}
						>
							Начать использование!
						</button>
				</div>
			</div>
		</main>
	);
}
