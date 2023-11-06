import { createResource, createSignal, onMount } from 'solid-js';
import { SideBar } from '~/components/side_bar';
import { Link } from '~/components/small/link';
import { getMyNickname, getVersionApp } from '~/ts/api-tauri';
import randomItem from 'random-item';
import { useNavigate } from 'solid-start';

function getRandomText(): string {
	const texts = [
		'Помогите детям в Уганде!', // VIM
		'Поддержите наш проект!',
		'Интересно, это кто-то читает?',
		'Как же я стал писать этот проект...',
		'Анонимность - свобода или ошибка?',
		'Нужно взломать Пентагон'
	];

	return randomItem(texts);
}

export default function Index() {
	const [version, setVersion] = createSignal('1.0.0');
	const navigate = useNavigate();
	const [nickname] = createResource(async () => {
		return await getMyNickname();
	});

	onMount(async () => {
		setVersion(await getVersionApp());
	});

	return (
		<main class="flex">
			<SideBar />

			<div class="hero min-h-screen">
				<div class="hero-content text-center">
					<div class="max-w-md">
						<h1 class="text-5xl font-bold">Security chat</h1>

						<div class="py-6">
							<p>Пользователь: {nickname.loading ? 'LOADING...' : nickname()}</p>
							<p>Версия: {version()}</p>
							<p>
								Made by <Link link="https://github.com/CryptoGladi">CryptoGladi</Link>
							</p>

							<p class="font-mono text-secondary">{getRandomText()}</p>
						</div>

						<button
							class="btn btn-primary"
							onclick={() => {
								navigate('/main_window/add_friends');
							}}
						>
							Найти друзей
						</button>
					</div>
				</div>
			</div>
		</main>
	);
}
