import { createSignal, onMount } from 'solid-js';
import { SideBar } from '~/components/side_bar';
import { Link } from '~/components/small/link';
import { getVersionApp } from '~/ts/api-tauri';
import randomItem from 'random-item';
import { useNavigate } from 'solid-start';

function getRandomText():string {
	const texts = ['Помогите детям в Уганде!', 'Поддержите наш проект!'];

	return randomItem(texts);
}

export default function Index() {
	const [version, setVersion] = createSignal('1.0.0');
	const navigate = useNavigate();

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
							<p>Версия: {version()}</p>
							<p>Made by <Link link="https://github.com/CryptoGladi">CryptoGladi</Link></p>

							<p>{getRandomText()}</p>
						</div>

						<button class="btn btn-primary" onclick={() => {
							navigate('/main_window/add_friends');
						}}>Найти друзей</button>
					</div>
				</div>
			</div>
		</main>
	);
}
