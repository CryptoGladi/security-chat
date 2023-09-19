import { Title, useNavigate } from 'solid-start';

export default function Home() {
	const navigate = useNavigate();

	return (
		<main>
			<Title>Добро пожаловать!</Title>

			<div class="hero min-h-screen">
				<div class="hero-content text-center">
					<div class="max-w-md">
						<h1 class="text-5xl font-bold">Security chat</h1>
						<p class="py-6">
							Универсальный чат который даёт вам <strong>приватность</strong>,{' '}
							<strong>анонимность</strong> c открытым исходным кодом
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
			</div>
		</main>
	);
}
