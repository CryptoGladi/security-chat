import { SideBar } from '~/components/side_bar';
import { autofocus } from '@solid-primitives/autofocus';

autofocus; // prevents from being tree-shaken by TS

export default function Index() {
	return (
		<main class="flex">
			<SideBar />

			<div class="flex rounded-lg border border-neutral bg-neutral p-1 shadow-2xl">
				<input
					class="input focus:outline-none"
					placeholder="Выполнить комманду..."
					autofocus
				></input>
			</div>
		</main>
	);
}
